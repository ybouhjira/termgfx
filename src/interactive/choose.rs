use crossterm::{
    cursor::{Hide, MoveTo, Show},
    event::{self, Event, KeyCode, KeyEvent},
    execute,
    style::{Color, Print, ResetColor, SetForegroundColor, Stylize},
    terminal::{self, Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::collections::HashSet;
use std::io::{self, Write};

pub fn render(prompt: &str, options: &[String], multi: bool) {
    if options.is_empty() {
        eprintln!("Error: No options provided");
        std::process::exit(1);
    }

    match run_choose(prompt, options, multi) {
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

fn run_choose(prompt: &str, options: &[String], multi: bool) -> io::Result<Vec<String>> {
    let mut stdout = io::stdout();
    let mut current_idx = 0;
    let mut selected_indices: HashSet<usize> = HashSet::new();

    // Setup terminal
    terminal::enable_raw_mode()?;
    execute!(stdout, EnterAlternateScreen, Hide)?;

    let result = loop {
        // Render the prompt and options
        render_menu(&mut stdout, prompt, options, current_idx, &selected_indices, multi)?;

        // Handle key events
        if let Event::Key(KeyEvent { code, .. }) = event::read()? {
            match code {
                KeyCode::Up | KeyCode::Char('k') => {
                    if current_idx > 0 {
                        current_idx -= 1;
                    }
                }
                KeyCode::Down | KeyCode::Char('j') => {
                    if current_idx < options.len() - 1 {
                        current_idx += 1;
                    }
                }
                KeyCode::Char(' ') if multi => {
                    // Toggle selection
                    if selected_indices.contains(&current_idx) {
                        selected_indices.remove(&current_idx);
                    } else {
                        selected_indices.insert(current_idx);
                    }
                }
                KeyCode::Enter => {
                    if multi {
                        // Return all selected items
                        let mut result: Vec<String> = selected_indices
                            .iter()
                            .map(|&idx| options[idx].clone())
                            .collect();
                        result.sort_by_key(|item| {
                            options.iter().position(|opt| opt == item).unwrap()
                        });
                        break Ok(result);
                    } else {
                        // Single select - return current item
                        break Ok(vec![options[current_idx].clone()]);
                    }
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
    current_idx: usize,
    selected_indices: &HashSet<usize>,
    multi: bool,
) -> io::Result<()> {
    execute!(stdout, Clear(ClearType::All), MoveTo(0, 0))?;

    // Print prompt with emoji
    execute!(
        stdout,
        SetForegroundColor(Color::Cyan),
        Print("❯ "),
        ResetColor,
        Print(prompt.bold()),
        Print("\n")
    )?;

    // Print helper text
    if multi {
        execute!(
            stdout,
            SetForegroundColor(Color::DarkGrey),
            Print("  (space to toggle, enter to confirm)\n\n"),
            ResetColor
        )?;
    } else {
        execute!(
            stdout,
            Print("\n")
        )?;
    }

    // Print options
    for (idx, option) in options.iter().enumerate() {
        let is_current = idx == current_idx;
        let is_selected = selected_indices.contains(&idx);

        // Cursor indicator
        if is_current {
            execute!(
                stdout,
                SetForegroundColor(Color::Green),
                Print("> "),
                ResetColor
            )?;
        } else {
            execute!(stdout, Print("  "))?;
        }

        // Checkbox (only for multi-select)
        if multi {
            if is_selected {
                execute!(
                    stdout,
                    SetForegroundColor(Color::Green),
                    Print("[x] "),
                    ResetColor
                )?;
            } else {
                execute!(
                    stdout,
                    SetForegroundColor(Color::DarkGrey),
                    Print("[ ] "),
                    ResetColor
                )?;
            }
        }

        // Option text
        if is_current {
            execute!(
                stdout,
                SetForegroundColor(Color::Green),
                Print(option.clone().bold()),
                ResetColor
            )?;
        } else if is_selected {
            execute!(
                stdout,
                SetForegroundColor(Color::Cyan),
                Print(option),
                ResetColor
            )?;
        } else {
            execute!(stdout, Print(option))?;
        }

        execute!(stdout, Print("\n"))?;
    }

    // Print help text
    execute!(
        stdout,
        Print("\n"),
        SetForegroundColor(Color::DarkGrey),
    )?;

    if multi {
        execute!(
            stdout,
            Print("↑↓: Navigate • Space: Toggle • Enter: Confirm • Esc: Cancel")
        )?;
    } else {
        execute!(
            stdout,
            Print("↑↓: Navigate • Enter: Select • Esc: Cancel")
        )?;
    }

    execute!(stdout, ResetColor)?;
    stdout.flush()?;
    Ok(())
}
