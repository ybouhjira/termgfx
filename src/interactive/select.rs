use crossterm::{
    cursor::{Hide, MoveTo, Show},
    event::{self, Event, KeyCode, KeyEvent},
    execute,
    style::{Color, Print, ResetColor, SetForegroundColor, Stylize},
    terminal::{self, Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{
    collections::HashSet,
    io::{self, IsTerminal, Write},
};

pub fn render(prompt: &str, options: &[String], multi: bool) {
    if options.is_empty() {
        eprintln!("Error: No options provided");
        std::process::exit(1);
    }

    match run_select(prompt, options, multi) {
        Ok(selected) => {
            if multi {
                println!("{}", selected.join(","));
            } else {
                println!("{}", selected[0]);
            }
        }
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    }
}

fn run_select(prompt: &str, options: &[String], multi: bool) -> io::Result<Vec<String>> {
    // Check for interactive terminal
    if !std::io::stdin().is_terminal() {
        return Err(io::Error::new(
            io::ErrorKind::Other,
            "Select requires an interactive terminal (TTY)",
        ));
    }

    let mut stdout = io::stdout();
    let mut selected_idx = 0;
    let mut selected_items: HashSet<usize> = HashSet::new();

    // Setup terminal
    terminal::enable_raw_mode()?;
    execute!(stdout, EnterAlternateScreen, Hide)?;

    let result = loop {
        // Render the prompt and options
        render_menu(
            &mut stdout,
            prompt,
            options,
            selected_idx,
            &selected_items,
            multi,
        )?;

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
                KeyCode::Char(' ') if multi => {
                    if selected_items.contains(&selected_idx) {
                        selected_items.remove(&selected_idx);
                    } else {
                        selected_items.insert(selected_idx);
                    }
                }
                KeyCode::Enter => {
                    if multi {
                        let mut result_vec = selected_items
                            .iter()
                            .map(|&idx| options[idx].clone())
                            .collect::<Vec<String>>();
                        result_vec
                            .sort_by_key(|item| options.iter().position(|x| x == item).unwrap()); // Maintain original order
                        break Ok(result_vec);
                    } else {
                        break Ok(vec![options[selected_idx].clone()]);
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
    selected_idx: usize,
    selected_items: &HashSet<usize>,
    multi: bool,
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
        let is_current = idx == selected_idx;
        let is_selected_multi = selected_items.contains(&idx);

        let prefix = if multi {
            if is_selected_multi {
                "[x]"
            } else {
                "[ ]"
            }
        } else {
            " "
        };

        let indicator = if is_current { "❯" } else { " " };
        let formatted_option = if is_current {
            option.clone().bold().to_string()
        } else {
            option.clone()
        };

        execute!(
            stdout,
            SetForegroundColor(if is_current {
                Color::Green
            } else {
                Color::Reset
            }),
            Print(format!("{} {} {}", indicator, prefix, formatted_option)),
            ResetColor,
            Print("\n")
        )?;
    }

    // Print help text
    let help_text = if multi {
        "↑↓: Navigate • Space: Toggle • Enter: Select • Esc: Cancel"
    } else {
        "↑↓: Navigate • Enter: Select • Esc: Cancel"
    };

    execute!(
        stdout,
        Print("\n"),
        SetForegroundColor(Color::DarkGrey),
        Print(help_text),
        ResetColor
    )?;

    stdout.flush()?;
    Ok(())
}
