use owo_colors::OwoColorize;

pub fn render(percent: u8, style: &str) {
    let percent = percent.min(100);
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

    // Green to cyan to blue gradient
    for i in 0..filled {
        let progress = i as f32 / width as f32;
        let char = if progress < 0.5 {
            '█'.green().to_string()
        } else {
            '█'.cyan().to_string()
        };
        bar.push_str(&char);
    }
    for _ in 0..empty {
        bar.push_str(&"░".bright_black().to_string());
    }
    let percent_str = format!("{}%", percent);
    println!("{} {}", bar, percent_str.bright_cyan().bold());
}
