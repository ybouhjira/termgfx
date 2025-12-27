use crossterm::{
    event::{self, Event, KeyCode, KeyModifiers},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use serde::{Deserialize, Serialize};
use std::fs;
use std::io::{self, IsTerminal, Write};
use std::time::{Duration, Instant};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TuiConfig {
    pub layout: String,
    #[serde(default)]
    pub widgets: Vec<Widget>,
    #[serde(default = "default_refresh_interval")]
    pub refresh_interval: u64, // milliseconds
}

fn default_refresh_interval() -> u64 {
    1000
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Widget {
    #[serde(rename = "type")]
    pub widget_type: String,
    pub content: String,
}

struct Layout {
    rows: usize,
    cols: usize,
}

pub struct TuiApp {
    config: TuiConfig,
    layout: Layout,
    running: bool,
    last_refresh: Instant,
}

impl TuiApp {
    pub fn new(config: TuiConfig) -> Result<Self, String> {
        let layout = parse_layout(&config.layout)?;
        let widget_count = config.widgets.len();
        let expected_count = layout.rows * layout.cols;

        if widget_count != expected_count {
            return Err(format!(
                "Widget count mismatch: expected {} widgets for {}x{} layout, got {}",
                expected_count, layout.rows, layout.cols, widget_count
            ));
        }

        Ok(TuiApp {
            config,
            layout,
            running: false,
            last_refresh: Instant::now(),
        })
    }

    pub fn run(&mut self) -> io::Result<()> {
        // Check for interactive terminal
        if !std::io::stdin().is_terminal() {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                "TUI mode requires an interactive terminal (TTY)",
            ));
        }

        // Setup terminal
        enable_raw_mode()?;
        let mut stdout = io::stdout();
        execute!(stdout, EnterAlternateScreen)?;

        // Setup Ctrl+C handler
        let running = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(true));
        let r = running.clone();
        ctrlc::set_handler(move || {
            r.store(false, std::sync::atomic::Ordering::SeqCst);
        })
        .expect("Error setting Ctrl+C handler");

        self.running = true;
        self.last_refresh = Instant::now();

        // Initial render
        self.render()?;

        // Event loop
        while self.running && running.load(std::sync::atomic::Ordering::SeqCst) {
            // Check if we need to refresh
            let elapsed = self.last_refresh.elapsed();
            if elapsed.as_millis() >= self.config.refresh_interval as u128 {
                self.render()?;
                self.last_refresh = Instant::now();
            }

            // Poll for events with timeout
            if event::poll(Duration::from_millis(100))? {
                if let Event::Key(key) = event::read()? {
                    match key.code {
                        KeyCode::Char('q') => {
                            self.running = false;
                        }
                        KeyCode::Char('r') => {
                            self.render()?;
                            self.last_refresh = Instant::now();
                        }
                        KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                            self.running = false;
                        }
                        _ => {}
                    }
                }
            }
        }

        // Cleanup
        self.cleanup()?;
        Ok(())
    }

    fn render(&self) -> io::Result<()> {
        let mut stdout = io::stdout();

        // Clear screen
        execute!(
            stdout,
            crossterm::terminal::Clear(crossterm::terminal::ClearType::All),
            crossterm::cursor::MoveTo(0, 0)
        )?;

        // Get terminal size
        let (term_width, term_height) = crossterm::terminal::size()?;

        // Calculate cell dimensions
        let cell_width = term_width / self.layout.cols as u16;
        let cell_height = term_height / self.layout.rows as u16;

        // Render each widget in its grid cell
        for (idx, widget) in self.config.widgets.iter().enumerate() {
            let row = idx / self.layout.cols;
            let col = idx % self.layout.cols;

            let x = col as u16 * cell_width;
            let y = row as u16 * cell_height;

            self.render_widget(widget, x, y, cell_width, cell_height)?;
        }

        // Render help text at bottom
        execute!(stdout, crossterm::cursor::MoveTo(0, term_height - 1),)?;
        write!(
            stdout,
            " [q] quit | [r] refresh | Auto-refresh: {}ms",
            self.config.refresh_interval
        )?;

        stdout.flush()?;
        Ok(())
    }

    fn render_widget(
        &self,
        widget: &Widget,
        x: u16,
        y: u16,
        width: u16,
        height: u16,
    ) -> io::Result<()> {
        let mut stdout = io::stdout();

        // Draw border
        self.draw_border(x, y, width, height)?;

        // Render widget content based on type
        execute!(stdout, crossterm::cursor::MoveTo(x + 2, y + 1))?;

        match widget.widget_type.as_str() {
            "box" => {
                write!(stdout, "{}", widget.content)?;
            }
            "gauge" => {
                let value = widget.content.parse::<f64>().unwrap_or(0.0);
                self.render_gauge(value, x + 2, y + 2, width - 4)?;
            }
            "sparkline" => {
                let values = self.parse_sparkline_data(&widget.content);
                self.render_sparkline(&values, x + 2, y + 2, width - 4)?;
            }
            "log" => {
                let lines: Vec<&str> = widget.content.lines().collect();
                let max_lines = (height - 3) as usize;
                for (i, line) in lines.iter().take(max_lines).enumerate() {
                    execute!(stdout, crossterm::cursor::MoveTo(x + 2, y + 2 + i as u16))?;
                    let truncated = if line.len() > (width - 4) as usize {
                        &line[0..(width - 4) as usize]
                    } else {
                        line
                    };
                    write!(stdout, "{}", truncated)?;
                }
            }
            _ => {
                write!(stdout, "Unknown widget type: {}", widget.widget_type)?;
            }
        }

        Ok(())
    }

    fn draw_border(&self, x: u16, y: u16, width: u16, height: u16) -> io::Result<()> {
        let mut stdout = io::stdout();

        // Guard against too-small dimensions
        if width < 3 || height < 3 {
            return Ok(());
        }

        // Top border
        execute!(stdout, crossterm::cursor::MoveTo(x, y))?;
        write!(stdout, "┌{}┐", "─".repeat(width.saturating_sub(2) as usize))?;

        // Side borders
        for i in 1..height.saturating_sub(1) {
            execute!(stdout, crossterm::cursor::MoveTo(x, y + i))?;
            write!(stdout, "│")?;
            execute!(
                stdout,
                crossterm::cursor::MoveTo(x + width.saturating_sub(1), y + i)
            )?;
            write!(stdout, "│")?;
        }

        // Bottom border
        execute!(
            stdout,
            crossterm::cursor::MoveTo(x, y + height.saturating_sub(1))
        )?;
        write!(stdout, "└{}┘", "─".repeat(width.saturating_sub(2) as usize))?;

        Ok(())
    }

    fn render_gauge(&self, value: f64, x: u16, y: u16, width: u16) -> io::Result<()> {
        let mut stdout = io::stdout();
        let percentage = value.min(100.0).max(0.0);
        let filled = ((width as f64) * percentage / 100.0) as usize;

        execute!(stdout, crossterm::cursor::MoveTo(x, y))?;
        write!(
            stdout,
            "{}{}  {:.1}%",
            "█".repeat(filled),
            "░".repeat((width as usize).saturating_sub(filled)),
            percentage
        )?;

        Ok(())
    }

    fn render_sparkline(&self, values: &[f64], x: u16, y: u16, width: u16) -> io::Result<()> {
        let mut stdout = io::stdout();

        if values.is_empty() {
            return Ok(());
        }

        let max = values.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b));
        let min = values.iter().fold(f64::INFINITY, |a, &b| a.min(b));
        let range = max - min;

        if range == 0.0 {
            execute!(stdout, crossterm::cursor::MoveTo(x, y))?;
            write!(stdout, "{}", "▄".repeat(values.len().min(width as usize)))?;
            return Ok(());
        }

        let sparkline_chars = ['▁', '▂', '▃', '▄', '▅', '▆', '▇', '█'];
        let max_values = values.len().min(width as usize);

        execute!(stdout, crossterm::cursor::MoveTo(x, y))?;
        for value in values.iter().take(max_values) {
            let normalized = ((value - min) / range * 7.0) as usize;
            let char_idx = normalized.min(7);
            write!(stdout, "{}", sparkline_chars[char_idx])?;
        }

        Ok(())
    }

    fn parse_sparkline_data(&self, data: &str) -> Vec<f64> {
        data.split(&[',', ';'][..])
            .filter_map(|s| s.trim().parse::<f64>().ok())
            .collect()
    }

    fn cleanup(&self) -> io::Result<()> {
        let mut stdout = io::stdout();
        execute!(stdout, LeaveAlternateScreen)?;
        disable_raw_mode()?;
        Ok(())
    }
}

