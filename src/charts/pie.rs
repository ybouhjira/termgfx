use owo_colors::OwoColorize;
use std::f64::consts::PI;

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

const BLOCKS: [&str; 8] = ["█", "▓", "▒", "░", "▄", "▀", "▐", "▌"];

pub fn render(data: &str) {
    let entries = parse_data(data);

    if entries.is_empty() {
        eprintln!("Error: No valid data provided");
        return;
    }

    // Calculate total and percentages
    let total: f64 = entries.iter().map(|(_, v)| v).sum();
    if total <= 0.0 {
        eprintln!("Error: Total value must be positive");
        return;
    }

    let segments: Vec<(String, f64, f64)> = entries
        .iter()
        .map(|(label, value)| {
            let percentage = (value / total) * 100.0;
            (label.clone(), *value, percentage)
        })
        .collect();

    // Render the pie chart
    render_circle(&segments);

    // Render legend
    println!();
    for (idx, (label, _, percentage)) in segments.iter().enumerate() {
        let block = BLOCKS[idx % BLOCKS.len()];
        let color = COLORS[idx % COLORS.len()];
        println!(
            "  {} {}: {:.1}%",
            block.repeat(2).color(owo_colors::XtermColors::from(color)),
            label,
            percentage
        );
    }
}

fn render_circle(segments: &[(String, f64, f64)]) {
    let radius = 9.0;
    let center_x = 10.0;
    let center_y = 9.0;

    // Calculate cumulative angles for each segment
    let mut cumulative_angle = 0.0;
    let mut segment_angles: Vec<(f64, f64, usize)> = Vec::new();

    for (idx, (_, _, percentage)) in segments.iter().enumerate() {
        let angle_span = (percentage / 100.0) * 2.0 * PI;
        segment_angles.push((cumulative_angle, cumulative_angle + angle_span, idx));
        cumulative_angle += angle_span;
    }

    // Render the circle grid
    for y in 0..18 {
        for x in 0..20 {
            let dx = x as f64 - center_x;
            let dy = (y as f64 - center_y) * 2.0; // Adjust for character aspect ratio
            let distance = (dx * dx + dy * dy).sqrt();

            if distance <= radius {
                // Calculate angle from center
                let angle = dy.atan2(dx) + PI; // Normalize to 0..2π

                // Find which segment this point belongs to
                let segment_idx = segment_angles
                    .iter()
                    .find(|(start, end, _)| angle >= *start && angle < *end)
                    .map(|(_, _, idx)| *idx)
                    .unwrap_or(segment_angles.last().unwrap().2);

                let block = BLOCKS[segment_idx % BLOCKS.len()];
                let color = COLORS[segment_idx % COLORS.len()];
                print!("{}", block.color(owo_colors::XtermColors::from(color)));
            } else {
                print!(" ");
            }
        }
        println!();
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
        let data = "A:30,B:50,C:20";
        let result = parse_data(data);
        assert_eq!(result.len(), 3);
        assert_eq!(result[0], ("A".to_string(), 30.0));
        assert_eq!(result[1], ("B".to_string(), 50.0));
        assert_eq!(result[2], ("C".to_string(), 20.0));
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
        let data = " A : 30 , B : 50 ";
        let result = parse_data(data);
        assert_eq!(result.len(), 2);
        assert_eq!(result[0], ("A".to_string(), 30.0));
    }

    #[test]
    fn test_parse_data_invalid_entries() {
        let data = "A:30,Invalid,B:abc,C:20";
        let result = parse_data(data);
        assert_eq!(result.len(), 2); // Only A and C
        assert_eq!(result[0], ("A".to_string(), 30.0));
        assert_eq!(result[1], ("C".to_string(), 20.0));
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
