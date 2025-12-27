use owo_colors::{OwoColorize, Style};
use similar::{ChangeTag, TextDiff};
use std::fs;
use unicode_width::UnicodeWidthStr;

/// Render a side-by-side diff of two files
pub fn render(file1: &str, file2: &str, unified: bool, context: Option<usize>) {
    // Read files
    let content1 = match fs::read_to_string(file1) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("{} {}: {}", "Error reading".bright_red().bold(), file1, e);
            std::process::exit(1);
        }
    };

    let content2 = match fs::read_to_string(file2) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("{} {}: {}", "Error reading".bright_red().bold(), file2, e);
            std::process::exit(1);
        }
    };

    // Create diff
    let diff = TextDiff::from_lines(&content1, &content2);

    if unified {
        render_unified(&diff, file1, file2, context);
    } else {
        render_side_by_side(&diff, file1, file2);
    }
}

/// Render unified diff format
fn render_unified(
    diff: &TextDiff<'_, '_, '_, str>,
    file1: &str,
    file2: &str,
    context: Option<usize>,
) {
    // Header
    println!("{} {}", "───".bright_cyan(), file1.bright_cyan().bold());
    println!("{} {}", "+++".bright_cyan(), file2.bright_cyan().bold());
    println!();

    let context_lines = context.unwrap_or(3);

    for (idx, group) in diff.grouped_ops(context_lines).iter().enumerate() {
        if idx > 0 {
            println!();
        }

        for op in group {
            for change in diff.iter_changes(op) {
                let (sign, style): (&str, Style) = match change.tag() {
                    ChangeTag::Delete => ("-", Style::new().bright_red()),
                    ChangeTag::Insert => ("+", Style::new().bright_green()),
                    ChangeTag::Equal => (" ", Style::new().dimmed()),
                };

                let line_num = change
                    .old_index()
                    .unwrap_or(change.new_index().unwrap_or(0))
                    + 1;
                print!(
                    "{} ",
                    format!("{:>4}", line_num).style(Style::new().dimmed())
                );
                print!("{} ", sign.style(style));
                print!("{}", change.value().style(style));

                if !change.value().ends_with('\n') {
                    println!();
                }
            }
        }
    }
}

/// Render side-by-side diff format
fn render_side_by_side(diff: &TextDiff<'_, '_, '_, str>, file1: &str, file2: &str) {
    // Get terminal width
    let term_width = termion::terminal_size()
        .map(|(w, _)| w as usize)
        .unwrap_or(120);
    let separator = " │ ";
    let line_num_width = 4;
    let available_width = term_width.saturating_sub(separator.len() + (line_num_width * 2) + 4);
    let col_width = available_width / 2;

    // Header
    let header_separator = "─".repeat(term_width);
    println!("{}", header_separator.bright_cyan());

    let left_header = truncate_or_pad(file1, col_width + line_num_width + 1);
    let right_header = truncate_or_pad(file2, col_width + line_num_width + 1);

    println!(
        "{}{}{}",
        left_header.bright_cyan().bold(),
        separator.bright_cyan(),
        right_header.bright_cyan().bold()
    );
    println!("{}", header_separator.bright_cyan());

    // Process changes
    let mut old_line = 1;
    let mut new_line = 1;

    for op in diff.ops() {
        for change in diff.iter_changes(op) {
            match change.tag() {
                ChangeTag::Equal => {
                    let line = change.value().trim_end_matches('\n');
                    let left = format!(
                        "{:>4} {}",
                        old_line.to_string().dimmed(),
                        truncate_or_pad(line, col_width)
                    );
                    let right = format!(
                        "{:>4} {}",
                        new_line.to_string().dimmed(),
                        truncate_or_pad(line, col_width)
                    );
                    println!("{}{}{}", left, separator.dimmed(), right);
                    old_line += 1;
                    new_line += 1;
                }
                ChangeTag::Delete => {
                    let line = change.value().trim_end_matches('\n');
                    let left = format!(
                        "{:>4} {}",
                        old_line.to_string().bright_red(),
                        truncate_or_pad(&format!("- {}", line), col_width).bright_red()
                    );
                    let right = format!(
                        "{:>4} {}",
                        "".dimmed(),
                        truncate_or_pad("", col_width).dimmed()
                    );
                    println!("{}{}{}", left, separator.dimmed(), right);
                    old_line += 1;
                }
                ChangeTag::Insert => {
                    let line = change.value().trim_end_matches('\n');
                    let left = format!(
                        "{:>4} {}",
                        "".dimmed(),
                        truncate_or_pad("", col_width).dimmed()
                    );
                    let right = format!(
                        "{:>4} {}",
                        new_line.to_string().bright_green(),
                        truncate_or_pad(&format!("+ {}", line), col_width).bright_green()
                    );
                    println!("{}{}{}", left, separator.dimmed(), right);
                    new_line += 1;
                }
            }
        }
    }

    println!("{}", header_separator.bright_cyan());
}

/// Truncate or pad string to exact width
fn truncate_or_pad(s: &str, width: usize) -> String {
    let current_width = UnicodeWidthStr::width(s);

    if current_width > width {
        let mut result = String::new();
        let mut current = 0;

        for ch in s.chars() {
            let ch_width = UnicodeWidthStr::width(ch.to_string().as_str());
            if current + ch_width > width.saturating_sub(3) {
                result.push_str("...");
                break;
            }
            result.push(ch);
            current += ch_width;
        }

        result
    } else {
        format!("{}{}", s, " ".repeat(width - current_width))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_truncate_or_pad() {
        assert_eq!(truncate_or_pad("hello", 10), "hello     ");
        assert_eq!(truncate_or_pad("hello world", 8), "hello...");
        assert_eq!(truncate_or_pad("test", 4), "test");
    }
}
