use owo_colors::OwoColorize;

const COLORS: [u8; 8] = [
    196, // Red
    208, // Orange
    226, // Yellow
    46,  // Green
    51,  // Cyan
    21,  // Blue
    129, // Purple
    201, // Magenta
];

pub fn render(data: &str) {
    let entries = parse_data(data);

    if entries.is_empty() {
        eprintln!("Error: No valid data provided");
        return;
    }

    // Find max value for scaling
    let max_value = entries.iter().map(|(_, v)| *v).max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap_or(1.0);

    // Get terminal width, default to 80
    let term_width = crossterm::terminal::size()
        .map(|(w, _)| w as usize)
        .unwrap_or(80);

    // Calculate max label width
    let max_label_width = entries.iter().map(|(label, _)| label.len()).max().unwrap_or(0);

    // Reserve space for label, spacing, value display
    let value_display_width = max_value.to_string().len().max(6); // At least 6 for "100.00"
    let available_width = term_width.saturating_sub(max_label_width + value_display_width + 5);
    let bar_max_width = available_width.max(20); // Minimum 20 chars for bars

    // Render each bar
    for (idx, (label, value)) in entries.iter().enumerate() {
        let color = COLORS[idx % COLORS.len()];
        let bar_width = if max_value > 0.0 {
            ((value / max_value) * bar_max_width as f64).round() as usize
        } else {
            0
        };

        // Create the bar
        let bar = "█".repeat(bar_width);

        // Format value display
        let value_str = if value.fract() == 0.0 {
            format!("{:.0}", value)
        } else {
            format!("{:.2}", value)
        };

        // Print with color: label (aligned), bar, value
        println!(
            "{:<width$}  {}  {}",
            label.truecolor(200, 200, 200),
            bar.color(owo_colors::XtermColors::from(color)),
            value_str.truecolor(150, 150, 150),
            width = max_label_width
        );
    }
}

fn parse_data(data: &str) -> Vec<(String, f64)> {
    data.split(',')
        .filter_map(|entry| {
            let parts: Vec<&str> = entry.trim().split(':').collect();
            if parts.len() != 2 {
                eprintln!("Warning: Invalid entry '{}' (expected Label:Value)", entry);
                return None;
            }

            let label = parts[0].trim().to_string();
            match parts[1].trim().parse::<f64>() {
                Ok(value) if value >= 0.0 => Some((label, value)),
                Ok(_) => {
                    eprintln!("Warning: Negative value for '{}' ignored", label);
                    None
                }
                Err(_) => {
                    eprintln!("Warning: Invalid number '{}' for '{}'", parts[1], label);
                    None
                }
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_data_valid() {
        let data = "Sales:100,Costs:60,Profit:40";
        let result = parse_data(data);
        assert_eq!(result.len(), 3);
        assert_eq!(result[0], ("Sales".to_string(), 100.0));
        assert_eq!(result[1], ("Costs".to_string(), 60.0));
        assert_eq!(result[2], ("Profit".to_string(), 40.0));
    }

    #[test]
    fn test_parse_data_decimals() {
        let data = "A:10.5,B:20.75";
        let result = parse_data(data);
        assert_eq!(result.len(), 2);
        assert_eq!(result[0], ("A".to_string(), 10.5));
        assert_eq!(result[1], ("B".to_string(), 20.75));
    }

    #[test]
    fn test_parse_data_whitespace() {
        let data = " Sales : 100 , Costs : 60 ";
        let result = parse_data(data);
        assert_eq!(result.len(), 2);
        assert_eq!(result[0], ("Sales".to_string(), 100.0));
    }

    #[test]
    fn test_parse_data_invalid_entries() {
        let data = "Sales:100,Invalid,Costs:abc,Profit:40";
        let result = parse_data(data);
        assert_eq!(result.len(), 2); // Only Sales and Profit
        assert_eq!(result[0], ("Sales".to_string(), 100.0));
        assert_eq!(result[1], ("Profit".to_string(), 40.0));
    }

    #[test]
    fn test_parse_data_negative_values() {
        let data = "A:10,B:-5,C:20";
        let result = parse_data(data);
        assert_eq!(result.len(), 2); // B ignored
        assert_eq!(result[0], ("A".to_string(), 10.0));
        assert_eq!(result[1], ("C".to_string(), 20.0));
    }

    #[test]
    fn test_parse_data_empty() {
        let data = "";
        let result = parse_data(data);
        assert_eq!(result.len(), 0);
    }
}
