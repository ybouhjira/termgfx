use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyEvent},
    execute,
    style::{Color, Print, ResetColor, SetForegroundColor},
    terminal,
};
use std::io::{self, IsTerminal, Write};

/// Render a yes/no confirmation prompt
pub fn render(prompt: &str, default: &str, style: &str) {
    let default_bool = match default.to_lowercase().as_str() {
        "yes" | "y" | "true" => true,
        "no" | "n" | "false" => false,
        _ => true,
    };

    let result = show_confirm_prompt(prompt, default_bool, style);

    match result {
        Ok(answer) => {
            if answer {
                println!("true");
                std::process::exit(0);
            } else {
                println!("false");
                std::process::exit(1);
            }
        }
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(2);
        }
    }
}

fn show_confirm_prompt(prompt: &str, default: bool, style: &str) -> io::Result<bool> {
    // Check for interactive terminal
    if !std::io::stdin().is_terminal() {
        return Err(io::Error::new(
            io::ErrorKind::Other,
            "Confirm requires an interactive terminal (TTY)",
        ));
    }

    terminal::enable_raw_mode()?;

    let mut stdout = io::stdout();

    let (prompt_color, bracket_color) = match style.to_lowercase().as_str() {
        "danger" => (Color::Red, Color::DarkRed),
        _ => (Color::Cyan, Color::DarkCyan),
    };

    let emoji = if style.to_lowercase() == "danger" {
        "⚠️  "
    } else {
        ""
    };

    let options = if default { "[Y/n]" } else { "[y/N]" };

    execute!(
        stdout,
        SetForegroundColor(prompt_color),
        Print(format!("{}{} ", emoji, prompt)),
        SetForegroundColor(bracket_color),
        Print(format!("{}: ", options)),
        ResetColor,
        cursor::Show,
    )?;
    stdout.flush()?;

    let answer = loop {
        if event::poll(std::time::Duration::from_millis(100))? {
            if let Event::Key(KeyEvent { code, .. }) = event::read()? {
                match code {
                    KeyCode::Char('y') | KeyCode::Char('Y') => break Some(true),
                    KeyCode::Char('n') | KeyCode::Char('N') => break Some(false),
                    KeyCode::Enter => break Some(default),
                    KeyCode::Esc | KeyCode::Char('c') => {
                        execute!(stdout, Print("\n"))?;
                        terminal::disable_raw_mode()?;
                        std::process::exit(130);
                    }
                    _ => {}
                }
            }
        }
    };

    let answer_bool = answer.unwrap_or(default);
    let answer_text = if answer_bool { "y" } else { "n" };
    execute!(
        stdout,
        SetForegroundColor(if answer_bool {
            Color::Green
        } else {
            Color::Red
        }),
        Print(answer_text),
        ResetColor,
        Print("\n"),
    )?;
    stdout.flush()?;

    terminal::disable_raw_mode()?;

    Ok(answer_bool)
}
