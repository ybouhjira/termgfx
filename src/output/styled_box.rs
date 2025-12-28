use owo_colors::{OwoColorize, Style};
use std::io::{stdout, Write};
use std::thread;
use std::time::Duration;
use unicode_width::UnicodeWidthStr;

/// Border character set for different styles
#[derive(Debug, Clone)]
struct BorderChars {
    top_left: &'static str,
    top_right: &'static str,
    bottom_left: &'static str,
    bottom_right: &'static str,
    horizontal: &'static str,
    vertical: &'static str,
    header_left: &'static str,
    header_right: &'static str,
}

impl BorderChars {
    fn get(border_style: &str) -> Self {
        match border_style.to_lowercase().as_str() {
            "single" => BorderChars {
                top_left: "┌",
                top_right: "┐",
                bottom_left: "└",
                bottom_right: "┘",
                horizontal: "─",
                vertical: "│",
                header_left: "├",
                header_right: "┤",
            },
            "double" => BorderChars {
                top_left: "╔",
                top_right: "╗",
                bottom_left: "╚",
                bottom_right: "╝",
                horizontal: "═",
                vertical: "║",
                header_left: "╠",
                header_right: "╣",
            },
            "rounded" => BorderChars {
                top_left: "╭",
                top_right: "╮",
                bottom_left: "╰",
                bottom_right: "╯",
                horizontal: "─",
                vertical: "│",
                header_left: "├",
                header_right: "┤",
            },
            "heavy" | "thick" => BorderChars {
                top_left: "┏",
                top_right: "┓",
                bottom_left: "┗",
                bottom_right: "┛",
                horizontal: "━",
                vertical: "┃",
                header_left: "┣",
                header_right: "┫",
            },
            "ascii" => BorderChars {
                top_left: "+",
                top_right: "+",
                bottom_left: "+",
                bottom_right: "+",
                horizontal: "-",
                vertical: "|",
                header_left: "+",
                header_right: "+",
            },
            _ => BorderChars::get("rounded"),
        }
    }
}

/// Get color style for the box
fn get_style(style_name: &str) -> Style {
    match style_name.to_lowercase().as_str() {
        "success" => Style::new().bright_green().bold(),
        "warning" => Style::new().bright_yellow().bold(),
        "danger" | "error" => Style::new().bright_red().bold(),
        "info" => Style::new().bright_blue().bold(),
        "gradient" => Style::new().bright_magenta().bold(),
        _ => Style::new().white(),
    }
}

/// Get emoji for style if not provided
fn get_default_emoji(style_name: &str) -> Option<&'static str> {
    match style_name.to_lowercase().as_str() {
        "success" => Some("✓"),
        "warning" => Some("⚠"),
        "danger" | "error" => Some("🚨"),
        "info" => Some("ℹ"),
        "gradient" => Some("✨"),
        _ => None,
    }
}

/// Render a styled box with the given message
pub fn render(message: &str, style: &str, border: &str, emoji: Option<&str>) {
    render_animated(message, style, border, emoji, false, 500);
}

/// Render a danger zone box with header
/// This creates a prominent warning box for destructive operations
pub fn render_danger_zone(
    message: &str,
    title: Option<&str>,
    border: &str,
    animate: bool,
    animation_time_ms: u64,
) {
    let borders = BorderChars::get(border);
    let danger_style = Style::new().bright_red().bold();
    let header_style = Style::new().on_bright_red().white().bold();
    let title_text = title.unwrap_or("⚠️  DANGER ZONE");

    let lines: Vec<&str> = message.lines().collect();
    let padding = 2;

    // Calculate widths
    let title_width = UnicodeWidthStr::width(title_text);
    let max_content_width = lines
        .iter()
        .map(|l| UnicodeWidthStr::width(*l))
        .max()
        .unwrap_or(0);
    let max_width = title_width.max(max_content_width);
    let box_width = max_width + (padding * 2);

    // Calculate delay
    let total_elements = lines.len() + 4; // title + header separator + content lines + borders
    let delay = if animate && total_elements > 0 {
        Duration::from_millis(animation_time_ms / total_elements as u64)
    } else {
        Duration::ZERO
    };
    let mut stdout = stdout();

    // Top border
    let top_border = format!(
        "{}{}{}",
        borders.top_left,
        borders.horizontal.repeat(box_width),
        borders.top_right
    );
    println!("{}", top_border.style(danger_style));
    if animate {
        stdout.flush().unwrap();
        thread::sleep(delay);
    }

    // Title line with background
    let title_padding = box_width - title_width;
    let left_pad = title_padding / 2;
    let right_pad = title_padding - left_pad;
    let title_line = format!(
        "{}{}{}{}{}",
        borders.vertical.style(danger_style),
        " ".repeat(left_pad).style(header_style),
        title_text.style(header_style),
        " ".repeat(right_pad).style(header_style),
        borders.vertical.style(danger_style)
    );
    println!("{}", title_line);
    if animate {
        stdout.flush().unwrap();
        thread::sleep(delay);
    }

    // Header separator
    let header_sep = format!(
        "{}{}{}",
        borders.header_left,
        borders.horizontal.repeat(box_width),
        borders.header_right
    );
    println!("{}", header_sep.style(danger_style));
    if animate {
        stdout.flush().unwrap();
        thread::sleep(delay);
    }

    // Content lines
    for line in &lines {
        let content_width = UnicodeWidthStr::width(*line);
        let total_padding = box_width - content_width;
        let left_padding = padding;
        let right_padding = total_padding - left_padding;
        let formatted_line = format!(
            "{}{}{}{:width$}{}",
            borders.vertical,
            " ".repeat(left_padding),
            line.bright_red(),
            "",
            borders.vertical,
            width = right_padding
        );
        println!("{}", formatted_line.style(danger_style));
        if animate {
            stdout.flush().unwrap();
            thread::sleep(delay);
        }
    }

    // Bottom border
    let bottom_border = format!(
        "{}{}{}",
        borders.bottom_left,
        borders.horizontal.repeat(box_width),
        borders.bottom_right
    );
    println!("{}", bottom_border.style(danger_style));
    if animate {
        stdout.flush().unwrap();
        thread::sleep(delay);
    }
}

