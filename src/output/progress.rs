//! Stateful progress bar component for animated loading indicators
//!
//! Provides a dual-mode progress bar that works both as:
//! - Standalone TTY mode (blocking, backwards-compatible)
//! - Ratatui Widget (non-blocking, for integration with TUI apps)
//!
//! # Example (TTY mode)
//! ```ignore
//! let mut progress = ProgressState::new(100);
//! progress.run_animated(Duration::from_millis(2000));
//! ```
//!
//! # Example (Ratatui mode)
//! ```ignore
//! let mut progress = ProgressState::new(75);
//! // In your event loop:
//! if progress.tick() {
//!     // Value changed, need to redraw
//! }
//! // In your render function:
//! frame.render_widget(&progress, area);
//! ```

use owo_colors::OwoColorize;
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Style},
    text::Span,
    widgets::Widget,
};
use std::io::{self, IsTerminal, Write};
use std::thread;
use std::time::{Duration, Instant};

/// Available progress bar styles
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ProgressStyle {
    #[default]
    Gradient,
    Blocks,
    Modern,
    Classic,
    Thin,
    Animated,
}

impl std::str::FromStr for ProgressStyle {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s.to_lowercase().as_str() {
            "gradient" => ProgressStyle::Gradient,
            "blocks" => ProgressStyle::Blocks,
            "modern" => ProgressStyle::Modern,
            "classic" => ProgressStyle::Classic,
            "thin" => ProgressStyle::Thin,
            "animated" => ProgressStyle::Animated,
            _ => ProgressStyle::Gradient,
        })
    }
}

impl ProgressStyle {
    /// Get the bar character for filled portions
    pub fn filled_char(&self) -> &'static str {
        match self {
            ProgressStyle::Blocks => "█",
            ProgressStyle::Gradient => "█",
            ProgressStyle::Modern => "█",
            ProgressStyle::Classic => "=",
            ProgressStyle::Thin => "━",
            ProgressStyle::Animated => "█",
        }
    }

    /// Get the bar character for empty portions
    pub fn empty_char(&self) -> &'static str {
        match self {
            ProgressStyle::Blocks => "░",
            ProgressStyle::Gradient => "░",
            ProgressStyle::Modern => "░",
            ProgressStyle::Classic => " ",
            ProgressStyle::Thin => "━",
            ProgressStyle::Animated => " ",
        }
    }
}

/// RGB color for gradient customization
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Rgb {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Rgb {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }

    /// Interpolate between two colors
    pub fn lerp(&self, other: &Rgb, t: f32) -> Rgb {
        Rgb {
            r: (self.r as f32 + t * (other.r as f32 - self.r as f32)) as u8,
            g: (self.g as f32 + t * (other.g as f32 - self.g as f32)) as u8,
            b: (self.b as f32 + t * (other.b as f32 - self.b as f32)) as u8,
        }
    }

    /// Convert to ratatui Color
    pub fn to_color(self) -> Color {
        Color::Rgb(self.r, self.g, self.b)
    }
}

impl Default for Rgb {
    fn default() -> Self {
        Self::new(88, 166, 255) // Cyan-blue
    }
}

/// Stateful progress bar that manages animation and value
#[derive(Debug, Clone)]
pub struct ProgressState {
    /// Target percentage (0-100)
    target: u8,
    /// Current displayed percentage (for animation)
    current: u8,
    /// Progress bar style
    style: ProgressStyle,
    /// Start color for gradient
    color_from: Rgb,
    /// End color for gradient
    color_to: Rgb,
    /// Bar width in characters
    width: u16,
    /// When the last update occurred
    last_tick: Instant,
    /// Animation step size (percent per tick)
    step: u8,
    /// Animation interval
    interval: Duration,
    /// Whether animation is complete
    finished: bool,
}

