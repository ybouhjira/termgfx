use crossterm::{
    cursor::{Hide, MoveTo, Show},
    event::{self, Event, KeyCode},
    execute,
    style::{Color, Print, ResetColor, SetForegroundColor},
    terminal::{self, Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::io::{self, BufRead, Read, Write};

pub struct Pager {
    lines: Vec<String>,
    line_numbers: bool,
    title: Option<String>,
}

impl Pager {
    pub fn new(content: String, line_numbers: bool, title: Option<String>) -> Self {
        let lines: Vec<String> = content.lines().map(|l| l.to_string()).collect();
        Self {
            lines,
            line_numbers,
            title,
        }
    }

    pub fn render(&self) -> io::Result<()> {
        if self.lines.is_empty() {
            return Ok(());
        }

        let mut stdout = io::stdout();
        let mut scroll_offset = 0usize;

        terminal::enable_raw_mode()?;
        execute!(stdout, EnterAlternateScreen, Hide)?;

        loop {
            let (_, rows) = terminal::size()?;
            let available_rows = rows as usize - 2; // Reserve for header/footer

            self.render_ui(&mut stdout, scroll_offset, available_rows)?;

            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Up | KeyCode::Char('k') => {
                        if scroll_offset > 0 {
                            scroll_offset -= 1;
                        }
                    }
                    KeyCode::Down | KeyCode::Char('j') => {
                        if scroll_offset + available_rows < self.lines.len() {
                            scroll_offset += 1;
                        }
                    }
                    KeyCode::PageUp | KeyCode::Char('b') => {
                        scroll_offset = scroll_offset.saturating_sub(available_rows);
                    }
                    KeyCode::PageDown | KeyCode::Char('f') | KeyCode::Char(' ') => {
                        let max_offset = self.lines.len().saturating_sub(available_rows);
                        scroll_offset = (scroll_offset + available_rows).min(max_offset);
                    }
                    KeyCode::Home | KeyCode::Char('g') => {
                        scroll_offset = 0;
                    }
                    KeyCode::End | KeyCode::Char('G') => {
                        scroll_offset = self.lines.len().saturating_sub(available_rows);
                    }
                    KeyCode::Char('q') | KeyCode::Esc => {
                        break;
                    }
                    _ => {}
                }
            }
        }

        execute!(stdout, Show, LeaveAlternateScreen)?;
        terminal::disable_raw_mode()?;

        Ok(())
    }

    fn render_ui(
        &self,
        stdout: &mut io::Stdout,
        scroll_offset: usize,
        available_rows: usize,
    ) -> io::Result<()> {
        let (cols, _) = terminal::size()?;

        execute!(stdout, Clear(ClearType::All), MoveTo(0, 0))?;

        // Header
        let title = self.title.as_deref().unwrap_or("termgfx pager");
        execute!(
            stdout,
            SetForegroundColor(Color::Cyan),
            Print(format!("─── {} ", title)),
        )?;
        let remaining = cols as usize - title.len() - 5;
        execute!(
            stdout,
            Print("─".repeat(remaining.min(cols as usize))),
            ResetColor,
            Print("\n")
        )?;

        // Content
        let line_num_width = if self.line_numbers {
            self.lines.len().to_string().len() + 2
        } else {
            0
        };
        let content_width = cols as usize - line_num_width;

        for (i, line) in self
            .lines
            .iter()
            .skip(scroll_offset)
            .take(available_rows)
            .enumerate()
        {
            let line_num = scroll_offset + i + 1;

            if self.line_numbers {
                execute!(
                    stdout,
                    SetForegroundColor(Color::DarkGrey),
                    Print(format!("{:>width$} ", line_num, width = line_num_width - 1)),
                    ResetColor
                )?;
            }

            // Truncate line if too long
            let display_line = if line.len() > content_width {
                format!("{}…", &line[..content_width.saturating_sub(1)])
            } else {
                line.clone()
            };

            execute!(stdout, Print(display_line), Print("\n"))?;
        }

        // Fill remaining space
        let displayed = self.lines.len().min(available_rows);
        for _ in displayed..available_rows {
            execute!(stdout, Print("~\n"))?;
        }

        // Footer
        let percent = if self.lines.len() <= available_rows {
            100
        } else {
            ((scroll_offset as f64 / (self.lines.len() - available_rows) as f64) * 100.0) as usize
        };

        execute!(
            stdout,
            SetForegroundColor(Color::DarkGrey),
            Print(format!(
                "Lines {}-{} of {} ({}%) | ↑/↓:scroll | PgUp/PgDn:page | g/G:top/bottom | q:quit",
                scroll_offset + 1,
                (scroll_offset + available_rows).min(self.lines.len()),
                self.lines.len(),
                percent
            )),
            ResetColor
        )?;

        stdout.flush()
    }
}

pub fn render(line_numbers: bool, title: Option<String>) {
    // Check if stdin is a TTY (no piped input)
    if atty::is(atty::Stream::Stdin) {
        eprintln!("Error: No input provided. Pipe content to pager:");
        eprintln!("  cat file.txt | termgfx pager");
        eprintln!("  ls -la | termgfx pager --line-numbers");
        std::process::exit(1);
    }

    // Read from stdin
    let mut content = String::new();
    if let Err(e) = io::stdin().read_to_string(&mut content) {
        eprintln!("Error reading stdin: {}", e);
        std::process::exit(1);
    }

    if content.trim().is_empty() {
        eprintln!("Error: No content to display");
        std::process::exit(1);
    }

    let pager = Pager::new(content, line_numbers, title);
    if let Err(e) = pager.render() {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}
