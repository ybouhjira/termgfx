use owo_colors::OwoColorize;
use unicode_width::UnicodeWidthStr;
use std::io::{stdout, Write};
use std::thread;
use std::time::Duration;

struct BorderChars {
    top_left: &'static str,
    top_right: &'static str,
    bottom_left: &'static str,
    bottom_right: &'static str,
    horizontal: &'static str,
    vertical: &'static str,
}

impl BorderChars {
    fn double() -> Self {
        BorderChars {
            top_left: "╔",
            top_right: "╗",
            bottom_left: "╚",
            bottom_right: "╝",
            horizontal: "═",
            vertical: "║",
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum GradientColors {
    BluePurple,
    RedOrange,
    GreenCyan,
    PinkYellow,
    Default,
}

impl GradientColors {
    fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "blue-purple" | "cyan-purple" => GradientColors::BluePurple,
            "red-orange" => GradientColors::RedOrange,
            "green-cyan" => GradientColors::GreenCyan,
            "pink-yellow" | "magenta-yellow" => GradientColors::PinkYellow,
            _ => GradientColors::Default,
        }
    }
}

fn get_terminal_width() -> usize {
    use crossterm::terminal;
    if let Ok((width, _)) = terminal::size() {
        width as usize
    } else {
        80
    }
}

fn apply_gradient(text: &str, gradient: GradientColors, position: f32) -> String {
    let chars: Vec<char> = text.chars().collect();
    let len = chars.len();
    if len == 0 {
        return String::new();
    }
    let mut result = String::new();
    for (idx, ch) in chars.iter().enumerate() {
        let ratio = if len > 1 { (idx as f32) / ((len - 1) as f32) } else { 0.5 };
        let final_ratio = (ratio + position) / 2.0;
        let colored_char = match gradient {
            GradientColors::BluePurple => {
                if final_ratio < 0.5 { ch.to_string().bright_blue().to_string() } else { ch.to_string().bright_magenta().to_string() }
            }
            GradientColors::RedOrange => {
                if final_ratio < 0.5 { ch.to_string().bright_red().to_string() } else { ch.to_string().bright_yellow().to_string() }
            }
            GradientColors::GreenCyan => {
                if final_ratio < 0.5 { ch.to_string().bright_green().to_string() } else { ch.to_string().bright_cyan().to_string() }
            }
            GradientColors::PinkYellow => {
                if final_ratio < 0.5 { ch.to_string().bright_magenta().to_string() } else { ch.to_string().bright_yellow().to_string() }
            }
            GradientColors::Default => {
                if final_ratio < 0.5 { ch.to_string().bright_cyan().to_string() } else { ch.to_string().bright_magenta().to_string() }
            }
        };
        result.push_str(&colored_char);
    }
    result
}

pub fn render(title: &str, gradient: Option<&str>) {
    render_animated(title, gradient, false, 500);
}

/// Render banner with optional animation
/// animation_time_ms: total animation duration in milliseconds (delay is calculated per line)
pub fn render_animated(title: &str, gradient: Option<&str>, animate: bool, animation_time_ms: u64) {
    let borders = BorderChars::double();
    let term_width = get_terminal_width();
    let gradient_colors = gradient.map(GradientColors::from_str).unwrap_or(GradientColors::Default);
    let parts: Vec<&str> = title.split('|').collect();
    let main_title = parts[0].trim();
    let subtitle = parts.get(1).map(|s| s.trim());
    let padding = 4;
    let title_width = UnicodeWidthStr::width(main_title);
    let subtitle_width = subtitle.map(|s| UnicodeWidthStr::width(s)).unwrap_or(0);
    let min_content_width = title_width.max(subtitle_width) + padding + 2;
    let banner_width = if term_width > min_content_width { term_width.min(100) } else { min_content_width };
    let inner_width = banner_width.saturating_sub(2);

    // Calculate total lines: top border + empty + title + optional subtitle + empty + bottom border
    let total_lines = if subtitle.is_some() { 6 } else { 5 };
    let delay = if animate && total_lines > 0 {
        Duration::from_millis(animation_time_ms / total_lines as u64)
    } else {
        Duration::ZERO
    };
    let mut stdout = stdout();

    let top_border = format!("{}{}{}", borders.top_left, borders.horizontal.repeat(inner_width), borders.top_right);
    print_animated(&apply_gradient(&top_border, gradient_colors, 0.0), animate, delay, &mut stdout);

    render_banner_line_animated("", inner_width, &borders, gradient_colors, 0.2, animate, delay, &mut stdout);
    render_banner_line_animated(main_title, inner_width, &borders, gradient_colors, 0.4, animate, delay, &mut stdout);
    if let Some(sub) = subtitle {
        render_banner_line_animated(sub, inner_width, &borders, gradient_colors, 0.6, animate, delay, &mut stdout);
    }
    render_banner_line_animated("", inner_width, &borders, gradient_colors, 0.8, animate, delay, &mut stdout);

    let bottom_border = format!("{}{}{}", borders.bottom_left, borders.horizontal.repeat(inner_width), borders.bottom_right);
    print_animated(&apply_gradient(&bottom_border, gradient_colors, 1.0), animate, delay, &mut stdout);
}

fn print_animated(text: &str, animate: bool, delay: Duration, stdout: &mut std::io::Stdout) {
    println!("{}", text);
    if animate {
        stdout.flush().unwrap();
        thread::sleep(delay);
    }
}

fn render_banner_line(text: &str, width: usize, borders: &BorderChars, gradient: GradientColors, position: f32) {
    let text_width = UnicodeWidthStr::width(text);
    let available_space = width.saturating_sub(text_width);
    let left_padding = available_space / 2;
    let right_padding = available_space - left_padding;
    let line = format!("{}{}{}{}{}", borders.vertical, " ".repeat(left_padding), text, " ".repeat(right_padding), borders.vertical);
    println!("{}", apply_gradient(&line, gradient, position));
}

fn render_banner_line_animated(text: &str, width: usize, borders: &BorderChars, gradient: GradientColors, position: f32, animate: bool, delay: Duration, stdout: &mut std::io::Stdout) {
    let text_width = UnicodeWidthStr::width(text);
    let available_space = width.saturating_sub(text_width);
    let left_padding = available_space / 2;
    let right_padding = available_space - left_padding;
    let line = format!("{}{}{}{}{}", borders.vertical, " ".repeat(left_padding), text, " ".repeat(right_padding), borders.vertical);
    print_animated(&apply_gradient(&line, gradient, position), animate, delay, stdout);
}