impl ProgressState {
    /// Create a new progress bar with target percentage
    pub fn new(target: u8) -> Self {
        Self {
            target: target.min(100),
            current: 0,
            style: ProgressStyle::default(),
            color_from: Rgb::new(63, 185, 80), // Green
            color_to: Rgb::new(88, 166, 255),  // Cyan-blue
            width: 30,
            last_tick: Instant::now(),
            step: 3,
            interval: Duration::from_millis(50),
            finished: false,
        }
    }

    /// Create a progress bar at a specific value (no animation)
    pub fn at(value: u8) -> Self {
        Self {
            target: value.min(100),
            current: value.min(100),
            style: ProgressStyle::default(),
            color_from: Rgb::new(63, 185, 80),
            color_to: Rgb::new(88, 166, 255),
            width: 30,
            last_tick: Instant::now(),
            step: 3,
            interval: Duration::from_millis(50),
            finished: true,
        }
    }

    /// Set the progress bar style
    pub fn with_style(mut self, style: ProgressStyle) -> Self {
        self.style = style;
        self
    }

    /// Set custom gradient colors
    pub fn with_gradient(mut self, from: Rgb, to: Rgb) -> Self {
        self.color_from = from;
        self.color_to = to;
        self
    }

    /// Set the bar width
    pub fn with_width(mut self, width: u16) -> Self {
        self.width = width;
        self
    }

    /// Set the animation step size
    pub fn with_step(mut self, step: u8) -> Self {
        self.step = step.max(1);
        self
    }

    /// Set the animation interval
    pub fn with_interval(mut self, interval: Duration) -> Self {
        self.interval = interval;
        self
    }

    /// Advance the animation if enough time has passed
    /// Returns true if the value changed (indicating a redraw is needed)
    pub fn tick(&mut self) -> bool {
        if self.finished || self.current >= self.target {
            self.finished = true;
            return false;
        }

        if self.last_tick.elapsed() >= self.interval {
            let new_current = (self.current + self.step).min(self.target);
            if new_current != self.current {
                self.current = new_current;
                self.last_tick = Instant::now();
                if self.current >= self.target {
                    self.finished = true;
                }
                return true;
            }
        }
        false
    }

    /// Force set the current value
    pub fn set_value(&mut self, value: u8) {
        self.current = value.min(100);
        self.last_tick = Instant::now();
        self.finished = self.current >= self.target;
    }

    /// Set a new target and optionally reset
    pub fn set_target(&mut self, target: u8, reset_current: bool) {
        self.target = target.min(100);
        if reset_current {
            self.current = 0;
            self.finished = false;
        }
        // Update finished status based on new target
        self.finished = self.current >= self.target;
    }

    /// Reset the animation
    pub fn reset(&mut self) {
        self.current = 0;
        self.finished = false;
        self.last_tick = Instant::now();
    }

    /// Get the current displayed percentage
    pub fn current(&self) -> u8 {
        self.current
    }

    /// Get the target percentage
    pub fn target(&self) -> u8 {
        self.target
    }

    /// Check if animation is finished
    pub fn is_finished(&self) -> bool {
        self.finished
    }

    /// Get the style
    pub fn style(&self) -> ProgressStyle {
        self.style
    }

    /// Get the bar width
    pub fn width(&self) -> u16 {
        self.width
    }

    /// Get the filled width based on current percentage
    fn filled_width(&self) -> u16 {
        ((self.width as u32 * self.current as u32) / 100) as u16
    }

    /// Calculate color at a given position (0.0 to 1.0)
    fn color_at(&self, t: f32) -> Rgb {
        self.color_from.lerp(&self.color_to, t)
    }

    /// Run the progress animation in blocking TTY mode
    pub fn run_animated(&mut self, duration: Duration) {
        // If not a TTY, just render final state
        if !io::stdout().is_terminal() {
            render(
                self.target,
                &format!("{:?}", self.style).to_lowercase(),
                None,
                None,
            );
            return;
        }

        let steps = 30u32;
        let step_delay = duration / steps;

        for i in 0..=steps {
            self.current = ((i * self.target as u32) / steps) as u8;

            // Build and print the bar
            let bar = self.build_ansi_bar();
            print!("\r{}", bar);
            io::stdout().flush().unwrap();
            thread::sleep(step_delay);
        }
        println!();
        self.finished = true;
    }

