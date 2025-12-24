use owo_colors::OwoColorize;

pub fn render(percent: u8, style: &str, from: Option<&str>, to: Option<&str>) {
    let percent = percent.min(100);

    // If custom colors provided, use custom gradient
    if from.is_some() || to.is_some() {
        let start = from.map(parse_color).unwrap_or((63, 185, 80));
        let end = to.map(parse_color).unwrap_or((88, 166, 255));
        render_custom_gradient(percent, start, end);
        return;
    }

    match style {
        "blocks" => render_blocks(percent),
        "gradient" => render_gradient(percent),
        "modern" => render_modern(percent),
        "classic" => render_classic(percent),
        "thin" => render_thin(percent),
        "animated" => render_animated(percent),
        _ => render_gradient(percent),
    }
}

fn parse_color(color: &str) -> (u8, u8, u8) {
    // Handle hex colors
    if color.starts_with('#') {
        let hex = color.trim_start_matches('#');
        if hex.len() == 6 {
            let r = u8::from_str_radix(&hex[0..2], 16).unwrap_or(255);
            let g = u8::from_str_radix(&hex[2..4], 16).unwrap_or(255);
            let b = u8::from_str_radix(&hex[4..6], 16).unwrap_or(255);
            return (r, g, b);
        }
    }

    // Handle named colors
    match color.to_lowercase().as_str() {
        "red" => (255, 85, 85),
        "green" => (63, 185, 80),
        "blue" => (88, 166, 255),
        "cyan" => (86, 214, 214),
        "magenta" | "purple" => (187, 154, 247),
        "yellow" => (224, 175, 104),
        "orange" => (255, 149, 0),
        "pink" => (255, 121, 198),
        "white" => (255, 255, 255),
        _ => (255, 255, 255),
    }
}

fn render_custom_gradient(percent: u8, start: (u8, u8, u8), end: (u8, u8, u8)) {
    let width = 30;
    let filled = (width * percent as usize) / 100;
    let empty = width - filled;
    let mut bar = String::new();

    for i in 0..filled {
        let t = i as f32 / width as f32;
        let r = (start.0 as f32 + t * (end.0 as f32 - start.0 as f32)) as u8;
        let g = (start.1 as f32 + t * (end.1 as f32 - start.1 as f32)) as u8;
        let b = (start.2 as f32 + t * (end.2 as f32 - start.2 as f32)) as u8;
        bar.push_str(&format!("\x1b[38;2;{};{};{}m█\x1b[0m", r, g, b));
    }
    for _ in 0..empty {
        bar.push_str("\x1b[38;2;72;79;88m░\x1b[0m");
    }
    let percent_str = format!("\x1b[1m\x1b[38;2;{};{};{}m{}%\x1b[0m", end.0, end.1, end.2, percent);
    println!("{} {}", bar, percent_str);
}

fn render_blocks(percent: u8) {
    let width = 20;
    let filled = (width * percent as usize) / 100;
    let empty = width - filled;
    let mut bar = String::new();
    for _ in 0..filled { bar.push('█'); }
    for _ in 0..empty { bar.push('░'); }
    let percent_str = format!("{}%", percent);
    println!("{} {}", bar.cyan(), percent_str.bright_cyan().bold());
}

fn render_gradient(percent: u8) {
    let width = 20;
    let filled = (width * percent as usize) / 100;
    let empty = width - filled;
    let mut bar = String::new();
    for i in 0..filled {
        let progress = (i as f32 / width as f32) * 100.0;
        let char = if progress < 33.0 { '█'.red().to_string() }
        else if progress < 66.0 { '█'.yellow().to_string() }
        else { '█'.green().to_string() };
        bar.push_str(&char);
    }
    for _ in 0..empty { bar.push_str(&"░".bright_black().to_string()); }
    let percent_display = format!("{}%", percent);
    let percent_colored = if percent < 33 { percent_display.red().to_string() }
    else if percent < 66 { percent_display.yellow().to_string() }
    else { percent_display.green().to_string() };
    println!("{} {}", bar, percent_colored.bold());
}

fn render_classic(percent: u8) {
    let width = 20;
    let filled = (width * percent as usize) / 100;
    let empty = width.saturating_sub(filled + 1);
    let mut bar = String::new();
    bar.push('[');
    for _ in 0..filled { bar.push_str(&"=".cyan().to_string()); }
    if filled < width { bar.push_str(&">".bright_cyan().to_string()); }
    for _ in 0..empty { bar.push(' '); }
    bar.push(']');
    let percent_str = format!("{}%", percent);
    println!("{} {}", bar, percent_str.bright_cyan().bold());
}

fn render_thin(percent: u8) {
    let width = 20;
    let filled = (width * percent as usize) / 100;
    let empty = width - filled;
    let mut bar = String::new();
    for _ in 0..filled { bar.push_str(&"━".cyan().to_string()); }
    for _ in 0..empty { bar.push_str(&"━".bright_black().to_string()); }
    let percent_str = format!("{}%", percent);
    println!("{} {}", bar, percent_str.bright_cyan().bold());
}

fn render_animated(percent: u8) {
    let width = 30;
    let filled = (width * percent as usize) / 100;
    let empty = width - filled;
    let mut bar = String::new();
    for i in 0..filled {
        let char = match i % 4 { 0 => '█', 1 => '▓', 2 => '▒', 3 => '░', _ => '█' };
        bar.push_str(&char.cyan().to_string());
    }
    for _ in 0..empty { bar.push(' '); }
    let percent_str = format!("{}%", percent);
    println!("{} {}", bar, percent_str.bright_cyan().bold());
}

fn render_modern(percent: u8) {
    let width = 30;
    let filled = (width * percent as usize) / 100;
    let empty = width - filled;
    let mut bar = String::new();

    // Smooth RGB gradient: green (#3fb950) → cyan (#58a6ff)
    // Start: (63, 185, 80)  End: (88, 166, 255)
    for i in 0..filled {
        let t = i as f32 / width as f32;
        let r = (63.0 + t * (88.0 - 63.0)) as u8;
        let g = (185.0 + t * (166.0 - 185.0)) as u8;
        let b = (80.0 + t * (255.0 - 80.0)) as u8;
        bar.push_str(&format!("\x1b[38;2;{};{};{}m█\x1b[0m", r, g, b));
    }
    for _ in 0..empty {
        bar.push_str("\x1b[38;2;72;79;88m░\x1b[0m");
    }
    let percent_str = format!("\x1b[1m\x1b[38;2;88;166;255m{}%\x1b[0m", percent);
    println!("{} {}", bar, percent_str);
}
