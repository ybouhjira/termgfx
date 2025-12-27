use crossterm::{
    cursor::{Hide, MoveTo, Show},
    event::{self, Event, KeyCode, KeyEvent},
    execute,
    style::{Color, Print, ResetColor, SetForegroundColor},
    terminal::{self, Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::io::{self, IsTerminal, Write};

#[derive(Debug, Clone, Copy, PartialEq)]
enum ComponentPage {
    Box,
    Progress,
    Gauge,
    Sparkline,
}

impl ComponentPage {
    fn all() -> Vec<Self> {
        vec![
            ComponentPage::Box,
            ComponentPage::Progress,
            ComponentPage::Gauge,
            ComponentPage::Sparkline,
        ]
    }

    fn name(&self) -> &str {
        match self {
            ComponentPage::Box => "Box",
            ComponentPage::Progress => "Progress",
            ComponentPage::Gauge => "Gauge",
            ComponentPage::Sparkline => "Sparkline",
        }
    }

    fn description(&self) -> &str {
        match self {
            ComponentPage::Box => "Styled boxes with borders",
            ComponentPage::Progress => "Progress bars",
            ComponentPage::Gauge => "Radial gauge indicators",
            ComponentPage::Sparkline => "Mini charts",
        }
    }
}

#[derive(Debug, Clone)]
struct BoxParams {
    message: String,
    style: String,
    border: String,
    emoji: String,
}

impl Default for BoxParams {
    fn default() -> Self {
        Self {
            message: "Hello World!".to_string(),
            style: "success".to_string(),
            border: "rounded".to_string(),
            emoji: "✨".to_string(),
        }
    }
}

#[derive(Debug, Clone)]
struct ProgressParams {
    percent: u8,
    style: String,
}

impl Default for ProgressParams {
    fn default() -> Self {
        Self {
            percent: 75,
            style: "gradient".to_string(),
        }
    }
}

#[derive(Debug, Clone)]
struct GaugeParams {
    value: f64,
    label: String,
    style: String,
}

impl Default for GaugeParams {
    fn default() -> Self {
        Self {
            value: 75.0,
            label: "CPU".to_string(),
            style: "semicircle".to_string(),
        }
    }
}

#[derive(Debug, Clone)]
struct SparklineParams {
    data: String,
}

impl Default for SparklineParams {
    fn default() -> Self {
        Self {
            data: "1,4,2,8,5,7,3,9,6".to_string(),
        }
    }
}

struct PlaygroundApp {
    current_page: usize,
    selected_param: usize,
    editing: bool,
    edit_buffer: String,
    box_params: BoxParams,
    progress_params: ProgressParams,
    gauge_params: GaugeParams,
    sparkline_params: SparklineParams,
}

impl PlaygroundApp {
    fn new() -> Self {
        Self {
            current_page: 0,
            selected_param: 0,
            editing: false,
            edit_buffer: String::new(),
            box_params: BoxParams::default(),
            progress_params: ProgressParams::default(),
            gauge_params: GaugeParams::default(),
            sparkline_params: SparklineParams::default(),
        }
    }

    fn current_page_type(&self) -> ComponentPage {
        ComponentPage::all()[self.current_page]
    }

    fn param_count(&self) -> usize {
        match self.current_page_type() {
            ComponentPage::Box => 4,
            ComponentPage::Progress => 2,
            ComponentPage::Gauge => 3,
            ComponentPage::Sparkline => 1,
        }
    }

    fn get_param_name(&self, idx: usize) -> String {
        match self.current_page_type() {
            ComponentPage::Box => match idx {
                0 => "Message".to_string(),
                1 => "Style".to_string(),
                2 => "Border".to_string(),
                3 => "Emoji".to_string(),
                _ => String::new(),
            },
            ComponentPage::Progress => match idx {
                0 => "Percent".to_string(),
                1 => "Style".to_string(),
                _ => String::new(),
            },
            ComponentPage::Gauge => match idx {
                0 => "Value".to_string(),
                1 => "Label".to_string(),
                2 => "Style".to_string(),
                _ => String::new(),
            },
            ComponentPage::Sparkline => match idx {
                0 => "Data".to_string(),
                _ => String::new(),
            },
        }
    }

    fn get_param_value(&self, idx: usize) -> String {
        match self.current_page_type() {
            ComponentPage::Box => match idx {
                0 => self.box_params.message.clone(),
                1 => self.box_params.style.clone(),
                2 => self.box_params.border.clone(),
                3 => self.box_params.emoji.clone(),
                _ => String::new(),
            },
            ComponentPage::Progress => match idx {
                0 => self.progress_params.percent.to_string(),
                1 => self.progress_params.style.clone(),
                _ => String::new(),
            },
            ComponentPage::Gauge => match idx {
                0 => self.gauge_params.value.to_string(),
                1 => self.gauge_params.label.clone(),
                2 => self.gauge_params.style.clone(),
                _ => String::new(),
            },
            ComponentPage::Sparkline => match idx {
                0 => self.sparkline_params.data.clone(),
                _ => String::new(),
            },
        }
    }

    fn set_param_value(&mut self, idx: usize, value: String) {
        match self.current_page_type() {
            ComponentPage::Box => match idx {
                0 => self.box_params.message = value,
                1 => self.box_params.style = value,
                2 => self.box_params.border = value,
                3 => self.box_params.emoji = value,
                _ => {}
            },
            ComponentPage::Progress => match idx {
                0 => {
                    if let Ok(v) = value.parse::<u8>() {
                        self.progress_params.percent = v.min(100);
                    }
                }
                1 => self.progress_params.style = value,
                _ => {}
            },
            ComponentPage::Gauge => match idx {
                0 => {
                    if let Ok(v) = value.parse::<f64>() {
                        self.gauge_params.value = v.min(100.0).max(0.0);
                    }
                }
                1 => self.gauge_params.label = value,
                2 => self.gauge_params.style = value,
                _ => {}
            },
            ComponentPage::Sparkline => match idx {
                0 => self.sparkline_params.data = value,
                _ => {}
            },
        }
    }

    fn generate_command(&self) -> String {
        match self.current_page_type() {
            ComponentPage::Box => {
                format!(
                    "termgfx box \"{}\" --style {} --border {} --emoji {}",
                    self.box_params.message,
                    self.box_params.style,
                    self.box_params.border,
                    self.box_params.emoji
                )
            }
            ComponentPage::Progress => {
                format!(
                    "termgfx progress {} --style {}",
                    self.progress_params.percent, self.progress_params.style
                )
            }
            ComponentPage::Gauge => {
                format!(
                    "termgfx gauge {} --label \"{}\" --style {}",
                    self.gauge_params.value, self.gauge_params.label, self.gauge_params.style
                )
            }
            ComponentPage::Sparkline => {
                format!("termgfx sparkline \"{}\"", self.sparkline_params.data)
            }
        }
    }
}

pub fn render() {
    match run_playground() {
        Ok(_) => {}
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    }
}

fn run_playground() -> io::Result<()> {
    // Check for interactive terminal
    if !std::io::stdin().is_terminal() {
        return Err(io::Error::new(
            io::ErrorKind::Other,
            "Playground requires an interactive terminal (TTY)",
        ));
    }

    let mut stdout = io::stdout();
    let mut app = PlaygroundApp::new();

    // Setup terminal
    terminal::enable_raw_mode()?;
    execute!(stdout, EnterAlternateScreen, Hide)?;

    let result = loop {
        // Render UI
        render_ui(&mut stdout, &app)?;

        // Handle key events
        if let Event::Key(KeyEvent { code, .. }) = event::read()? {
            if app.editing {
                match code {
                    KeyCode::Char(c) => {
                        app.edit_buffer.push(c);
                    }
                    KeyCode::Backspace => {
                        app.edit_buffer.pop();
                    }
                    KeyCode::Enter => {
                        app.set_param_value(app.selected_param, app.edit_buffer.clone());
                        app.editing = false;
                        app.edit_buffer.clear();
                    }
                    KeyCode::Esc => {
                        app.editing = false;
                        app.edit_buffer.clear();
                    }
                    _ => {}
                }
            } else {
                match code {
                    KeyCode::Left | KeyCode::Char('h') => {
                        if app.current_page > 0 {
                            app.current_page -= 1;
                            app.selected_param = 0;
                        }
                    }
                    KeyCode::Right | KeyCode::Char('l') => {
                        if app.current_page < ComponentPage::all().len() - 1 {
                            app.current_page += 1;
                            app.selected_param = 0;
                        }
                    }
                    KeyCode::Up | KeyCode::Char('k') => {
                        if app.selected_param > 0 {
                            app.selected_param -= 1;
                        }
                    }
                    KeyCode::Down | KeyCode::Char('j') => {
                        if app.selected_param < app.param_count() - 1 {
                            app.selected_param += 1;
                        }
                    }
                    KeyCode::Enter => {
                        app.editing = true;
                        app.edit_buffer = app.get_param_value(app.selected_param);
                    }
                    KeyCode::Char('q') | KeyCode::Esc => {
                        break Ok(());
                    }
                    _ => {}
                }
            }
        }
    };

    // Cleanup terminal
    execute!(stdout, Show, LeaveAlternateScreen)?;
    terminal::disable_raw_mode()?;

    result
}

fn render_ui(stdout: &mut io::Stdout, app: &PlaygroundApp) -> io::Result<()> {
    execute!(stdout, Clear(ClearType::All), MoveTo(0, 0))?;

    let pages = ComponentPage::all();
    let current = app.current_page_type();

    // Header
    execute!(
        stdout,
        SetForegroundColor(Color::Cyan),
        Print("╔══════════════════════════════════════════════════════════════════════╗\n"),
        Print("║"),
        SetForegroundColor(Color::White),
        Print("           🎨 "),
        SetForegroundColor(Color::Magenta),
        Print("termgfx Interactive Playground"),
        SetForegroundColor(Color::White),
        Print("                        ║\n"),
        SetForegroundColor(Color::Cyan),
        Print("╚══════════════════════════════════════════════════════════════════════╝\n"),
        ResetColor,
    )?;

    // Page tabs
    execute!(stdout, Print("\n  "))?;
    for (i, page) in pages.iter().enumerate() {
        if i == app.current_page {
            execute!(
                stdout,
                SetForegroundColor(Color::Green),
                Print("[ "),
                Print(page.name()),
                Print(" ]"),
                ResetColor,
                Print("  ")
            )?;
        } else {
            execute!(
                stdout,
                SetForegroundColor(Color::DarkGrey),
                Print("  "),
                Print(page.name()),
                Print("   "),
                ResetColor
            )?;
        }
    }

    execute!(stdout, Print("\n  "))?;
    execute!(
        stdout,
        SetForegroundColor(Color::DarkGrey),
        Print(current.description()),
        ResetColor,
        Print("\n\n")
    )?;

    // Parameters section
    execute!(
        stdout,
        SetForegroundColor(Color::Yellow),
        Print("  Parameters:\n"),
        ResetColor
    )?;

    for i in 0..app.param_count() {
        let name = app.get_param_name(i);
        let value = if app.editing && app.selected_param == i {
            &app.edit_buffer
        } else {
            &app.get_param_value(i)
        };

        let marker = if app.selected_param == i { "▶" } else { " " };
        let marker_color = if app.selected_param == i {
            Color::Green
        } else {
            Color::DarkGrey
        };

        execute!(
            stdout,
            Print("  "),
            SetForegroundColor(marker_color),
            Print(marker),
            ResetColor,
            Print(" "),
            SetForegroundColor(Color::Cyan),
            Print(format!("{:12}", name)),
            ResetColor,
            Print(": "),
        )?;

        if app.editing && app.selected_param == i {
            execute!(
                stdout,
                SetForegroundColor(Color::Yellow),
                Print(value),
                Print("█"),
                ResetColor
            )?;
        } else {
            execute!(
                stdout,
                SetForegroundColor(Color::White),
                Print(value),
                ResetColor
            )?;
        }

        execute!(stdout, Print("\n"))?;
    }

    // Preview section
    execute!(
        stdout,
        Print("\n"),
        SetForegroundColor(Color::Yellow),
        Print("  Preview:\n"),
        ResetColor,
        Print("  "),
        SetForegroundColor(Color::DarkGrey),
        Print("─────────────────────────────────────────────────────────────────────\n"),
        ResetColor
    )?;

    // Render preview
    render_preview(stdout, app)?;

    execute!(
        stdout,
        Print("  "),
        SetForegroundColor(Color::DarkGrey),
        Print("─────────────────────────────────────────────────────────────────────\n"),
        ResetColor
    )?;

    // Command section
    execute!(
        stdout,
        Print("\n"),
        SetForegroundColor(Color::Yellow),
        Print("  Generated Command:\n"),
        ResetColor,
        Print("  "),
        SetForegroundColor(Color::Green),
        Print(app.generate_command()),
        ResetColor,
        Print("\n\n")
    )?;

    // Help footer
    execute!(
        stdout,
        SetForegroundColor(Color::DarkGrey),
        Print("  ← → or h/l: Change page  │  ↑ ↓ or k/j: Select param  │  Enter: Edit  │  q/Esc: Quit\n"),
        ResetColor
    )?;

    stdout.flush()?;
    Ok(())
}

fn render_preview(stdout: &mut io::Stdout, app: &PlaygroundApp) -> io::Result<()> {
    match app.current_page_type() {
        ComponentPage::Box => {
            render_box_preview(stdout, &app.box_params)?;
        }
        ComponentPage::Progress => {
            render_progress_preview(stdout, &app.progress_params)?;
        }
        ComponentPage::Gauge => {
            render_gauge_preview(stdout, &app.gauge_params)?;
        }
        ComponentPage::Sparkline => {
            render_sparkline_preview(stdout, &app.sparkline_params)?;
        }
    }
    Ok(())
}

fn render_box_preview(stdout: &mut io::Stdout, params: &BoxParams) -> io::Result<()> {
    // Simple box preview - just show the styled message
    let border_char = match params.border.as_str() {
        "rounded" => "╭─╮│╰─╯",
        "double" => "╔═╗║╚═╝",
        "thick" => "┏━┓┃┗━┛",
        "single" | _ => "┌─┐│└─┘",
    };

    let color = match params.style.as_str() {
        "success" => Color::Green,
        "warning" => Color::Yellow,
        "danger" => Color::Red,
        "info" | _ => Color::Cyan,
    };

    let chars: Vec<char> = border_char.chars().collect();
    let width = params.message.len() + 4;

    execute!(stdout, Print("  "))?;
    execute!(stdout, SetForegroundColor(color), Print(chars[0]))?;
    for _ in 0..width {
        execute!(stdout, Print(chars[1]))?;
    }
    execute!(stdout, Print(chars[2]), Print("\n  "))?;

    execute!(
        stdout,
        Print(chars[3]),
        Print(" "),
        Print(&params.emoji),
        Print(" "),
        Print(&params.message),
        Print(" "),
        Print(chars[3]),
        Print("\n  ")
    )?;

    execute!(stdout, Print(chars[4]))?;
    for _ in 0..width {
        execute!(stdout, Print(chars[5]))?;
    }
    execute!(stdout, Print(chars[6]), ResetColor, Print("\n"))?;

    Ok(())
}

fn render_progress_preview(stdout: &mut io::Stdout, params: &ProgressParams) -> io::Result<()> {
    let width = 50;
    let filled = (width * params.percent as usize) / 100;

    execute!(stdout, Print("  "))?;

    let bar_char = match params.style.as_str() {
        "blocks" => "█",
        "modern" => "━",
        "thin" => "─",
        "gradient" | _ => "█",
    };

    execute!(stdout, SetForegroundColor(Color::Green))?;
    for _ in 0..filled {
        execute!(stdout, Print(bar_char))?;
    }
    execute!(stdout, SetForegroundColor(Color::DarkGrey))?;
    for _ in filled..width {
        execute!(stdout, Print("░"))?;
    }
    execute!(stdout, ResetColor, Print(format!(" {}%\n", params.percent)))?;

    Ok(())
}

fn render_gauge_preview(stdout: &mut io::Stdout, params: &GaugeParams) -> io::Result<()> {
    // Simple semicircle gauge
    execute!(stdout, Print("  "))?;

    if params.style == "semicircle" {
        let segments = 20;
        let filled = ((params.value / 100.0) * segments as f64) as usize;

        execute!(stdout, SetForegroundColor(Color::Cyan))?;
        for i in 0..segments {
            if i < filled {
                execute!(stdout, Print("●"))?;
            } else {
                execute!(stdout, SetForegroundColor(Color::DarkGrey), Print("○"))?;
            }
        }
        execute!(
            stdout,
            ResetColor,
            Print(format!("\n  {} {:.0}%\n", params.label, params.value))
        )?;
    } else {
        execute!(
            stdout,
            Print(format!("{}: {:.0}%\n", params.label, params.value))
        )?;
    }

    Ok(())
}

fn render_sparkline_preview(stdout: &mut io::Stdout, params: &SparklineParams) -> io::Result<()> {
    // Parse data and render mini chart
    let values: Vec<f64> = params
        .data
        .split(|c| c == ',' || c == ';')
        .filter_map(|s| s.trim().parse().ok())
        .collect();

    if values.is_empty() {
        execute!(stdout, Print("  (no valid data)\n"))?;
        return Ok(());
    }

    let max_val = values.iter().cloned().fold(0.0, f64::max);
    let min_val = values.iter().cloned().fold(f64::INFINITY, f64::min);
    let range = max_val - min_val;

    execute!(stdout, Print("  "), SetForegroundColor(Color::Green))?;

    let spark_chars = ['▁', '▂', '▃', '▄', '▅', '▆', '▇', '█'];

    for val in values {
        let normalized = if range > 0.0 {
            ((val - min_val) / range * (spark_chars.len() - 1) as f64) as usize
        } else {
            spark_chars.len() / 2
        };
        execute!(
            stdout,
            Print(spark_chars[normalized.min(spark_chars.len() - 1)])
        )?;
    }

    execute!(stdout, ResetColor, Print("\n"))?;

    Ok(())
}