    /// Build ANSI-colored bar string for TTY output
    fn build_ansi_bar(&self) -> String {
        let filled = self.filled_width() as usize;
        let empty = (self.width as usize).saturating_sub(filled);
        let mut bar = String::new();

        for i in 0..filled {
            let t = i as f32 / self.width as f32;
            let color = self.color_at(t);
            bar.push_str(&format!(
                "\x1b[38;2;{};{};{}m{}\x1b[0m",
                color.r,
                color.g,
                color.b,
                self.style.filled_char()
            ));
        }

        for _ in 0..empty {
            bar.push_str("\x1b[38;2;72;79;88m");
            bar.push_str(self.style.empty_char());
            bar.push_str("\x1b[0m");
        }

        let end_color = self.color_to;
        let percent_str = format!(
            "\x1b[1m\x1b[38;2;{};{};{}m{:>3}%\x1b[0m",
            end_color.r, end_color.g, end_color.b, self.current
        );

        format!("{} {}", bar, percent_str)
    }
}

impl Default for ProgressState {
    fn default() -> Self {
        Self::new(100)
    }
}

/// Widget for rendering progress bar with ratatui
impl Widget for &ProgressState {
    fn render(self, area: Rect, buf: &mut Buffer) {
        if area.width < 5 || area.height == 0 {
            return;
        }

        // Reserve space for percentage display (e.g., "100%")
        let percent_width = 5;
        let bar_width = area.width.saturating_sub(percent_width + 1).min(self.width);
        let filled = ((bar_width as u32 * self.current as u32) / 100) as u16;
        let empty = bar_width.saturating_sub(filled);

        let mut x = area.x;

        // Render filled portion with gradient
        for i in 0..filled {
            let t = i as f32 / bar_width as f32;
            let color = self.color_at(t);
            let style = Style::default().fg(color.to_color());
            let span = Span::styled(self.style.filled_char(), style);
            buf.set_span(x, area.y, &span, 1);
            x += 1;
        }

        // Render empty portion
        let empty_style = Style::default().fg(Color::Rgb(72, 79, 88));
        for _ in 0..empty {
            let span = Span::styled(self.style.empty_char(), empty_style);
            buf.set_span(x, area.y, &span, 1);
            x += 1;
        }

        // Render percentage
        x += 1; // Gap
        let percent_text = format!("{:>3}%", self.current);
        let percent_style = Style::default()
            .fg(self.color_to.to_color())
            .add_modifier(ratatui::style::Modifier::BOLD);
        let span = Span::styled(percent_text, percent_style);
        buf.set_span(x, area.y, &span, percent_width);
    }
}

// ============================================================================
// Legacy API (backwards compatibility)
// ============================================================================

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
    let percent_str = format!(
        "\x1b[1m\x1b[38;2;{};{};{}m{}%\x1b[0m",
        end.0, end.1, end.2, percent
    );
    println!("{} {}", bar, percent_str);
}

fn render_blocks(percent: u8) {
    let width = 20;
    let filled = (width * percent as usize) / 100;
    let empty = width - filled;
    let mut bar = String::new();
    for _ in 0..filled {
        bar.push('█');
    }
    for _ in 0..empty {
        bar.push('░');
    }
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
        let char = if progress < 33.0 {
            '█'.red().to_string()
        } else if progress < 66.0 {
            '█'.yellow().to_string()
        } else {
            '█'.green().to_string()
        };
        bar.push_str(&char);
    }
    for _ in 0..empty {
        bar.push_str(&"░".bright_black().to_string());
    }
    let percent_display = format!("{}%", percent);
    let percent_colored = if percent < 33 {
        percent_display.red().to_string()
    } else if percent < 66 {
        percent_display.yellow().to_string()
    } else {
        percent_display.green().to_string()
    };
    println!("{} {}", bar, percent_colored.bold());
}

