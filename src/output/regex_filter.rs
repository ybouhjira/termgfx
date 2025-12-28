//! Regex pattern selector for filtering entries
//!
//! Displays items with regex pattern matching, showing which entries match
//! and which don't. Useful for previewing bulk operations before execution.

use owo_colors::{OwoColorize, Style};
use regex::Regex;
use unicode_width::UnicodeWidthStr;

/// Border character set for the filter pane
struct BorderChars {
    top_left: &'static str,
    top_right: &'static str,
    bottom_left: &'static str,
    bottom_right: &'static str,
    horizontal: &'static str,
    vertical: &'static str,
    header_left: &'static str,
    header_right: &'static str,
    footer_left: &'static str,
    footer_right: &'static str,
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
                footer_left: "├",
                footer_right: "┤",
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
                footer_left: "╠",
                footer_right: "╣",
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
                footer_left: "├",
                footer_right: "┤",
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
                footer_left: "┣",
                footer_right: "┫",
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
                footer_left: "+",
                footer_right: "+",
            },
            _ => BorderChars::get("rounded"),
        }
    }
}

/// Result of filtering items
#[derive(Debug)]
pub struct FilterResult {
    pub matches: Vec<String>,
    pub non_matches: Vec<String>,
    pub match_count: usize,
    pub total_count: usize,
}

/// Configuration for the regex filter
pub struct RegexFilterConfig {
    pub pattern: String,
    pub action: String,
    pub cancel_label: String,
    pub border: String,
    pub max_items: usize,
    pub show_non_matches: bool,
    pub case_insensitive: bool,
    pub invert: bool,
}

impl Default for RegexFilterConfig {
    fn default() -> Self {
        Self {
            pattern: String::new(),
            action: "Apply".to_string(),
            cancel_label: "Cancel".to_string(),
            border: "rounded".to_string(),
            max_items: 20,
            show_non_matches: true,
            case_insensitive: false,
            invert: false,
        }
    }
}

/// Filter items using a regex pattern
pub fn filter_items(items: &[String], config: &RegexFilterConfig) -> Result<FilterResult, String> {
    let pattern = if config.case_insensitive {
        format!("(?i){}", config.pattern)
    } else {
        config.pattern.clone()
    };

    let regex = Regex::new(&pattern).map_err(|e| format!("Invalid regex: {}", e))?;

    let mut matches = Vec::new();
    let mut non_matches = Vec::new();

    for item in items {
        let is_match = regex.is_match(item);
        let is_match = if config.invert { !is_match } else { is_match };

        if is_match {
            matches.push(item.clone());
        } else {
            non_matches.push(item.clone());
        }
    }

    Ok(FilterResult {
        match_count: matches.len(),
        total_count: items.len(),
        matches,
        non_matches,
    })
}

