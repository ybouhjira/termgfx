use owo_colors::OwoColorize;

const COLORS: [u8; 8] = [196, 208, 226, 46, 51, 21, 129, 201];

pub fn render(data: &str) {
    let entries = parse_data(data);
    if entries.is_empty() {
        eprintln!("Error: No valid data provided");
        return;
    }
    let max_value = entries.iter().map(|(_, v)| *v).max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap_or(1.0);
    let term_width = crossterm::terminal::size().map(|(w, _)| w as usize).unwrap_or(80);
    let max_label_width = entries.iter().map(|(label, _)| label.len()).max().unwrap_or(0);
    let value_display_width = max_value.to_string().len().max(6);
    let available_width = term_width.saturating_sub(max_label_width + value_display_width + 5);
    let bar_max_width = available_width.max(20);
    for (idx, (label, value)) in entries.iter().enumerate() {
        let color = COLORS[idx % COLORS.len()];
        let bar_width = if max_value > 0.0 { ((value / max_value) * bar_max_width as f64).round() as usize } else { 0 };
        let bar = "█".repeat(bar_width);
        let value_str = if value.fract() == 0.0 { format!("{:.0}", value) } else { format!("{:.2}", value) };
        println!("{:<width$}  {}  {}", label.truecolor(200, 200, 200), bar.color(owo_colors::XtermColors::from(color)), value_str.truecolor(150, 150, 150), width = max_label_width);
    }
}

fn parse_data(data: &str) -> Vec<(String, f64)> {
    data.split(',').filter_map(|entry| {
        let parts: Vec<&str> = entry.trim().split(':').collect();
        if parts.len() != 2 { eprintln!("Warning: Invalid entry '{}'", entry); return None; }
        let label = parts[0].trim().to_string();
        match parts[1].trim().parse::<f64>() {
            Ok(value) if value >= 0.0 => Some((label, value)),
            Ok(_) => { eprintln!("Warning: Negative value for '{}'", label); None }
            Err(_) => { eprintln!("Warning: Invalid number '{}'", parts[1]); None }
        }
    }).collect()
}