fn render_classic(percent: u8) {
    let width = 20;
    let filled = (width * percent as usize) / 100;
    let empty = width.saturating_sub(filled + 1);
    let mut bar = String::new();
    bar.push('[');
    for _ in 0..filled {
        bar.push_str(&"=".cyan().to_string());
    }
    if filled < width {
        bar.push_str(&">".bright_cyan().to_string());
    }
    for _ in 0..empty {
        bar.push(' ');
    }
    bar.push(']');
    let percent_str = format!("{}%", percent);
    println!("{} {}", bar, percent_str.bright_cyan().bold());
}

fn render_thin(percent: u8) {
    let width = 20;
    let filled = (width * percent as usize) / 100;
    let empty = width - filled;
    let mut bar = String::new();
    for _ in 0..filled {
        bar.push_str(&"━".cyan().to_string());
    }
    for _ in 0..empty {
        bar.push_str(&"━".bright_black().to_string());
    }
    let percent_str = format!("{}%", percent);
    println!("{} {}", bar, percent_str.bright_cyan().bold());
}

fn render_animated(percent: u8) {
    let width = 30;
    let filled = (width * percent as usize) / 100;
    let empty = width - filled;
    let mut bar = String::new();
    for i in 0..filled {
        let char = match i % 4 {
            0 => '█',
            1 => '▓',
            2 => '▒',
            3 => '░',
            _ => '█',
        };
        bar.push_str(&char.cyan().to_string());
    }
    for _ in 0..empty {
        bar.push(' ');
    }
    let percent_str = format!("{}%", percent);
    println!("{} {}", bar, percent_str.bright_cyan().bold());
}

pub fn render_animated_progress(
    target: u8,
    style: &str,
    from: Option<&str>,
    to: Option<&str>,
    duration_ms: u64,
) {
    let target = target.min(100);

    // If not a TTY (piped/captured), just show final result
    if !io::stdout().is_terminal() {
        render(target, style, from, to);
        return;
    }

    let steps = 30;
    let step_delay = Duration::from_millis(duration_ms / steps as u64);

    for i in 0..=steps {
        let current = (i * target as u32 / steps) as u8;

        // Build the progress bar string
        let bar = if from.is_some() || to.is_some() {
            let start = from.map(parse_color).unwrap_or((63, 185, 80));
            let end = to.map(parse_color).unwrap_or((88, 166, 255));
            build_custom_gradient_bar(current, start, end)
        } else {
            match style {
                "modern" => build_modern_bar(current),
                _ => build_modern_bar(current),
            }
        };

        // Use \r to return to start of line for in-place updates
        print!("\r{}", bar);
        io::stdout().flush().unwrap();
        thread::sleep(step_delay);
    }
    println!(); // Final newline when done
}

fn build_custom_gradient_bar(percent: u8, start: (u8, u8, u8), end: (u8, u8, u8)) -> String {
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
    let percent_str = format!(
        "\x1b[1m\x1b[38;2;{};{};{}m{:>3}%\x1b[0m",
        end.0, end.1, end.2, percent
    );
    format!("{} {}", bar, percent_str)
}

