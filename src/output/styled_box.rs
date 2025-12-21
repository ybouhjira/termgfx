use owo_colors::{OwoColorize, Style};
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
            },
            "double" => BorderChars {
                top_left: "╔",
                top_right: "╗",
                bottom_left: "╚",
                bottom_right: "╝",
                horizontal: "═",
                vertical: "║",
            },
            "rounded" => BorderChars {
                top_left: "╭",
                top_right: "╮",
                bottom_left: "╰",
                bottom_right: "╯",
                horizontal: "─",
                vertical: "│",
            },
            "heavy" | "thick" => BorderChars {
                top_left: "┏",
                top_right: "┓",
                bottom_left: "┗",
                bottom_right: "┛",
                horizontal: "━",
                vertical: "┃",
            },
            "ascii" => BorderChars {
                top_left: "+",
                top_right: "+",
                bottom_left: "+",
                bottom_right: "+",
                horizontal: "-",
                vertical: "|",
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
    println!("{}", top_border.style(color_style));

    for (idx, line) in lines.iter().enumerate() {
        let mut content = String::new();
        if idx == 0 && emoji_str.is_some() {
            content.push_str(emoji_str.unwrap());
            content.push(' ');
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
    }

    let bottom_border = format!(
        "{}{}{}",
        borders.bottom_left,
        borders.horizontal.repeat(box_width),
        borders.bottom_right
    );
    println!("{}", bottom_border.style(color_style));
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
