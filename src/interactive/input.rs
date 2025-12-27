use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyEvent, KeyModifiers},
    execute,
    style::{Color, Print, ResetColor, SetForegroundColor},
    terminal::{self, ClearType},
};
use std::io::{self, IsTerminal, Write};

pub fn render(prompt: &str, placeholder: Option<&str>, password: bool) {
    match run_input(prompt, placeholder, password) {
        Ok(input) => {
            println!("{}", input);
        }
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    }
}

fn run_input(prompt: &str, placeholder: Option<&str>, password: bool) -> io::Result<String> {
    // Check for interactive terminal
    if !std::io::stdin().is_terminal() {
        return Err(io::Error::new(
            io::ErrorKind::Other,
            "Input requires an interactive terminal (TTY)",
        ));
    }

    let mut stdout = io::stdout();
    let mut input = String::new();

    // Enable raw mode for character-by-character input
    terminal::enable_raw_mode()?;

    // Display the prompt
    execute!(
        stdout,
        SetForegroundColor(Color::Cyan),
        Print(prompt),
        Print(" "),
        ResetColor
    )?;

    // Display placeholder if provided and input is empty
    if let Some(placeholder_text) = placeholder {
        execute!(
            stdout,
            SetForegroundColor(Color::DarkGrey),
            Print(placeholder_text),
            ResetColor
        )?;

        // Move cursor back to start of placeholder
        let placeholder_width = placeholder_text.chars().count();
        for _ in 0..placeholder_width {
            execute!(stdout, cursor::MoveLeft(1))?;
        }
    }

    stdout.flush()?;

    let result = loop {
        // Read keyboard events
        if let Event::Key(key_event) = event::read()? {
            match key_event {
                // Enter key - submit input
                KeyEvent {
                    code: KeyCode::Enter,
                    ..
                } => {
                    break Ok(input.clone());
                }

                // Ctrl+C - cancel
                KeyEvent {
                    code: KeyCode::Char('c'),
                    modifiers: KeyModifiers::CONTROL,
                    ..
                } => {
                    break Err(io::Error::new(
                        io::ErrorKind::Interrupted,
                        "Cancelled by user",
                    ));
                }

                // Backspace - delete character
                KeyEvent {
                    code: KeyCode::Backspace,
                    ..
                } => {
                    if !input.is_empty() {
                        input.pop();

                        // Move cursor back, clear character, move back again
                        execute!(
                            stdout,
                            cursor::MoveLeft(1),
                            terminal::Clear(ClearType::UntilNewLine)
                        )?;

                        // Redisplay placeholder if input is now empty
                        if input.is_empty() {
                            if let Some(placeholder_text) = placeholder {
                                execute!(
                                    stdout,
                                    SetForegroundColor(Color::DarkGrey),
                                    Print(placeholder_text),
                                    ResetColor
                                )?;

                                // Move cursor back to start
                                let placeholder_width = placeholder_text.chars().count();
                                for _ in 0..placeholder_width {
                                    execute!(stdout, cursor::MoveLeft(1))?;
                                }
                            }
                        }
                    }
                }

                // Regular character input
                KeyEvent {
                    code: KeyCode::Char(c),
                    modifiers: KeyModifiers::NONE | KeyModifiers::SHIFT,
                    ..
                } => {
                    // If this is the first character, clear placeholder
                    if input.is_empty() && placeholder.is_some() {
                        execute!(stdout, terminal::Clear(ClearType::UntilNewLine))?;
                    }

                    input.push(c);

                    // Display character (or mask if password mode)
                    if password {
                        execute!(stdout, Print('*'))?;
                    } else {
                        execute!(stdout, Print(c))?;
                    }
                }

                _ => {
                    // Ignore other keys
                }
            }

            stdout.flush()?;
        }
    };

    // Disable raw mode and move to new line
    terminal::disable_raw_mode()?;
    execute!(stdout, Print("\n"))?;

    result
}
