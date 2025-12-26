use std::io::{self, Read};
use unicode_width::UnicodeWidthStr;

/// Strip ANSI escape codes to calculate actual display width
fn strip_ansi(text: &str) -> String {
    let mut result = String::new();
    let mut chars = text.chars().peekable();

    while let Some(c) = chars.next() {
        if c == '\x1b' {
            // Skip escape sequence
            if chars.peek() == Some(&'[') {
                chars.next();
                while let Some(&c) = chars.peek() {
                    chars.next();
                    if c.is_alphabetic() {
                        break;
                    }
                }
            }
        } else {
            result.push(c);
        }
    }
    result
}

/// Calculate the actual display width of a string (without ANSI codes)
fn display_width(text: &str) -> usize {
    UnicodeWidthStr::width(strip_ansi(text).as_str())
}

/// Read content from stdin
fn read_stdin() -> io::Result<String> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;
    Ok(buffer)
}

/// Join content horizontally or vertically
pub fn join(
    inputs: Vec<String>,
    vertical: bool,
    gap: usize,
    align: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    if inputs.is_empty() {
        return Err("No input provided".into());
    }

    if vertical {
        // Vertical join: stack with gaps
        for (i, content) in inputs.iter().enumerate() {
            print!("{}", content);
            if i < inputs.len() - 1 {
                for _ in 0..gap {
                    println!();
                }
            }
        }
        println!();
    } else {
        // Horizontal join: side by side
        let contents: Vec<Vec<&str>> = inputs.iter().map(|s| s.lines().collect()).collect();
        let max_lines = contents.iter().map(|c| c.len()).max().unwrap_or(0);
        let widths: Vec<usize> = contents
            .iter()
            .map(|lines| {
                lines
                    .iter()
                    .map(|line| display_width(line))
                    .max()
                    .unwrap_or(0)
            })
            .collect();

        for line_idx in 0..max_lines {
            for (col_idx, content) in contents.iter().enumerate() {
                let line = content.get(line_idx).unwrap_or(&"");
                let width = widths[col_idx];
                let line_width = display_width(line);
                let padding = width.saturating_sub(line_width);

                // Apply alignment
                let output = match align {
                    "right" => format!("{:>width$}{}", "", line, width = padding),
                    "center" => {
                        let left_pad = padding / 2;
                        let right_pad = padding - left_pad;
                        format!("{}{}{}", " ".repeat(left_pad), line, " ".repeat(right_pad))
                    }
                    _ => format!("{}{}", line, " ".repeat(padding)), // left (default)
                };

                print!("{}", output);

                // Add gap between columns
                if col_idx < contents.len() - 1 {
                    print!("{}", " ".repeat(gap));
                }
            }
            println!();
        }
    }

    Ok(())
}

/// Split content into columns with specified widths
pub fn columns(
    content: &str,
    widths: Vec<usize>,
    gap: usize,
) -> Result<(), Box<dyn std::error::Error>> {
    let lines: Vec<&str> = content.lines().collect();

    if widths.is_empty() {
        return Err("No column widths specified".into());
    }

    let num_cols = widths.len();
    let total_lines = lines.len();
    let lines_per_col = (total_lines + num_cols - 1) / num_cols; // Ceiling division

    let mut columns: Vec<Vec<&str>> = vec![Vec::new(); num_cols];
    for (i, line) in lines.iter().enumerate() {
        let col_idx = i / lines_per_col;
        if col_idx < num_cols {
            columns[col_idx].push(line);
        }
    }

    // Print columns side by side
    let max_rows = columns.iter().map(|c| c.len()).max().unwrap_or(0);
    for row_idx in 0..max_rows {
        for (col_idx, column) in columns.iter().enumerate() {
            let width = widths[col_idx];
            let line = column.get(row_idx).unwrap_or(&"");
            let line_width = display_width(line);

            // Truncate if too wide, pad if too narrow
            if line_width > width {
                let stripped = strip_ansi(line);
                let truncated = stripped
                    .chars()
                    .take(width.saturating_sub(1))
                    .collect::<String>();
                print!("{}…", truncated);
            } else {
                let padding = width.saturating_sub(line_width);
                print!("{}{}", line, " ".repeat(padding));
            }

            // Add gap between columns
            if col_idx < columns.len() - 1 {
                print!("{}", " ".repeat(gap));
            }
        }
        println!();
    }

    Ok(())
}

