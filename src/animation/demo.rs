use std::io::{stdout, Write};
use std::thread;
use std::time::Duration;
use owo_colors::OwoColorize;
use crossterm::{cursor::{Hide, Show}, ExecutableCommand};

use crate::output::{banner, styled_box, progress};
use crate::charts::{sparkline, bar, pie, pie::PieChart};
use crate::animation::effects;

fn wait(secs: f64) {
    thread::sleep(Duration::from_secs_f64(secs));
}

fn typewriter_print(text: &str, delay_ms: u64) {
    for ch in text.chars() {
        print!("{}", ch);
        stdout().flush().unwrap();
        thread::sleep(Duration::from_millis(delay_ms));
    }
}

fn section_header(title: &str) {
    println!();
    println!("{}", format!("━━━ {} ━━━", title).cyan().bold());
    println!();
}

/// Run the full demo showcase
pub fn run_demo(section: Option<&str>) {
    let mut stdout = stdout();
    stdout.execute(Hide).unwrap();

    match section {
        None => run_full_demo(),
        Some("boxes") => demo_boxes(),
        Some("charts") => demo_charts(),
        Some("progress") => demo_progress(),
        Some("animation") | Some("animations") => demo_animations(),
        Some("all") => run_full_demo(),
        Some(s) => {
            stdout.execute(Show).unwrap();
            eprintln!("Unknown section: {}. Available: boxes, charts, progress, animation, all", s);
            return;
        }
    }

    stdout.execute(Show).unwrap();
}

fn run_full_demo() {
    // Intro
    println!();
    banner::render("termgfx", Some("cyan-purple"));
    println!();
    wait(0.3);

    typewriter_print("  ", 0);
    effects::typewriter("Terminal Graphics Library - Animated Demo", 40.0);
    wait(0.5);

    // Sections
    demo_boxes();
    wait(0.5);

    demo_progress();
    wait(0.5);

    demo_charts();
    wait(0.5);

    demo_animations();
    wait(0.5);

    // Outro
    println!();
    banner::render("Complete!", Some("green-cyan"));
    println!();
    typewriter_print("  ", 0);
    effects::typewriter("Thanks for watching the demo!", 30.0);
    println!();
}

fn demo_boxes() {
    section_header("STYLED BOXES");

    let styles = [
        ("info", "Information message"),
        ("success", "Operation completed!"),
        ("warning", "Please review this"),
        ("danger", "Critical error!"),
    ];

    for (style, msg) in styles {
        print!("  ");
        styled_box::render(msg, style, "rounded", None);
        wait(0.4);
    }
}

fn demo_progress() {
    section_header("PROGRESS BARS");

    // Static examples
    let styles = ["gradient", "blocks", "thin", "classic"];

    for (i, style) in styles.iter().enumerate() {
        let percent = 25 + (i as u8 * 25);
        print!("  {:>8}: ", style.bright_black());
        progress::render(percent, style, None, None);
        wait(0.3);
    }

    println!();
    print!("  ");
    typewriter_print("Animated progress: ", 20);
    println!();
    print!("  ");
    effects::progress(2.0, "gradient");
    wait(0.3);
}

fn demo_charts() {
    section_header("CHARTS");

    // Sparkline
    print!("  ");
    typewriter_print("CPU Usage: ", 20);
    println!();
    print!("  ");
    sparkline::render("20,35,28,45,52,48,60,75,82,68,55,42,38,25,30");
    wait(0.5);

    // Animated sparkline
    println!();
    print!("  ");
    typewriter_print("Building chart: ", 20);
    println!();
    print!("  ");
    effects::chart_build("10,25,15,40,35,50,45,60,55,70,65,80", 1.5);
    wait(0.5);

    // Bar chart
    println!();
    print!("  ");
    typewriter_print("Sales by Quarter:", 20);
    println!();
    bar::render("Q1:120,Q2:150,Q3:180,Q4:220");
    wait(0.5);

    // Pie chart
    println!();
    print!("  ");
    typewriter_print("Market Share:", 20);
    println!();
    let pie_chart = PieChart::new("Chrome:65,Safari:19,Firefox:10,Other:6", false, 500);
    pie_chart.render();
    wait(0.3);
}

fn demo_animations() {
    section_header("ANIMATIONS");

    // Typewriter
    print!("  ");
    typewriter_print("Typewriter effect: ", 20);
    effects::typewriter("Hello, World!", 25.0);
    wait(0.3);

    // Counter
    println!();
    print!("  ");
    typewriter_print("Counter: ", 20);
    effects::counter(0, 100, 1.5, "", "%");
    wait(0.3);

    // Counter with prefix
    println!();
    print!("  ");
    typewriter_print("Revenue: ", 20);
    effects::counter(0, 50000, 2.0, "$", "");
    wait(0.3);

    // Multiple progress bars
    println!();
    print!("  ");
    typewriter_print("Multi-step process:", 20);
    println!();

    let steps = ["Downloading", "Installing", "Configuring", "Verifying"];
    for step in steps {
        print!("    {} ", format!("{}:", step).bright_black());
        effects::progress(0.8, "thin");
        print!("    {} ", "✓".green().bold());
        println!("{}", step.green());
        wait(0.2);
    }
}
