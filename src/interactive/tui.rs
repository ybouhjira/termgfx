use crossterm::{
    cursor::{Hide, Show, MoveTo},
    event::{self, Event, KeyCode, KeyModifiers},
    execute,
    terminal::{self, Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen},
};
use serde::{Deserialize, Serialize};
use std::fs;
use std::io::{self, Write};
use std::time::Duration;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TuiConfig {
    pub layout: String,
    pub widgets: Vec<WidgetConfig>,
    #[serde(default = "default_refresh")]
    pub refresh_interval: u64,
}

fn default_refresh() -> u64 {
    1000
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WidgetConfig {
    #[serde(rename = "type")]
    pub widget_type: String,
    pub content: String,
    #[serde(default)]
    pub title: Option<String>,
}

#[derive(Debug)]
struct Layout {
    rows: usize,
    cols: usize,
}

impl Layout {
    fn parse(layout_str: &str) -> Result<Self, String> {
        let parts: Vec<&str> = layout_str.split('x').collect();
        if parts.len() != 2 {
            return Err("Invalid layout format (use NxM, e.g., 2x2)".to_string());
        }
        let rows = parts[0]
            .parse::<usize>()
            .map_err(|_| "Invalid layout format (use NxM, e.g., 2x2)".to_string())?;
        let cols = parts[1]
            .parse::<usize>()
            .map_err(|_| "Invalid layout format (use NxM, e.g., 2x2)".to_string())?;
        Ok(Layout { rows, cols })
    }

    fn cell_count(&self) -> usize {
        self.rows * self.cols
    }
}

fn parse_inline_widgets(widgets_str: &str) -> Result<Vec<WidgetConfig>, String> {
    let mut widgets = Vec::new();
    for widget_def in widgets_str.split(',') {
        let widget_def = widget_def.trim();
        if widget_def.is_empty() {
            continue;
        }
        let parts: Vec<&str> = widget_def.splitn(2, ':').collect();
        if parts.len() < 2 {
            return Err(format!("Invalid widget definition: '{}' (use type:content)", widget_def));
        }
        widgets.push(WidgetConfig {
            widget_type: parts[0].to_string(),
            content: parts[1].to_string(),
            title: None,
        });
    }
    Ok(widgets)
}

fn render_box_widget(content: &str, width: usize, height: usize) -> Vec<String> {
    let mut lines = Vec::new();
    let inner_width = width.saturating_sub(2);
    let inner_height = height.saturating_sub(2);

    // Top border
    lines.push(format!("\x1b[36m{}{}{}\x1b[0m", "┌", "─".repeat(inner_width), "┐"));

    // Content lines
    let content_lines: Vec<&str> = content.lines().collect();
    for i in 0..inner_height {
        let text = content_lines.get(i).copied().unwrap_or("");
        let display_width = text.chars().count().min(inner_width);
        let padding = inner_width.saturating_sub(display_width);
        let truncated: String = text.chars().take(inner_width).collect();
        lines.push(format!(
            "\x1b[36m│\x1b[0m{}{}\x1b[36m│\x1b[0m",
            truncated,
            " ".repeat(padding)
        ));
    }

    // Bottom border
    lines.push(format!("\x1b[36m{}{}{}\x1b[0m", "└", "─".repeat(inner_width), "┘"));

    lines
}

fn render_gauge_widget(content: &str, width: usize, height: usize) -> Vec<String> {
    let mut lines = Vec::new();
    let inner_width = width.saturating_sub(4);

    let value: f64 = content.parse().unwrap_or(0.0);
    let percent = (value / 100.0).clamp(0.0, 1.0);
    let filled = (inner_width as f64 * percent) as usize;
    let empty = inner_width.saturating_sub(filled);

    // Title
    lines.push(format!("\x1b[33m Gauge: {:.0}% \x1b[0m", value));

    // Gauge bar
    let bar = format!(
        "  \x1b[42m{}\x1b[0m\x1b[100m{}\x1b[0m",
        " ".repeat(filled),
        " ".repeat(empty)
    );
    lines.push(bar);

    // Pad remaining height
    for _ in 2..height {
        lines.push(" ".repeat(width));
    }

    lines
}

fn render_sparkline_widget(content: &str, width: usize, height: usize) -> Vec<String> {
    let mut lines = Vec::new();
    let blocks = ['▁', '▂', '▃', '▄', '▅', '▆', '▇', '█'];

    // Parse values (support both , and ; as delimiters)
    let values: Vec<f64> = content
        .split(|c| c == ',' || c == ';')
        .filter_map(|s| s.trim().parse().ok())
        .collect();

    if values.is_empty() {
        lines.push(" No data ".to_string());
        for _ in 1..height {
            lines.push(" ".repeat(width));
        }
        return lines;
    }

    let max_val = values.iter().cloned().fold(f64::MIN, f64::max);
    let min_val = values.iter().cloned().fold(f64::MAX, f64::min);
    let range = (max_val - min_val).max(0.001);

    // Title
    lines.push(format!("\x1b[35m Sparkline \x1b[0m"));

    // Build sparkline
    let mut spark = String::new();
    let display_count = values.len().min(width.saturating_sub(2));
    for val in values.iter().take(display_count) {
        let normalized = ((val - min_val) / range * 7.0) as usize;
        let idx = normalized.min(7);
        spark.push_str(&format!("\x1b[35m{}\x1b[0m", blocks[idx]));
    }
    let padding = width.saturating_sub(spark.chars().count() / 10); // Rough estimate due to ANSI codes
    lines.push(format!(" {}", spark));

    // Pad remaining height
    for _ in 2..height {
        lines.push(" ".repeat(width));
    }

    lines
}

fn render_log_widget(content: &str, width: usize, height: usize) -> Vec<String> {
    let mut lines = Vec::new();
    let inner_width = width.saturating_sub(2);

    // Title
    lines.push(format!("\x1b[32m Log \x1b[0m"));

    // Log lines
    let content_lines: Vec<&str> = content.lines().collect();
    let max_lines = height.saturating_sub(1);
    let start = content_lines.len().saturating_sub(max_lines);

    for line in content_lines.iter().skip(start).take(max_lines) {
        let truncated: String = line.chars().take(inner_width).collect();
        lines.push(format!(" \x1b[90m>\x1b[0m {}", truncated));
    }

    // Pad remaining height
    while lines.len() < height {
        lines.push(" ".repeat(width));
    }

    lines
}

fn render_widget(widget: &WidgetConfig, width: usize, height: usize) -> Vec<String> {
    match widget.widget_type.as_str() {
        "box" => render_box_widget(&widget.content, width, height),
        "gauge" => render_gauge_widget(&widget.content, width, height),
        "sparkline" => render_sparkline_widget(&widget.content, width, height),
        "log" => render_log_widget(&widget.content, width, height),
        _ => {
            let mut lines = vec![format!(" Unknown: {} ", widget.widget_type)];
            for _ in 1..height {
                lines.push(" ".repeat(width));
            }
            lines
        }
    }
}

fn render_grid(
    stdout: &mut io::Stdout,
    layout: &Layout,
    widgets: &[WidgetConfig],
    term_width: u16,
    term_height: u16,
) -> io::Result<()> {
    let cell_width = term_width as usize / layout.cols;
    let cell_height = (term_height as usize).saturating_sub(2) / layout.rows;

    execute!(stdout, Clear(ClearType::All), MoveTo(0, 0))?;

    // Header
    writeln!(stdout, "\x1b[7m TUI Mode | {}x{} | Press 'q' to quit \x1b[0m", layout.rows, layout.cols)?;

    for row in 0..layout.rows {
        let mut row_lines: Vec<Vec<String>> = Vec::new();

        for col in 0..layout.cols {
            let idx = row * layout.cols + col;
            let widget = widgets.get(idx);
            let rendered = if let Some(w) = widget {
                render_widget(w, cell_width, cell_height)
            } else {
                vec![" ".repeat(cell_width); cell_height]
            };
            row_lines.push(rendered);
        }

        // Print row lines
        for line_idx in 0..cell_height {
            let mut line = String::new();
            for col_lines in &row_lines {
                if let Some(col_line) = col_lines.get(line_idx) {
                    line.push_str(col_line);
                } else {
                    line.push_str(&" ".repeat(cell_width));
                }
            }
            writeln!(stdout, "{}", line)?;
        }
    }

    stdout.flush()?;
    Ok(())
}

pub fn render(
    config_path: Option<String>,
    layout: Option<String>,
    widgets: Option<String>,
    refresh: u64,
) -> Result<(), String> {
    // Parse configuration
    let config = if let Some(path) = config_path {
        let content = fs::read_to_string(&path)
            .map_err(|e| format!("Failed to read config file: {}", e))?;
        serde_json::from_str::<TuiConfig>(&content)
            .map_err(|e| format!("Failed to parse config file: {}", e))?
    } else if let (Some(layout_str), Some(widgets_str)) = (layout, widgets) {
        let parsed_layout = Layout::parse(&layout_str)?;
        let parsed_widgets = parse_inline_widgets(&widgets_str)?;

        if parsed_widgets.len() != parsed_layout.cell_count() {
            return Err(format!(
                "Widget count mismatch: layout {} requires {} widgets, got {}",
                layout_str,
                parsed_layout.cell_count(),
                parsed_widgets.len()
            ));
        }

        TuiConfig {
            layout: layout_str,
            widgets: parsed_widgets,
            refresh_interval: refresh,
        }
    } else {
        return Err("Either --config or both --layout and --widgets must be provided".to_string());
    };

    // Validate layout
    let layout = Layout::parse(&config.layout)?;
    if config.widgets.len() != layout.cell_count() {
        return Err(format!(
            "Widget count mismatch: layout {} requires {} widgets, got {}",
            config.layout,
            layout.cell_count(),
            config.widgets.len()
        ));
    }

    // Enter TUI mode
    let mut stdout = io::stdout();
    terminal::enable_raw_mode().map_err(|e| e.to_string())?;
    execute!(stdout, EnterAlternateScreen, Hide).map_err(|e| e.to_string())?;

    let result = run_tui_loop(&mut stdout, &layout, &config.widgets, config.refresh_interval);

    // Cleanup
    execute!(stdout, LeaveAlternateScreen, Show).ok();
    terminal::disable_raw_mode().ok();

    result
}

fn run_tui_loop(
    stdout: &mut io::Stdout,
    layout: &Layout,
    widgets: &[WidgetConfig],
    refresh_ms: u64,
) -> Result<(), String> {
    let refresh_duration = Duration::from_millis(refresh_ms);

    loop {
        let (term_width, term_height) = terminal::size().map_err(|e| e.to_string())?;
        render_grid(stdout, layout, widgets, term_width, term_height)
            .map_err(|e| e.to_string())?;

        // Poll for events with timeout
        if event::poll(refresh_duration).map_err(|e| e.to_string())? {
            if let Event::Key(key_event) = event::read().map_err(|e| e.to_string())? {
                match key_event.code {
                    KeyCode::Char('q') => break,
                    KeyCode::Char('c') if key_event.modifiers.contains(KeyModifiers::CONTROL) => {
                        break;
                    }
                    KeyCode::Esc => break,
                    _ => {}
                }
            }
        }
    }

    Ok(())
}
