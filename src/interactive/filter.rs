use crossterm::{
    cursor::{Hide, MoveTo, Show},
    event::{self, Event, KeyCode, KeyEvent},
    execute,
    style::{Color, Print, ResetColor, SetForegroundColor, Stylize},
    terminal::{self, Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::collections::HashSet;
use std::io::{self, BufRead, IsTerminal, Write};

pub struct FuzzyFilter {
    items: Vec<String>,
    prompt: String,
    multi: bool,
    height: Option<usize>,
}

impl FuzzyFilter {
    pub fn new(items: Vec<String>, prompt: Option<String>, multi: bool, height: Option<usize>) -> Self {
        Self {
            items,
            prompt: prompt.unwrap_or_else(|| "Filter:".to_string()),
            multi,
            height,
        }
    }

    pub fn render(&self) -> io::Result<Vec<String>> {
        // Check for interactive terminal
        if !std::io::stdin().is_terminal() {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                "Filter requires an interactive terminal (TTY)",
            ));
        }

        if self.items.is_empty() {
            return Ok(vec![]);
        }

        let mut stdout = io::stdout();
        let mut query = String::new();
        let mut selected_idx = 0;
        let mut selected_items: HashSet<usize> = HashSet::new();

        terminal::enable_raw_mode()?;
        execute!(stdout, EnterAlternateScreen, Hide)?;

        let result = loop {
            let matches = self.filter_items(&query);
            self.render_ui(&mut stdout, &query, &matches, selected_idx, &selected_items)?;

            if let Event::Key(KeyEvent { code, .. }) = event::read()? {
                match code {
                    KeyCode::Up | KeyCode::Char('k') if !matches.is_empty() => {
                        if selected_idx > 0 {
                            selected_idx -= 1;
                        }
                    }
                    KeyCode::Down | KeyCode::Char('j') if !matches.is_empty() => {
                        if selected_idx < matches.len() - 1 {
                            selected_idx += 1;
                        }
                    }
                    KeyCode::Char(' ') if self.multi && !matches.is_empty() => {
                        let original_idx = matches[selected_idx].0;
                        if selected_items.contains(&original_idx) {
                            selected_items.remove(&original_idx);
                        } else {
                            selected_items.insert(original_idx);
                        }
                    }
                    KeyCode::Enter if !matches.is_empty() => {
                        if self.multi {
                            let result: Vec<String> = selected_items
                                .iter()
                                .map(|&idx| self.items[idx].clone())
                                .collect();
                            break Ok(result);
                        } else {
                            break Ok(vec![matches[selected_idx].1.clone()]);
                        }
                    }
                    KeyCode::Char(c) => {
                        query.push(c);
                        selected_idx = 0;
                    }
                    KeyCode::Backspace => {
                        query.pop();
                        selected_idx = 0;
                    }
                    KeyCode::Esc => {
                        break Ok(vec![]);
                    }
                    _ => {}
                }
            }
        };

        execute!(stdout, Show, LeaveAlternateScreen)?;
        terminal::disable_raw_mode()?;

        result
    }

    fn filter_items(&self, query: &str) -> Vec<(usize, String)> {
        if query.is_empty() {
            return self.items.iter().cloned().enumerate().collect();
        }

        let query_lower = query.to_lowercase();
        self.items
            .iter()
            .enumerate()
            .filter(|(_, item)| item.to_lowercase().contains(&query_lower))
            .map(|(i, item)| (i, item.clone()))
            .collect()
    }

    fn render_ui(
        &self,
        stdout: &mut io::Stdout,
        query: &str,
        matches: &[(usize, String)],
        selected_idx: usize,
        selected_items: &HashSet<usize>,
    ) -> io::Result<()> {
        execute!(stdout, Clear(ClearType::All), MoveTo(0, 0))?;

        // Prompt and query
        execute!(
            stdout,
            SetForegroundColor(Color::Cyan),
            Print(&self.prompt),
            Print(" "),
            ResetColor,
            Print(query),
            Print("█\n\n")
        )?;

        // Calculate visible items
        let max_height = self.height.unwrap_or(10).min(matches.len());

        // Render matches
        for (i, (original_idx, item)) in matches.iter().take(max_height).enumerate() {
            let is_current = i == selected_idx;
            let is_selected = selected_items.contains(original_idx);

            let prefix = if self.multi {
                if is_selected { "[x]" } else { "[ ]" }
            } else {
                " "
            };

            let indicator = if is_current { "❯" } else { " " };

            execute!(
                stdout,
                SetForegroundColor(if is_current { Color::Green } else { Color::Reset }),
                Print(format!("{} {} {}\n", indicator, prefix, item)),
                ResetColor
            )?;
        }

        // Show count
        execute!(
            stdout,
            Print("\n"),
            SetForegroundColor(Color::DarkGrey),
            Print(format!("{}/{} items", matches.len(), self.items.len())),
            ResetColor
        )?;

        stdout.flush()
    }
}

pub fn render(prompt: Option<String>, multi: bool, height: Option<usize>) {
    // Read from stdin
    let stdin = io::stdin();
    let items: Vec<String> = stdin.lock().lines().filter_map(|l| l.ok()).collect();

    if items.is_empty() {
        eprintln!("Error: No input provided");
        std::process::exit(1);
    }

    let filter = FuzzyFilter::new(items, prompt, multi, height);

    match filter.render() {
        Ok(selected) => {
            for item in selected {
                println!("{}", item);
            }
        }
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    }
}