/// Render a styled box with optional animation
/// animation_time_ms: total animation duration in milliseconds (delay is calculated per line)
pub fn render_animated(
    message: &str,
    style: &str,
    border: &str,
    emoji: Option<&str>,
    animate: bool,
    animation_time_ms: u64,
) {
    let borders = BorderChars::get(border);
    let color_style = get_style(style);
    let emoji_str = emoji.or_else(|| get_default_emoji(style));
    let lines: Vec<&str> = message.lines().collect();
    let padding = 2;
    let mut max_width = 0;

    for line in &lines {
        let mut line_width = UnicodeWidthStr::width(*line);
        if emoji_str.is_some() && lines.iter().position(|&l| l == *line) == Some(0) {
            line_width += 2;
        }
        max_width = max_width.max(line_width);
    }

    let box_width = max_width + (padding * 2);
    let top_border = format!(
        "{}{}{}",
        borders.top_left,
        borders.horizontal.repeat(box_width),
        borders.top_right
    );

    // Calculate delay per line: total_time / (lines + 2 borders)
    let total_elements = lines.len() + 2; // content lines + top + bottom border
    let delay = if animate && total_elements > 0 {
        Duration::from_millis(animation_time_ms / total_elements as u64)
    } else {
        Duration::ZERO
    };
    let mut stdout = stdout();

    // Print top border
    println!("{}", top_border.style(color_style));
    if animate {
        stdout.flush().unwrap();
        thread::sleep(delay);
    }

    for (idx, line) in lines.iter().enumerate() {
        let mut content = String::new();
        if idx == 0 {
            if let Some(emoji) = emoji_str {
                content.push_str(emoji);
                content.push(' ');
            }
        }
        content.push_str(line);
        let content_width = UnicodeWidthStr::width(content.as_str());
        let total_padding = box_width - content_width;
        let left_padding = padding;
        let right_padding = total_padding - left_padding;
        let formatted_line = format!(
            "{}{}{}{:width$}{}",
            borders.vertical,
            " ".repeat(left_padding),
            content,
            "",
            borders.vertical,
            width = right_padding
        );

        println!("{}", formatted_line.style(color_style));
        if animate {
            stdout.flush().unwrap();
            thread::sleep(delay);
        }
    }

    let bottom_border = format!(
        "{}{}{}",
        borders.bottom_left,
        borders.horizontal.repeat(box_width),
        borders.bottom_right
    );

    println!("{}", bottom_border.style(color_style));
    if animate {
        stdout.flush().unwrap();
        thread::sleep(delay);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_border_chars_single() {
        let borders = BorderChars::get("single");
        assert_eq!(borders.top_left, "┌");
        assert_eq!(borders.horizontal, "─");
    }

    #[test]
    fn test_border_chars_double() {
        let borders = BorderChars::get("double");
        assert_eq!(borders.top_left, "╔");
        assert_eq!(borders.horizontal, "═");
    }

    #[test]
    fn test_border_chars_ascii() {
        let borders = BorderChars::get("ascii");
        assert_eq!(borders.top_left, "+");
        assert_eq!(borders.horizontal, "-");
    }

    #[test]
    fn test_get_default_emoji() {
        assert_eq!(get_default_emoji("success"), Some("✓"));
        assert_eq!(get_default_emoji("warning"), Some("⚠"));
        assert_eq!(get_default_emoji("danger"), Some("🚨"));
    }

    #[test]
    fn test_render_basic() {
        render("Test", "info", "rounded", None);
        render("Multi\nLine", "success", "double", Some("🎉"));
    }
}
