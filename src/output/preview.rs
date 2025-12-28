//! Preview pane component for data inspection before actions
//!
//! Displays a list of items with a header, count, and action buttons.
//! Useful for confirming bulk operations, file deletions, etc.

use owo_colors::{OwoColorize, Style};
use unicode_width::UnicodeWidthStr;

/// Border character set for the preview pane
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

/// Get color style based on style name
fn get_style(style_name: &str) -> Style {
    match style_name.to_lowercase().as_str() {
        "success" => Style::new().bright_green().bold(),
        "warning" => Style::new().bright_yellow().bold(),
        "danger" | "error" => Style::new().bright_red().bold(),
        "info" => Style::new().bright_blue().bold(),
        _ => Style::new().cyan().bold(),
    }
}

/// Preview pane configuration
pub struct PreviewConfig {
    pub title: String,
    pub action: String,
    pub cancel_label: String,
    pub style: String,
    pub border: String,
    pub max_items: usize,
    pub show_numbers: bool,
}

impl Default for PreviewConfig {
    fn default() -> Self {
        Self {
            title: "Preview".to_string(),
            action: "Confirm".to_string(),
            cancel_label: "Cancel".to_string(),
            style: "info".to_string(),
            border: "rounded".to_string(),
            max_items: 20,
            show_numbers: true,
        }
    }
}

/// Render a preview pane with items
pub fn render(items: &[String], config: &PreviewConfig) {
    let borders = BorderChars::get(&config.border);
    let color_style = get_style(&config.style);
    let padding = 2;

    let total_items = items.len();
    let display_items: Vec<&String> = items.iter().take(config.max_items).collect();
    let truncated = total_items > config.max_items;

    // Build header text with count
    let header_text = format!(
        "📋 {}: {} ({})",
        config.title,
        total_items,
        if total_items == 1 { "item" } else { "items" }
    );

    // Calculate max width from items and header
    let max_item_width = display_items
        .iter()
        .enumerate()
        .map(|(i, item)| {
            if config.show_numbers {
                format!("{}. {}", i + 1, item).len()
            } else {
                item.len()
            }
        })
        .max()
        .unwrap_or(0);

    let header_width = UnicodeWidthStr::width(header_text.as_str());
    let action_button = format!("[{}]", config.action);
    let cancel_button = format!("[{}]", config.cancel_label);
    let footer_width = action_button.len() + cancel_button.len() + 3; // space between buttons

    let max_width = max_item_width.max(header_width).max(footer_width);
    let box_width = max_width + (padding * 2);

    // Top border
    println!(
        "{}",
        format!(
            "{}{}{}",
            borders.top_left,
            borders.horizontal.repeat(box_width),
            borders.top_right
        )
        .style(color_style)
    );

    // Header with title and count
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
        .style(color_style)
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
        .style(color_style)
    );

    // Items
    for (i, item) in display_items.iter().enumerate() {
        let content = if config.show_numbers {
            format!("{}. {}", i + 1, item)
        } else {
            (*item).clone()
        };
        let content_width = UnicodeWidthStr::width(content.as_str());
        let total_padding = box_width - content_width;
        let right_padding = total_padding - padding;

        println!(
            "{}{}{}{:>width$}{}",
            borders.vertical.style(color_style),
            " ".repeat(padding),
            content,
            "",
            borders.vertical.style(color_style),
            width = right_padding
        );
    }

    // Show truncation indicator if needed
    if truncated {
        let more_text = format!("... and {} more", total_items - config.max_items);
        let more_width = UnicodeWidthStr::width(more_text.as_str());
        let total_padding = box_width - more_width;
        let right_padding = total_padding - padding;

        println!(
            "{}{}{}{:>width$}{}",
            borders.vertical.style(color_style),
            " ".repeat(padding),
            more_text.dimmed(),
            "",
            borders.vertical.style(color_style),
            width = right_padding
        );
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
        .style(color_style)
    );

    // Action buttons
    let button_style = match config.style.as_str() {
        "danger" | "error" => Style::new().on_bright_red().white().bold(),
        "warning" => Style::new().on_bright_yellow().black().bold(),
        "success" => Style::new().on_bright_green().black().bold(),
        _ => Style::new().on_bright_blue().white().bold(),
    };
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
        borders.vertical.style(color_style),
        " ".repeat(left_button_pad),
        buttons,
        " ".repeat(right_button_pad),
        borders.vertical.style(color_style)
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
        .style(color_style)
    );
}

