use crossterm::{
    cursor::{Hide, MoveTo, Show},
    event::{self, Event, KeyCode, KeyEvent},
    execute,
    style::{Color, Print, ResetColor, SetForegroundColor, Stylize},
    terminal::{self, Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::io::{self, Write};

pub fn render(prompt: &str, options: &[String]) {
    if options.is_empty() {
        eprintln!("Error: No options provided");
        std::process::exit(1);
    }

    match run_select(prompt, options) {
        Ok(selected) => {
            println!("{}", selected);
        }
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    }
}

fn run_select(prompt: &str, options: &[String]) -> io::Result<String> {
    let mut stdout = io::stdout();
    let mut selected_idx = 0;

    // Setup terminal
    terminal::enable_raw_mode()?;
    execute!(stdout, EnterAlternateScreen, Hide)?;

    let result = loop {
        // Render the prompt and options
        render_menu(&mut stdout, prompt, options, selected_idx)?;

        // Handle key events
        if let Event::Key(KeyEvent { code, .. }) = event::read()? {
            match code {
                KeyCode::Up | KeyCode::Char('k') => {
                    if selected_idx > 0 {
                        selected_idx -= 1;
                    }
                }
                KeyCode::Down | KeyCode::Char('j') => {
                    if selected_idx < options.len() - 1 {
                        selected_idx += 1;
                    }
                }
                KeyCode::Enter => {
                    break Ok(options[selected_idx].clone());
                }
                KeyCode::Esc | KeyCode::Char('q') => {
                    break Err(io::Error::new(io::ErrorKind::Interrupted, "Cancelled"));
                }
                _ => {}
            }
        }
    };

    // Cleanup terminal
    execute!(stdout, Show, LeaveAlternateScreen)?;
    terminal::disable_raw_mode()?;

    result
}

fn render_menu(
    stdout: &mut io::Stdout,
    prompt: &str,
    options: &[String],
    selected_idx: usize,
) -> io::Result<()> {
    execute!(stdout, Clear(ClearType::All), MoveTo(0, 0))?;

    // Print prompt with emoji
    execute!(
        stdout,
        SetForegroundColor(Color::Cyan),
        Print("❯ "),
        ResetColor,
        Print(prompt.to_string().bold()),
        Print("\n\n")
    )?;

    // Print options
    for (idx, option) in options.iter().enumerate() {
        if idx == selected_idx {
            // Highlighted selection
            let bold_option = option.clone().bold().to_string();
            execute!(
                stdout,
                SetForegroundColor(Color::Green),
                Print("❯ "),
                Print(bold_option),
                ResetColor,
                Print("\n")
            )?;
        } else {
            // Normal option
            execute!(
                stdout,
                Print("  "),
                Print(option),
                Print("\n")
            )?;
        }
    }

    // Print help text
    execute!(
        stdout,
        Print("\n"),
        SetForegroundColor(Color::DarkGrey),
        Print("↑↓: Navigate • Enter: Select • Esc: Cancel"),
        ResetColor
    )?;

    stdout.flush()?;
    Ok(())
}
