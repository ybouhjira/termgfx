/// Style configuration for notifications
#[allow(dead_code)]
struct NotificationStyle {
    emoji: &'static str,
    color: &'static str,
    sound: &'static str,
}

impl NotificationStyle {
    fn from_name(style: &str) -> Self {
        match style {
            "success" => NotificationStyle {
                emoji: "✅",
                color: "\x1b[32m", // Green
                sound: "Glass",
            },
            "warning" => NotificationStyle {
                emoji: "⚠️",
                color: "\x1b[33m", // Yellow
                sound: "Ping",
            },
            "error" => NotificationStyle {
                emoji: "❌",
                color: "\x1b[31m", // Red
                sound: "Basso",
            },
            _ => NotificationStyle {
                emoji: "ℹ️",
                color: "\x1b[36m", // Cyan
                sound: "Glass",
            },
        }
    }
}

/// Render a notification with both terminal and desktop components
pub fn render(
    message: &str,
    title: Option<&str>,
    style: &str,
    sound: bool,
    terminal_only: bool,
    desktop_only: bool,
) {
    let notification_style = NotificationStyle::from_name(style);

    // Show terminal notification (unless desktop-only)
    if !desktop_only {
        render_terminal(message, title, &notification_style);
    }

    // Show desktop notification (unless terminal-only)
    if !terminal_only {
        render_desktop(message, title, &notification_style, sound);
    }
}

/// Render terminal notification with styled box and bell character
fn render_terminal(message: &str, title: Option<&str>, style: &NotificationStyle) {
    let reset = "\x1b[0m";
    let bold = "\x1b[1m";

    // Bell character for audio feedback
    print!("\x07");

    // Top border
    println!(
        "{}{}╭─────────────────────────────────────────╮{}",
        style.color, bold, reset
    );

    // Title (if provided) or emoji-only line
    if let Some(t) = title {
        println!(
            "{}{}│ {}  {}                                 │{}",
            style.color, bold, style.emoji, t, reset
        );
        println!(
            "{}{}├─────────────────────────────────────────┤{}",
            style.color, bold, reset
        );
    } else {
        println!(
            "{}{}│ {}                                      │{}",
            style.color, bold, style.emoji, reset
        );
    }

    // Message (word-wrapped to fit box)
    let wrapped_lines = wrap_text(message, 38);
    for line in wrapped_lines {
        println!("{}{}│ {:<39} │{}", style.color, bold, line, reset);
    }

    // Bottom border
    println!(
        "{}{}╰─────────────────────────────────────────╯{}",
        style.color, bold, reset
    );
}

/// Render desktop notification using macOS osascript
fn render_desktop(message: &str, title: Option<&str>, style: &NotificationStyle, sound: bool) {
    #[cfg(target_os = "macos")]
    {
        use std::process::Command;
        let notification_title = format!("{} {}", style.emoji, title.unwrap_or("Notification"));

        let mut script = format!(
            "display notification \"{}\" with title \"{}\"",
            message.replace('"', "\\\""),
            notification_title.replace('"', "\\\"")
        );

        if sound {
            script.push_str(&format!(" sound name \"{}\"", style.sound));
        }

        let result = Command::new("osascript").arg("-e").arg(&script).output();

        // Graceful fallback - don't panic if desktop notification fails
        if let Err(_e) = result {
            // Silently fail - terminal notification already shown
            eprintln!("Note: Desktop notification unavailable");
        }
    }

    #[cfg(not(target_os = "macos"))]
    {
        // On non-macOS, silently skip desktop notification
        let _ = (message, title, style, sound);
    }
}

/// Wrap text to fit within a specified width
fn wrap_text(text: &str, width: usize) -> Vec<String> {
    let mut lines = Vec::new();
    let mut current_line = String::new();

    for word in text.split_whitespace() {
        if current_line.is_empty() {
            current_line = word.to_string();
        } else if current_line.len() + word.len() < width {
            current_line.push(' ');
            current_line.push_str(word);
        } else {
            lines.push(current_line);
            current_line = word.to_string();
        }
    }

    if !current_line.is_empty() {
        lines.push(current_line);
    }

    if lines.is_empty() {
        lines.push(String::new());
    }

    lines
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wrap_text_short() {
        let result = wrap_text("Hello", 20);
        assert_eq!(result, vec!["Hello"]);
    }

    #[test]
    fn test_wrap_text_long() {
        let result = wrap_text("This is a very long message that needs wrapping", 20);
        assert!(result.len() > 1);
        for line in result {
            assert!(line.len() <= 20);
        }
    }

    #[test]
    fn test_notification_styles() {
        let info = NotificationStyle::from_name("info");
        assert_eq!(info.emoji, "ℹ️");

        let success = NotificationStyle::from_name("success");
        assert_eq!(success.emoji, "✅");

        let warning = NotificationStyle::from_name("warning");
        assert_eq!(warning.emoji, "⚠️");

        let error = NotificationStyle::from_name("error");
        assert_eq!(error.emoji, "❌");
    }
}