/// Stack content vertically with alignment
pub fn stack(
    inputs: Vec<String>,
    align: &str,
    gap: usize,
) -> Result<(), Box<dyn std::error::Error>> {
    if inputs.is_empty() {
        return Err("No input provided".into());
    }

    // Find max width across all inputs
    let max_width = inputs
        .iter()
        .flat_map(|s| s.lines())
        .map(|line| display_width(line))
        .max()
        .unwrap_or(0);

    for (i, content) in inputs.iter().enumerate() {
        for line in content.lines() {
            let line_width = display_width(line);
            let padding = max_width.saturating_sub(line_width);

            // Apply alignment
            match align {
                "right" => println!("{:>width$}{}", "", line, width = padding),
                "center" => {
                    let left_pad = padding / 2;
                    println!("{}{}", " ".repeat(left_pad), line)
                }
                _ => println!("{}", line), // left (default)
            }
        }

        // Add gap between stacked items
        if i < inputs.len() - 1 {
            for _ in 0..gap {
                println!();
            }
        }
    }

    Ok(())
}

/// Handle join command from CLI
pub fn handle_join(
    inputs: Vec<String>,
    stdin: bool,
    vertical: bool,
    gap: usize,
    align: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut contents = inputs;

    if stdin {
        let stdin_content = read_stdin()?;
        contents.push(stdin_content);
    }

    if contents.is_empty() {
        return Err("No input provided (use arguments or pipe content to stdin)".into());
    }

    join(contents, vertical, gap, align)
}

/// Handle columns command from CLI
pub fn handle_columns(widths: Vec<usize>, gap: usize) -> Result<(), Box<dyn std::error::Error>> {
    let content = read_stdin()?;
    columns(&content, widths, gap)
}

/// Handle stack command from CLI
pub fn handle_stack(
    inputs: Vec<String>,
    stdin: bool,
    align: &str,
    gap: usize,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut contents = inputs;

    if stdin {
        let stdin_content = read_stdin()?;
        contents.push(stdin_content);
    }

    if contents.is_empty() {
        return Err("No input provided (use arguments or pipe content to stdin)".into());
    }

    stack(contents, align, gap)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_strip_ansi() {
        let colored = "\x1b[31mRed\x1b[0m";
        assert_eq!(strip_ansi(colored), "Red");
    }

    #[test]
    fn test_display_width() {
        assert_eq!(display_width("Hello"), 5);
        assert_eq!(display_width("\x1b[31mRed\x1b[0m"), 3);
    }

    #[test]
    fn test_join_horizontal() {
        let inputs = vec!["A\nB".to_string(), "1\n2".to_string()];
        let result = join(inputs, false, 2, "left");
        assert!(result.is_ok());
    }

    #[test]
    fn test_join_vertical() {
        let inputs = vec!["A".to_string(), "B".to_string()];
        let result = join(inputs, true, 1, "left");
        assert!(result.is_ok());
    }

    #[test]
    fn test_stack() {
        let inputs = vec!["Short".to_string(), "Longer line".to_string()];
        let result = stack(inputs, "center", 1);
        assert!(result.is_ok());
    }

    #[test]
    fn test_columns() {
        let content = "Line1\nLine2\nLine3\nLine4\nLine5\nLine6";
        let result = columns(content, vec![10, 10, 10], 2);
        assert!(result.is_ok());
    }
}
