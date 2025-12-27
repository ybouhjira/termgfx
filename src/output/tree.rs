use owo_colors::OwoColorize;
use serde_json::Value;
use std::io::Write;
use std::thread;
use std::time::Duration;

/// Tree characters for drawing hierarchical structures
#[derive(Debug, Clone)]
struct TreeChars {
    branch: &'static str,   // ├──
    last: &'static str,     // └──
    vertical: &'static str, // │
    space: &'static str,    // "   "
}

impl TreeChars {
    fn unicode() -> Self {
        TreeChars {
            branch: "├── ",
            last: "└── ",
            vertical: "│   ",
            space: "    ",
        }
    }
}

/// Get color for depth level (cycling through rainbow colors)
fn get_depth_color(depth: usize) -> owo_colors::Style {
    let colors = [
        owo_colors::Style::new().cyan(),
        owo_colors::Style::new().green(),
        owo_colors::Style::new().yellow(),
        owo_colors::Style::new().magenta(),
        owo_colors::Style::new().blue(),
        owo_colors::Style::new().bright_cyan(),
    ];
    colors[depth % colors.len()]
}

/// Render a tree structure from JSON data
fn render_json_tree(value: &Value, prefix: &str, _is_last: bool, depth: usize, chars: &TreeChars) {
    let color = get_depth_color(depth);

    match value {
        Value::Object(map) => {
            let entries: Vec<_> = map.iter().collect();
            for (i, (key, val)) in entries.iter().enumerate() {
                let is_last_item = i == entries.len() - 1;
                let connector = if is_last_item {
                    chars.last
                } else {
                    chars.branch
                };

                // Icon based on value type
                let icon = match val {
                    Value::Object(_) => "📁",
                    Value::Null => "📄",
                    Value::Array(_) => "📦",
                    _ => "📌",
                };

                println!(
                    "{}{}{}{}",
                    prefix.style(color),
                    connector.style(color),
                    icon,
                    format!(" {}", key).style(color).bold()
                );

                // Recurse for nested objects
                if let Value::Object(_) = val {
                    let extension = if is_last_item {
                        chars.space
                    } else {
                        chars.vertical
                    };
                    let new_prefix = format!("{}{}", prefix, extension);
                    render_json_tree(val, &new_prefix, is_last_item, depth + 1, chars);
                }
            }
        }
        Value::Array(arr) => {
            for (i, item) in arr.iter().enumerate() {
                let is_last_item = i == arr.len() - 1;
                let connector = if is_last_item {
                    chars.last
                } else {
                    chars.branch
                };

                println!(
                    "{}{}📌 {}",
                    prefix.style(color),
                    connector.style(color),
                    format!("[{}]", i).style(color)
                );

                let extension = if is_last_item {
                    chars.space
                } else {
                    chars.vertical
                };
                let new_prefix = format!("{}{}", prefix, extension);
                render_json_tree(item, &new_prefix, is_last_item, depth + 1, chars);
            }
        }
        _ => {}
    }
}

/// Render a tree from inline data format: "root>child1,child2>grandchild"
#[allow(dead_code)]
fn render_inline_tree(data: &str) {
    render_inline_tree_animated(data, false, 500);
}

/// Render a tree from inline data with optional animation
/// animation_time_ms: total animation duration in milliseconds (delay is calculated per node)
fn render_inline_tree_animated(data: &str, animate: bool, animation_time_ms: u64) {
    let chars = TreeChars::unicode();
    let parts: Vec<&str> = data.split('>').collect();

    if parts.is_empty() {
        return;
    }

    // Count total nodes: root + all children across all levels
    let total_nodes: usize = 1 + parts
        .iter()
        .skip(1)
        .map(|p| p.split(',').count())
        .sum::<usize>();
    let delay = if animate && total_nodes > 0 {
        Duration::from_millis(animation_time_ms / total_nodes as u64)
    } else {
        Duration::ZERO
    };
    let mut stdout = std::io::stdout();

    println!("{} {}", "📁".bright_cyan(), parts[0].bright_cyan().bold());
    if animate {
        stdout.flush().unwrap();
        thread::sleep(delay);
    }

    for (i, part) in parts.iter().enumerate().skip(1) {
        let children: Vec<&str> = part.split(',').collect();
        let depth = i;
        let color = get_depth_color(depth);

        for child in children {
            let mut prefix = String::new();
            for _ in 0..depth - 1 {
                prefix.push_str(chars.space);
            }

            println!(
                "{}{}📄 {}",
                prefix.style(color),
                chars.branch.style(color),
                child.style(color)
            );

            if animate {
                stdout.flush().unwrap();
                thread::sleep(delay);
            }
        }
    }
}

/// Main render function - handles all tree types
#[allow(dead_code)]
pub fn render(data: Option<&str>, path: Option<&str>) {
    render_animated(data, path, false, 500);
}

/// Render tree with optional animation
/// animation_time_ms: total animation duration in milliseconds (delay is calculated per node)
pub fn render_animated(
    data: Option<&str>,
    path: Option<&str>,
    animate: bool,
    animation_time_ms: u64,
) {
    let chars = TreeChars::unicode();

    if let Some(_p) = path {
        eprintln!(
            "{} Directory tree requires 'walkdir' feature. Use JSON input instead.",
            "Warning:".bright_yellow().bold()
        );
        std::process::exit(1);
    } else if let Some(d) = data {
        render_inline_tree_animated(d, animate, animation_time_ms);
    } else {
        use std::io::{self, Read};
        let mut buffer = String::new();

        if let Err(e) = io::stdin().read_to_string(&mut buffer) {
            eprintln!(
                "{} Failed to read stdin: {}",
                "Error:".bright_red().bold(),
                e
            );
            std::process::exit(1);
        }

        match serde_json::from_str::<Value>(buffer.trim()) {
            Ok(json) => {
                println!("{} {}", "📁".bright_cyan(), "root".bright_cyan().bold());
                render_json_tree(&json, "", true, 0, &chars);
            }
            Err(e) => {
                eprintln!("{} Invalid JSON: {}", "Error:".bright_red().bold(), e);
                std::process::exit(1);
            }
        }
    }
}