/// Render preview with column data (for tabular display)
pub fn render_with_columns(items: &[Vec<String>], columns: &[String], config: &PreviewConfig) {
    let borders = BorderChars::get(&config.border);
    let color_style = get_style(&config.style);
    let padding = 2;
    let column_sep = " │ ";

    let total_items = items.len();
    let display_items: Vec<&Vec<String>> = items.iter().take(config.max_items).collect();
    let truncated = total_items > config.max_items;

    // Calculate column widths
    let mut col_widths: Vec<usize> = columns.iter().map(|c| c.len()).collect();
    for row in &display_items {
        for (i, cell) in row.iter().enumerate() {
            if i < col_widths.len() {
                col_widths[i] = col_widths[i].max(cell.len());
            }
        }
    }

    // Calculate total width
    let header_text = format!(
        "📋 {}: {} ({})",
        config.title,
        total_items,
        if total_items == 1 { "item" } else { "items" }
    );
    let header_width = UnicodeWidthStr::width(header_text.as_str());

    let row_content_width: usize =
        col_widths.iter().sum::<usize>() + (columns.len().saturating_sub(1)) * column_sep.len();

    let action_button = format!("[{}]", config.action);
    let cancel_button = format!("[{}]", config.cancel_label);
    let footer_width = action_button.len() + cancel_button.len() + 3;

    let max_width = row_content_width.max(header_width).max(footer_width);
    let box_width = max_width + (padding * 2);

    // Top border
    println!(
        "{}",
        format!(
            "{}{}{}",
            borders.top_left,
            borders.horizontal.repeat(box_width),
            borders.top_right
        )
        .style(color_style)
    );

    // Header
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
        .style(color_style)
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
        .style(color_style)
    );

    // Column headers
    let col_header: String = columns
        .iter()
        .enumerate()
        .map(|(i, c)| {
            format!(
                "{:width$}",
                c,
                width = col_widths.get(i).copied().unwrap_or(0)
            )
        })
        .collect::<Vec<_>>()
        .join(column_sep);
    let col_header_width = UnicodeWidthStr::width(col_header.as_str());
    let col_header_padding = box_width - col_header_width;
    let right_col_pad = col_header_padding - padding;

    println!(
        "{}{}{}{}{}",
        borders.vertical.style(color_style),
        " ".repeat(padding),
        col_header.underline(),
        " ".repeat(right_col_pad),
        borders.vertical.style(color_style)
    );

    // Data rows
    for row in display_items {
        let row_content: String = row
            .iter()
            .enumerate()
            .map(|(i, cell)| {
                format!(
                    "{:width$}",
                    cell,
                    width = col_widths.get(i).copied().unwrap_or(0)
                )
            })
            .collect::<Vec<_>>()
            .join(column_sep);
        let row_width = UnicodeWidthStr::width(row_content.as_str());
        let row_padding = box_width - row_width;
        let right_row_pad = row_padding - padding;

        println!(
            "{}{}{}{}{}",
            borders.vertical.style(color_style),
            " ".repeat(padding),
            row_content,
            " ".repeat(right_row_pad),
            borders.vertical.style(color_style)
        );
    }

    // Truncation indicator
    if truncated {
        let more_text = format!("... and {} more", total_items - config.max_items);
        let more_width = UnicodeWidthStr::width(more_text.as_str());
        let total_padding = box_width - more_width;
        let right_padding = total_padding - padding;

        println!(
            "{}{}{}{}{}",
            borders.vertical.style(color_style),
            " ".repeat(padding),
            more_text.dimmed(),
            " ".repeat(right_padding),
            borders.vertical.style(color_style)
        );
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
        .style(color_style)
    );

    // Action buttons
    let button_style = match config.style.as_str() {
        "danger" | "error" => Style::new().on_bright_red().white().bold(),
        "warning" => Style::new().on_bright_yellow().black().bold(),
        "success" => Style::new().on_bright_green().black().bold(),
        _ => Style::new().on_bright_blue().white().bold(),
    };
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
        borders.vertical.style(color_style),
        " ".repeat(left_button_pad),
        buttons,
        " ".repeat(right_button_pad),
        borders.vertical.style(color_style)
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
        .style(color_style)
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_border_chars() {
        let borders = BorderChars::get("rounded");
        assert_eq!(borders.top_left, "╭");
        assert_eq!(borders.horizontal, "─");
    }

    #[test]
    fn test_default_config() {
        let config = PreviewConfig::default();
        assert_eq!(config.title, "Preview");
        assert_eq!(config.action, "Confirm");
        assert_eq!(config.max_items, 20);
    }

    #[test]
    fn test_render_simple() {
        let items = vec!["file1.txt".to_string(), "file2.txt".to_string()];
        let config = PreviewConfig::default();
        render(&items, &config);
    }

    #[test]
    fn test_render_with_columns() {
        let items = vec![
            vec!["file1.txt".to_string(), "10KB".to_string()],
            vec!["file2.txt".to_string(), "20KB".to_string()],
        ];
        let columns = vec!["Name".to_string(), "Size".to_string()];
        let config = PreviewConfig::default();
        render_with_columns(&items, &columns, &config);
    }

    #[test]
    fn test_render_danger_style() {
        let items = vec!["important.db".to_string()];
        let config = PreviewConfig {
            title: "Files to DELETE".to_string(),
            action: "Delete".to_string(),
            style: "danger".to_string(),
            ..Default::default()
        };
        render(&items, &config);
    }
}