/// Render the regex filter pane
pub fn render(items: &[String], config: &RegexFilterConfig) -> Result<FilterResult, String> {
    let result = filter_items(items, config)?;
    let borders = BorderChars::get(&config.border);
    let padding = 2;

    // Calculate header
    let header_text = format!("🔍 Pattern: {}", config.pattern);
    let match_info = format!(
        "Matching entries ({}/{})",
        result.match_count, result.total_count
    );

    // Get items to display
    let display_matches: Vec<&String> = result.matches.iter().take(config.max_items).collect();
    let truncated_matches = result.matches.len() > config.max_items;

    let display_non_matches: Vec<&String> = if config.show_non_matches {
        result
            .non_matches
            .iter()
            .take(config.max_items.saturating_sub(display_matches.len()))
            .collect()
    } else {
        vec![]
    };

    // Calculate max width
    let max_item_width = display_matches
        .iter()
        .chain(display_non_matches.iter())
        .map(|s| s.len() + 4) // ☑/☐ prefix
        .max()
        .unwrap_or(0);

    let header_width = UnicodeWidthStr::width(header_text.as_str());
    let match_info_width = UnicodeWidthStr::width(match_info.as_str());
    let action_button = format!("[{}]", config.action);
    let cancel_button = format!("[{}]", config.cancel_label);
    let footer_width = action_button.len() + cancel_button.len() + 3;

    let max_width = max_item_width
        .max(header_width)
        .max(match_info_width)
        .max(footer_width);
    let box_width = max_width + (padding * 2);

    let match_style = Style::new().bright_green().bold();
    let non_match_style = Style::new().dimmed();
    let border_style = Style::new().cyan().bold();

    // Top border
    println!(
        "{}",
        format!(
            "{}{}{}",
            borders.top_left,
            borders.horizontal.repeat(box_width),
            borders.top_right
        )
        .style(border_style)
    );

    // Header with pattern
    let header_padding = box_width - header_width;
    let left_pad = padding;
    let right_pad = header_padding - left_pad;
    println!(
        "{}",
        format!(
            "{}{}{}{}{}",
            borders.vertical,
            " ".repeat(left_pad),
            header_text.bold(),
            " ".repeat(right_pad),
            borders.vertical
        )
        .style(border_style)
    );

    // Header separator
    println!(
        "{}",
        format!(
            "{}{}{}",
            borders.header_left,
            borders.horizontal.repeat(box_width),
            borders.header_right
        )
        .style(border_style)
    );

    // Match info line
    let info_padding = box_width - match_info_width;
    let right_info_pad = info_padding - padding;
    println!(
        "{}{}{}{}{}",
        borders.vertical.style(border_style),
        " ".repeat(padding),
        match_info.style(match_style),
        " ".repeat(right_info_pad),
        borders.vertical.style(border_style)
    );

    // Matching items
    for item in &display_matches {
        let content = format!(" ☑ {}", item);
        let content_width = UnicodeWidthStr::width(content.as_str());
        let total_padding = box_width.saturating_sub(content_width);
        let right_padding = total_padding.saturating_sub(padding);

        println!(
            "{}{}{}{}{}",
            borders.vertical.style(border_style),
            " ".repeat(padding),
            content.style(match_style),
            " ".repeat(right_padding),
            borders.vertical.style(border_style)
        );
    }

    // Show truncation for matches
    if truncated_matches {
        let more_text = format!(
            "  ... and {} more matches",
            result.matches.len() - config.max_items
        );
        let more_width = UnicodeWidthStr::width(more_text.as_str());
        let total_padding = box_width.saturating_sub(more_width);
        let right_padding = total_padding.saturating_sub(padding);

        println!(
            "{}{}{}{}{}",
            borders.vertical.style(border_style),
            " ".repeat(padding),
            more_text.dimmed(),
            " ".repeat(right_padding),
            borders.vertical.style(border_style)
        );
    }

    // Non-matching items
    if config.show_non_matches && !display_non_matches.is_empty() {
        for item in &display_non_matches {
            let content = format!(" ☐ {} (no match)", item);
            let content_width = UnicodeWidthStr::width(content.as_str());
            let total_padding = box_width.saturating_sub(content_width);
            let right_padding = total_padding.saturating_sub(padding);

            println!(
                "{}{}{}{}{}",
                borders.vertical.style(border_style),
                " ".repeat(padding),
                content.style(non_match_style),
                " ".repeat(right_padding),
                borders.vertical.style(border_style)
            );
        }
    }

    // Footer separator
    println!(
        "{}",
        format!(
            "{}{}{}",
            borders.footer_left,
            borders.horizontal.repeat(box_width),
            borders.footer_right
        )
        .style(border_style)
    );

    // Action buttons
    let button_style = Style::new().on_bright_cyan().black().bold();
    let cancel_style = Style::new().dimmed();

    let buttons = format!(
        "{} {}",
        action_button.style(button_style),
        cancel_button.style(cancel_style)
    );
    let buttons_raw_width = action_button.len() + cancel_button.len() + 1;
    let buttons_padding = box_width - buttons_raw_width;
    let left_button_pad = buttons_padding / 2;
    let right_button_pad = buttons_padding - left_button_pad;

    println!(
        "{}{}{}{}{}",
        borders.vertical.style(border_style),
        " ".repeat(left_button_pad),
        buttons,
        " ".repeat(right_button_pad),
        borders.vertical.style(border_style)
    );

    // Bottom border
    println!(
        "{}",
        format!(
            "{}{}{}",
            borders.bottom_left,
            borders.horizontal.repeat(box_width),
            borders.bottom_right
        )
        .style(border_style)
    );

    Ok(result)
}

/// Render filter and output matches only (for piping)
pub fn render_matches_only(items: &[String], config: &RegexFilterConfig) -> Result<(), String> {
    let result = filter_items(items, config)?;
    for item in result.matches {
        println!("{}", item);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_filter_simple() {
        let items = vec![
            "app.log".to_string(),
            "config.json".to_string(),
            "error.log".to_string(),
        ];
        let config = RegexFilterConfig {
            pattern: r"\.log$".to_string(),
            ..Default::default()
        };
        let result = filter_items(&items, &config).unwrap();
        assert_eq!(result.match_count, 2);
        assert_eq!(result.matches, vec!["app.log", "error.log"]);
    }

    #[test]
    fn test_filter_case_insensitive() {
        let items = vec![
            "ERROR.log".to_string(),
            "error.log".to_string(),
            "config.json".to_string(),
        ];
        let config = RegexFilterConfig {
            pattern: "error".to_string(),
            case_insensitive: true,
            ..Default::default()
        };
        let result = filter_items(&items, &config).unwrap();
        assert_eq!(result.match_count, 2);
    }

    #[test]
    fn test_filter_invert() {
        let items = vec![
            "app.log".to_string(),
            "config.json".to_string(),
            "error.log".to_string(),
        ];
        let config = RegexFilterConfig {
            pattern: r"\.log$".to_string(),
            invert: true,
            ..Default::default()
        };
        let result = filter_items(&items, &config).unwrap();
        assert_eq!(result.match_count, 1);
        assert_eq!(result.matches, vec!["config.json"]);
    }

    #[test]
    fn test_filter_invalid_regex() {
        let items = vec!["test".to_string()];
        let config = RegexFilterConfig {
            pattern: "[invalid".to_string(),
            ..Default::default()
        };
        let result = filter_items(&items, &config);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Invalid regex"));
    }

    #[test]
    fn test_filter_empty_pattern() {
        let items = vec!["a".to_string(), "b".to_string()];
        let config = RegexFilterConfig {
            pattern: "".to_string(),
            ..Default::default()
        };
        let result = filter_items(&items, &config).unwrap();
        assert_eq!(result.match_count, 2); // Empty pattern matches everything
    }

    #[test]
    fn test_render_basic() {
        let items = vec!["app.log".to_string(), "config.json".to_string()];
        let config = RegexFilterConfig {
            pattern: r"\.log$".to_string(),
            ..Default::default()
        };
        let result = render(&items, &config);
        assert!(result.is_ok());
    }
}
