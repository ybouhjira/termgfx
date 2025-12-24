use std::fs;
use std::io::{stdout, IsTerminal};
use std::thread;
use std::time::Duration;
use crate::output;
use crate::charts;

#[derive(Debug, Clone)]
pub struct ScriptCommand {
    pub command: String,
    pub args: Vec<String>,
    pub options: Vec<(String, String)>,
}

pub fn parse_script(content: &str) -> Vec<ScriptCommand> {
    let mut commands = Vec::new();

    for line in content.lines() {
        let line = line.trim();

        // Skip empty lines and comments
        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        // Parse command line
        if let Some(cmd) = parse_command_line(line) {
            commands.push(cmd);
        }
    }

    commands
}

fn parse_command_line(line: &str) -> Option<ScriptCommand> {
    let parts: Vec<&str> = line.split_whitespace().collect();
    if parts.is_empty() {
        return None;
    }

    let command = parts[0].to_string();
    let mut args = Vec::new();
    let mut options = Vec::new();

    let mut i = 1;
    while i < parts.len() {
        let part = parts[i];

        // Check if it's an option (contains :)
        if part.contains(':') {
            let opt_parts: Vec<&str> = part.splitn(2, ':').collect();
            if opt_parts.len() == 2 {
                options.push((opt_parts[0].to_string(), opt_parts[1].to_string()));
            }
        } else {
            // It's an argument - handle quoted strings
            if part.starts_with('"') {
                let mut quoted = part.trim_start_matches('"').to_string();
                i += 1;
                while i < parts.len() {
                    if parts[i].ends_with('"') {
                        quoted.push(' ');
                        quoted.push_str(parts[i].trim_end_matches('"'));
                        break;
                    } else {
                        quoted.push(' ');
                        quoted.push_str(parts[i]);
                    }
                    i += 1;
                }
                args.push(quoted);
            } else {
                args.push(part.to_string());
            }
        }
        i += 1;
    }

    Some(ScriptCommand { command, args, options })
}

fn get_option(options: &[(String, String)], key: &str) -> Option<String> {
    options.iter()
        .find(|(k, _)| k == key)
        .map(|(_, v)| v.clone())
}

fn parse_duration(duration_str: &str) -> Duration {
    if duration_str.ends_with("ms") {
        let ms = duration_str.trim_end_matches("ms").parse::<u64>().unwrap_or(1000);
        Duration::from_millis(ms)
    } else if duration_str.ends_with('s') {
        let s = duration_str.trim_end_matches('s').parse::<u64>().unwrap_or(1);
        Duration::from_secs(s)
    } else {
        // Default to seconds
        let s = duration_str.parse::<u64>().unwrap_or(1);
        Duration::from_secs(s)
    }
}

pub fn execute_script(commands: Vec<ScriptCommand>) {
    for cmd in commands {
        execute_command(&cmd);
    }
}

fn execute_command(cmd: &ScriptCommand) {
    match cmd.command.as_str() {
        "banner" => {
            let title = cmd.args.get(0).map(|s| s.as_str()).unwrap_or("Banner");
            let gradient = get_option(&cmd.options, "gradient");
            output::banner::render(title, gradient.as_deref());
        }

        "box" => {
            let message = cmd.args.get(0).map(|s| s.as_str()).unwrap_or("");
            let style = get_option(&cmd.options, "style").unwrap_or_else(|| "info".to_string());
            let border = get_option(&cmd.options, "border").unwrap_or_else(|| "rounded".to_string());
            let emoji = get_option(&cmd.options, "emoji");
            output::styled_box::render(message, &style, &border, emoji.as_deref());
        }

        "progress" => {
            let percent = if let Some(arg) = cmd.args.get(0) {
                if arg.contains('-') {
                    // Range format: 0-100
                    let parts: Vec<&str> = arg.split('-').collect();
                    if parts.len() == 2 {
                        parts[1].parse::<u8>().unwrap_or(100)
                    } else {
                        100
                    }
                } else {
                    arg.parse::<u8>().unwrap_or(0)
                }
            } else {
                100
            };

            let style = get_option(&cmd.options, "style").unwrap_or_else(|| "gradient".to_string());
            let duration = get_option(&cmd.options, "duration");

            if let Some(dur_str) = duration {
                // Animated progress - only if TTY
                if stdout().is_terminal() {
                    let _duration = parse_duration(&dur_str);
                    let steps = 20;
                    let step_duration = Duration::from_millis(100);

                    for i in 0..=steps {
                        let current = (i * percent as u32 / steps) as u8;
                        if i > 0 {
                            print!("\x1B[1A\x1B[2K"); // Move up and clear line
                        }
                        output::progress::render(current, &style, None, None);
                        thread::sleep(step_duration);
                    }
                } else {
                    // Not a TTY, just show final result
                    output::progress::render(percent, &style, None, None);
                }
            } else {
                output::progress::render(percent, &style, None, None);
            }
        }

        "spinner" => {
            let _message = cmd.args.get(0).map(|s| s.as_str()).unwrap_or("Loading...");
            let _style = get_option(&cmd.options, "style").unwrap_or_else(|| "dots".to_string());
            // Note: Spinner runs indefinitely with Ctrl+C, not suitable for scripts
            eprintln!("Note: spinner command not supported in scripts (runs indefinitely)");
        }

        "typewriter" => {
            let message = cmd.args.get(0).map(|s| s.as_str()).unwrap_or("");
            let speed = get_option(&cmd.options, "speed")
                .and_then(|s| s.parse::<u64>().ok())
                .unwrap_or(50);
            output::typewriter::render(message, speed);
        }

        "wait" => {
            if let Some(duration_str) = cmd.args.get(0) {
                let duration = parse_duration(duration_str);
                thread::sleep(duration);
            }
        }

        "sparkline" => {
            let data = cmd.args.get(0).map(|s| s.as_str()).unwrap_or("1,2,3,4,5");
            charts::sparkline::render(data);
        }

        _ => {
            eprintln!("Unknown command: {}", cmd.command);
        }
    }
}

pub fn run_script_file(path: &str) -> Result<(), String> {
    let content = fs::read_to_string(path)
        .map_err(|e| format!("Failed to read script file: {}", e))?;

    let commands = parse_script(&content);
    execute_script(commands);

    Ok(())
}

pub fn run_inline_script(script: &str) -> Result<(), String> {
    let commands = parse_script(script);
    execute_script(commands);
    Ok(())
}

pub fn run(file: Option<&str>, inline: Option<&str>) {
    if let Some(script_file) = file {
        if let Err(e) = run_script_file(script_file) {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    } else if let Some(inline_script) = inline {
        if let Err(e) = run_inline_script(inline_script) {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    } else {
        eprintln!("Error: Either --file or --inline must be provided");
        std::process::exit(1);
    }
}
