use owo_colors::OwoColorize;
use serde::{Deserialize, Serialize};
use std::io::{self, Write};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatItem {
    pub label: String,
    pub value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatsOutput {
    pub items: Vec<StatItem>,
}

/// Separator styles for stats bar
#[derive(Debug, Clone, Copy)]
pub enum Separator {
    Pipe,    // │
    Dot,     // •
    Slash,   // /
    Bar,     // |
    Diamond, // ◆
    Arrow,   // →
}

impl Separator {
    fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "dot" | "•" => Separator::Dot,
            "slash" | "/" => Separator::Slash,
            "bar" | "|" => Separator::Bar,
            "diamond" | "◆" => Separator::Diamond,
            "arrow" | "→" => Separator::Arrow,
            _ => Separator::Pipe,
        }
    }

    fn char(&self) -> &'static str {
        match self {
            Separator::Pipe => "│",
            Separator::Dot => "•",
            Separator::Slash => "/",
            Separator::Bar => "|",
            Separator::Diamond => "◆",
            Separator::Arrow => "→",
        }
    }
}

/// Parse items from format: "label:value,label:value" or "label:value" "label:value"
fn parse_items(items_str: &str) -> Vec<StatItem> {
    items_str
        .split(',')
        .filter(|s| !s.trim().is_empty())
        .map(|item| {
            let parts: Vec<&str> = item.splitn(2, ':').collect();
            let label = parts.first().unwrap_or(&"").trim().to_string();
            let value = parts.get(1).unwrap_or(&"").trim().to_string();
            StatItem { label, value }
        })
        .collect()
}

/// Parse items from multiple string arguments
fn parse_items_from_args(items: &[String]) -> Vec<StatItem> {
    items
        .iter()
        .filter(|s| !s.trim().is_empty())
        .map(|item| {
            let parts: Vec<&str> = item.splitn(2, ':').collect();
            let label = parts.first().unwrap_or(&"").trim().to_string();
            let value = parts.get(1).unwrap_or(&"").trim().to_string();
            StatItem { label, value }
        })
        .collect()
}

/// Detect value type and apply appropriate coloring
fn colorize_value(value: &str) -> String {
    let value_lower = value.to_lowercase();

    // Time/duration patterns
    if value.ends_with("ago")
        || value.ends_with("s")
        || value.ends_with("m")
        || value.ends_with("h")
        || value.ends_with("d")
    {
        return value.cyan().to_string();
    }

    // Size patterns (MB, GB, KB, etc.)
    if value_lower.ends_with("mb")
        || value_lower.ends_with("gb")
        || value_lower.ends_with("kb")
        || value_lower.ends_with("tb")
        || value_lower.ends_with("bytes")
    {
        return value.yellow().to_string();
    }

    // Percentage patterns
    if value.ends_with('%') {
        let num_str = value.trim_end_matches('%');
        if let Ok(num) = num_str.parse::<f64>() {
            return if num >= 90.0 {
                value.red().to_string()
            } else if num >= 70.0 {
                value.yellow().to_string()
            } else {
                value.green().to_string()
            };
        }
    }

    // Status keywords
    match value_lower.as_str() {
        "ok" | "good" | "online" | "healthy" | "success" | "active" => {
            return value.green().to_string()
        }
        "error" | "fail" | "failed" | "offline" | "unhealthy" | "critical" => {
            return value.red().to_string()
        }
        "warning" | "warn" | "degraded" | "slow" => return value.yellow().to_string(),
        _ => {}
    }

    // Numbers
    if value.parse::<f64>().is_ok() || value.replace(',', "").parse::<f64>().is_ok() {
        return value.bright_white().bold().to_string();
    }

    // Default
    value.white().to_string()
}

/// Render the stats bar as JSON
fn render_json(items: &[StatItem]) {
    let output = StatsOutput {
        items: items.to_vec(),
    };

    match serde_json::to_string_pretty(&output) {
        Ok(json) => println!("{}", json),
        Err(e) => eprintln!("Error serializing to JSON: {}", e),
    }
}

/// Main render function
pub fn render(
    items_str: Option<&str>,
    items_args: &[String],
    separator: &str,
    emoji: Option<&str>,
    output_json: bool,
    no_color: bool,
) {
    // Parse items from either comma-separated string or multiple arguments
    let items = if let Some(items_str) = items_str {
        parse_items(items_str)
    } else if !items_args.is_empty() {
        parse_items_from_args(items_args)
    } else {
        eprintln!("No items to display");
        return;
    };

    if items.is_empty() {
        eprintln!("No items to display");
        return;
    }

    if output_json {
        render_json(&items);
        return;
    }

    let sep = Separator::from_str(separator);
    let sep_char = sep.char();

    let mut output = String::new();

    // Add emoji prefix if provided
    if let Some(emoji) = emoji {
        output.push_str(emoji);
        output.push(' ');
    }

    // Build the stats line
    for (i, item) in items.iter().enumerate() {
        if i > 0 {
            output.push_str(&format!(" {} ", sep_char.dimmed()));
        }

        let value_str = if no_color {
            item.value.clone()
        } else {
            colorize_value(&item.value)
        };

        if item.label.is_empty() {
            output.push_str(&value_str);
        } else {
            output.push_str(&format!("{} {}", item.label.dimmed(), value_str));
        }
    }

    println!("{}", output);
    io::stdout().flush().ok();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_items_basic() {
        let items = parse_items("entries:500,size:2.3 MB,modified:2h ago");
        assert_eq!(items.len(), 3);
        assert_eq!(items[0].label, "entries");
        assert_eq!(items[0].value, "500");
        assert_eq!(items[1].label, "size");
        assert_eq!(items[1].value, "2.3 MB");
    }

    #[test]
    fn test_parse_items_from_args() {
        let args = vec![
            "Files:1,234".to_string(),
            "Size:45.2 MB".to_string(),
            "Last sync:5m ago".to_string(),
        ];
        let items = parse_items_from_args(&args);
        assert_eq!(items.len(), 3);
        assert_eq!(items[0].label, "Files");
        assert_eq!(items[0].value, "1,234");
    }

    #[test]
    fn test_separator_from_str() {
        assert!(matches!(Separator::from_str("pipe"), Separator::Pipe));
        assert!(matches!(Separator::from_str("dot"), Separator::Dot));
        assert!(matches!(Separator::from_str("slash"), Separator::Slash));
        assert!(matches!(Separator::from_str("diamond"), Separator::Diamond));
    }

    #[test]
    fn test_separator_chars() {
        assert_eq!(Separator::Pipe.char(), "│");
        assert_eq!(Separator::Dot.char(), "•");
        assert_eq!(Separator::Slash.char(), "/");
    }
}