fn build_modern_bar(percent: u8) -> String {
    let width = 30;
    let filled = (width * percent as usize) / 100;
    let empty = width - filled;
    let mut bar = String::new();

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
    let percent_str = format!("\x1b[1m\x1b[38;2;88;166;255m{:>3}%\x1b[0m", percent);
    format!("{} {}", bar, percent_str)
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_progress_style_from_str() {
        assert_eq!(
            "gradient".parse::<ProgressStyle>().unwrap(),
            ProgressStyle::Gradient
        );
        assert_eq!(
            "blocks".parse::<ProgressStyle>().unwrap(),
            ProgressStyle::Blocks
        );
        assert_eq!(
            "MODERN".parse::<ProgressStyle>().unwrap(),
            ProgressStyle::Modern
        );
        assert_eq!(
            "invalid".parse::<ProgressStyle>().unwrap(),
            ProgressStyle::Gradient
        );
    }

    #[test]
    fn test_progress_state_new() {
        let progress = ProgressState::new(75);
        assert_eq!(progress.target(), 75);
        assert_eq!(progress.current(), 0);
        assert!(!progress.is_finished());
    }

    #[test]
    fn test_progress_state_at() {
        let progress = ProgressState::at(50);
        assert_eq!(progress.target(), 50);
        assert_eq!(progress.current(), 50);
        assert!(progress.is_finished());
    }

    #[test]
    fn test_progress_state_clamps_to_100() {
        let progress = ProgressState::new(150);
        assert_eq!(progress.target(), 100);
    }

    #[test]
    fn test_progress_set_value() {
        let mut progress = ProgressState::new(100);
        progress.set_value(50);
        assert_eq!(progress.current(), 50);
    }

    #[test]
    fn test_progress_set_target() {
        let mut progress = ProgressState::at(50);
        // When setting a higher target, progress is no longer finished
        progress.set_target(100, false);
        assert_eq!(progress.target(), 100);
        assert_eq!(progress.current(), 50);
        assert!(!progress.is_finished());

        // When setting equal or lower target, it remains finished
        progress.set_target(50, false);
        assert!(progress.is_finished());
    }

    #[test]
    fn test_progress_reset() {
        let mut progress = ProgressState::at(100);
        progress.reset();
        assert_eq!(progress.current(), 0);
        assert!(!progress.is_finished());
    }

    #[test]
    fn test_progress_with_style() {
        let progress = ProgressState::new(50).with_style(ProgressStyle::Blocks);
        assert_eq!(progress.style(), ProgressStyle::Blocks);
    }

    #[test]
    fn test_progress_with_width() {
        let progress = ProgressState::new(50).with_width(20);
        assert_eq!(progress.width(), 20);
    }

    #[test]
    fn test_rgb_lerp() {
        let black = Rgb::new(0, 0, 0);
        let white = Rgb::new(255, 255, 255);

        let mid = black.lerp(&white, 0.5);
        assert_eq!(mid.r, 127);
        assert_eq!(mid.g, 127);
        assert_eq!(mid.b, 127);
    }

    #[test]
    fn test_rgb_to_color() {
        let rgb = Rgb::new(100, 150, 200);
        let color = rgb.to_color();
        assert_eq!(color, Color::Rgb(100, 150, 200));
    }

    #[test]
    fn test_filled_width_calculation() {
        let progress = ProgressState::at(50).with_width(30);
        assert_eq!(progress.filled_width(), 15);

        let progress2 = ProgressState::at(100).with_width(30);
        assert_eq!(progress2.filled_width(), 30);

        let progress3 = ProgressState::at(0).with_width(30);
        assert_eq!(progress3.filled_width(), 0);
    }

    #[test]
    fn test_progress_default() {
        let progress = ProgressState::default();
        assert_eq!(progress.target(), 100);
        assert_eq!(progress.current(), 0);
    }

    // Widget rendering tests
    #[test]
    fn test_progress_widget_render() {
        let progress = ProgressState::at(50).with_width(20);
        let area = Rect::new(0, 0, 30, 1);
        let mut buf = Buffer::empty(area);

        (&progress).render(area, &mut buf);

        // Check that there's content in the buffer
        let cell = buf.cell((0, 0)).unwrap();
        assert_eq!(cell.symbol(), "█");
    }

    #[test]
    fn test_progress_widget_empty_area() {
        let progress = ProgressState::at(50);
        let area = Rect::new(0, 0, 0, 0);
        let mut buf = Buffer::empty(Rect::new(0, 0, 10, 1));

        // Should not panic with empty area
        (&progress).render(area, &mut buf);
    }
}