fn parse_layout(layout_str: &str) -> Result<Layout, String> {
    let parts: Vec<&str> = layout_str.split('x').collect();
    if parts.len() != 2 {
        return Err(format!(
            "Invalid layout format: '{}'. Expected format: NxM (e.g., 2x2, 3x1)",
            layout_str
        ));
    }

    let rows = parts[0]
        .parse::<usize>()
        .map_err(|_| format!("Invalid row count in layout: '{}'", parts[0]))?;
    let cols = parts[1]
        .parse::<usize>()
        .map_err(|_| format!("Invalid column count in layout: '{}'", parts[1]))?;

    if rows == 0 || cols == 0 {
        return Err("Layout dimensions must be greater than 0".to_string());
    }

    Ok(Layout { rows, cols })
}

pub fn render(
    config_path: Option<String>,
    layout: Option<String>,
    widgets: Option<String>,
    refresh: u64,
) -> Result<(), String> {
    let config = if let Some(path) = config_path {
        // Load from JSON config file
        let content = fs::read_to_string(&path)
            .map_err(|e| format!("Failed to read config file '{}': {}", path, e))?;
        serde_json::from_str::<TuiConfig>(&content)
            .map_err(|e| format!("Failed to parse config file: {}", e))?
    } else if let (Some(layout_str), Some(widgets_str)) = (layout, widgets) {
        // Parse inline definition
        let parsed_widgets = parse_inline_widgets(&widgets_str)?;
        TuiConfig {
            layout: layout_str,
            widgets: parsed_widgets,
            refresh_interval: refresh,
        }
    } else {
        return Err("Either --config or both --layout and --widgets must be provided".to_string());
    };

    let mut app = TuiApp::new(config)?;
    app.run().map_err(|e| format!("TUI error: {}", e))?;

    Ok(())
}

fn parse_inline_widgets(widgets_str: &str) -> Result<Vec<Widget>, String> {
    let mut widgets = Vec::new();

    for widget_def in widgets_str.split(',') {
        let parts: Vec<&str> = widget_def.trim().split(':').collect();
        if parts.len() != 2 {
            return Err(format!(
                "Invalid widget definition: '{}'. Expected format: type:content",
                widget_def
            ));
        }

        widgets.push(Widget {
            widget_type: parts[0].to_string(),
            content: parts[1].to_string(),
        });
    }

    Ok(widgets)
}
