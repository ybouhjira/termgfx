//! Stateful spinner component for animated loading indicators
//!
//! Provides a dual-mode spinner that works both as:
//! - Standalone TTY mode (blocking, backwards-compatible)
//! - Ratatui Widget (non-blocking, for integration with TUI apps)
//!
//! # Example (TTY mode)
//! ```ignore
//! let mut spinner = SpinnerState::new(SpinnerStyle::Dots);
//! spinner.run_blocking("Loading...", Duration::from_secs(3));
//! ```
//!
//! # Example (Ratatui mode)
//! ```ignore
//! let mut spinner = SpinnerState::new(SpinnerStyle::Dots);
//! // In your event loop:
//! if spinner.tick() {
//!     // Frame changed, need to redraw
//! }
//! // In your render function:
//! frame.render_widget(&spinner.widget("Loading..."), area);
//! ```

use crossterm::{
    cursor::{Hide, MoveToColumn, Show},
    terminal::{Clear, ClearType},
    ExecutableCommand,
};
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Style},
    text::Span,
    widgets::Widget,
};
use std::io::{stdout, IsTerminal, Write};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::{Duration, Instant};

/// Available spinner animation styles
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SpinnerStyle {
    #[default]
    Dots,
    Line,
    Arc,
    Bouncing,
    Clock,
    Circle,
    Bounce,
    Moon,
}

impl SpinnerStyle {
    /// Get the animation frames for this style
    pub fn frames(&self) -> &'static [&'static str] {
        match self {
            SpinnerStyle::Dots => &["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"],
            SpinnerStyle::Line => &["|", "/", "-", "\\"],
            SpinnerStyle::Arc => &["◜", "◠", "◝", "◞", "◡", "◟"],
            SpinnerStyle::Bouncing => &["⠁", "⠂", "⠄", "⠂"],
            SpinnerStyle::Clock => &[
                "🕐", "🕑", "🕒", "🕓", "🕔", "🕕", "🕖", "🕗", "🕘", "🕙", "🕚", "🕛",
            ],
            SpinnerStyle::Circle => &["◐", "◓", "◑", "◒"],
            SpinnerStyle::Bounce => &["⠁", "⠂", "⠄", "⡀", "⢀", "⠠", "⠐", "⠈"],
            SpinnerStyle::Moon => &["🌑", "🌒", "🌓", "🌔", "🌕", "🌖", "🌗", "🌘"],
        }
    }

    /// Default interval between frames
    pub fn default_interval(&self) -> Duration {
        Duration::from_millis(80)
    }
}

impl std::str::FromStr for SpinnerStyle {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s.to_lowercase().as_str() {
            "dots" => SpinnerStyle::Dots,
            "line" => SpinnerStyle::Line,
            "arc" => SpinnerStyle::Arc,
            "bouncing" => SpinnerStyle::Bouncing,
            "clock" => SpinnerStyle::Clock,
            "circle" => SpinnerStyle::Circle,
            "bounce" => SpinnerStyle::Bounce,
            "moon" => SpinnerStyle::Moon,
            _ => SpinnerStyle::Dots,
        })
    }
}

/// Stateful spinner that manages animation timing and frame advancement
#[derive(Debug, Clone)]
pub struct SpinnerState {
    /// The spinner animation style
    style: SpinnerStyle,
    /// Current frame index
    frame_index: usize,
    /// When the last frame update occurred
    last_tick: Instant,
    /// Time between frame updates
    interval: Duration,
    /// Optional color for the spinner
    color: Option<Color>,
}

impl SpinnerState {
    /// Create a new spinner with the given style
    pub fn new(style: SpinnerStyle) -> Self {
        Self {
            style,
            frame_index: 0,
            last_tick: Instant::now(),
            interval: style.default_interval(),
            color: Some(Color::Cyan),
        }
    }

    /// Create a new spinner from a style string
    pub fn from_style_str(style: &str) -> Self {
        Self::new(style.parse().unwrap_or_default())
    }

    /// Set the animation interval
    pub fn with_interval(mut self, interval: Duration) -> Self {
        self.interval = interval;
        self
    }

    /// Set the spinner color
    pub fn with_color(mut self, color: Color) -> Self {
        self.color = Some(color);
        self
    }

    /// Advance the animation if enough time has passed
    /// Returns true if the frame changed (indicating a redraw is needed)
    pub fn tick(&mut self) -> bool {
        if self.last_tick.elapsed() >= self.interval {
            let frames = self.style.frames();
            self.frame_index = (self.frame_index + 1) % frames.len();
            self.last_tick = Instant::now();
            true
        } else {
            false
        }
    }

    /// Force advance to the next frame (ignoring timing)
    pub fn advance(&mut self) {
        let frames = self.style.frames();
        self.frame_index = (self.frame_index + 1) % frames.len();
        self.last_tick = Instant::now();
    }

    /// Reset the spinner to its initial state
    pub fn reset(&mut self) {
        self.frame_index = 0;
        self.last_tick = Instant::now();
    }

    /// Get the current frame character
    pub fn current_frame(&self) -> &'static str {
        let frames = self.style.frames();
        frames[self.frame_index]
    }

    /// Get the current frame index
    pub fn frame_index(&self) -> usize {
        self.frame_index
    }

    /// Get the total number of frames
    pub fn frame_count(&self) -> usize {
        self.style.frames().len()
    }

    /// Get the spinner style
    pub fn style(&self) -> SpinnerStyle {
        self.style
    }

    /// Get the configured color
    pub fn color(&self) -> Option<Color> {
        self.color
    }

    /// Create a widget for rendering with a message
    pub fn widget<'a>(&'a self, message: &'a str) -> SpinnerWidget<'a> {
        SpinnerWidget {
            state: self,
            message,
        }
    }

    /// Run the spinner in blocking TTY mode (backwards-compatible)
    /// This is the standalone mode that owns the terminal output
    pub fn run_blocking(&mut self, message: &str, duration: Duration) {
        // If not a TTY (piped/captured), just print static and return
        if !stdout().is_terminal() {
            println!("{} {}", self.current_frame(), message);
            return;
        }

        let running = Arc::new(AtomicBool::new(true));
        let r = running.clone();

        // Set up Ctrl+C handler
        ctrlc::set_handler(move || {
            r.store(false, Ordering::SeqCst);
        })
        .expect("Error setting Ctrl-C handler");

        let mut stdout = stdout();

        // Hide cursor
        stdout.execute(Hide).unwrap();

        let start_time = Instant::now();

        while running.load(Ordering::SeqCst) && start_time.elapsed() < duration {
            // Move to beginning of line and clear it
            stdout.execute(MoveToColumn(0)).unwrap();
            stdout.execute(Clear(ClearType::CurrentLine)).unwrap();

            // Print spinner frame and message
            print!("{} {}", self.current_frame(), message);
            stdout.flush().unwrap();

            // Sleep for the interval
            thread::sleep(self.interval);
            self.advance();
        }

        // Clean up: clear line, show cursor
        stdout.execute(MoveToColumn(0)).unwrap();
        stdout.execute(Clear(ClearType::CurrentLine)).unwrap();
        stdout.execute(Show).unwrap();
        stdout.flush().unwrap();
    }

    /// Run the spinner indefinitely until Ctrl+C (backwards-compatible)
    pub fn run_indefinitely(&mut self, message: &str) {
        // If not a TTY, just print static and return
        if !stdout().is_terminal() {
            println!("{} {}", self.current_frame(), message);
            return;
        }

        let running = Arc::new(AtomicBool::new(true));
        let r = running.clone();

        ctrlc::set_handler(move || {
            r.store(false, Ordering::SeqCst);
        })
        .expect("Error setting Ctrl-C handler");

        let mut stdout = stdout();
        stdout.execute(Hide).unwrap();

        while running.load(Ordering::SeqCst) {
            stdout.execute(MoveToColumn(0)).unwrap();
            stdout.execute(Clear(ClearType::CurrentLine)).unwrap();
            print!("{} {}", self.current_frame(), message);
            stdout.flush().unwrap();
            thread::sleep(self.interval);
            self.advance();
        }

        stdout.execute(MoveToColumn(0)).unwrap();
        stdout.execute(Clear(ClearType::CurrentLine)).unwrap();
        stdout.execute(Show).unwrap();
        stdout.flush().unwrap();
    }
}

impl Default for SpinnerState {
    fn default() -> Self {
        Self::new(SpinnerStyle::default())
    }
}

/// Widget for rendering a spinner with a message
pub struct SpinnerWidget<'a> {
    state: &'a SpinnerState,
    message: &'a str,
}

impl<'a> SpinnerWidget<'a> {
    /// Create a new spinner widget
    pub fn new(state: &'a SpinnerState, message: &'a str) -> Self {
        Self { state, message }
    }
}

impl Widget for SpinnerWidget<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        if area.width == 0 || area.height == 0 {
            return;
        }

        let frame = self.state.current_frame();
        let style = match self.state.color {
            Some(color) => Style::default().fg(color),
            None => Style::default(),
        };

        // Render spinner frame
        let frame_span = Span::styled(frame, style);
        buf.set_span(area.x, area.y, &frame_span, area.width);

        // Render message after spinner (with gap)
        let frame_width = frame.chars().count() as u16;
        if area.width > frame_width + 1 {
            let message_x = area.x + frame_width + 1;
            let message_width = area.width - frame_width - 1;
            let message_span = Span::raw(self.message);
            buf.set_span(message_x, area.y, &message_span, message_width);
        }
    }
}

/// Implement Widget for &SpinnerState (renders just the spinner frame)
impl Widget for &SpinnerState {
    fn render(self, area: Rect, buf: &mut Buffer) {
        if area.width == 0 || area.height == 0 {
            return;
        }

        let frame = self.current_frame();
        let style = match self.color {
            Some(color) => Style::default().fg(color),
            None => Style::default(),
        };

        let span = Span::styled(frame, style);
        buf.set_span(area.x, area.y, &span, area.width);
    }
}

// ============================================================================
// Legacy API (backwards compatibility)
// ============================================================================

/// Get spinner frames for different styles (legacy function)
fn get_spinner_frames(style: &str) -> Vec<&'static str> {
    style
        .parse::<SpinnerStyle>()
        .unwrap_or_default()
        .frames()
        .to_vec()
}

/// Render an animated loading spinner (legacy API)
/// If duration is Some(n), the spinner auto-stops after n seconds
pub fn render(message: &str, style: &str, duration: Option<u64>) {
    let frames = get_spinner_frames(style);

    // If not a TTY (piped/captured), just print static message and return
    if !stdout().is_terminal() {
        println!("{} {}", frames[0], message);
        return;
    }

    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();

    // Set up Ctrl+C handler
    ctrlc::set_handler(move || {
        r.store(false, Ordering::SeqCst);
    })
    .expect("Error setting Ctrl-C handler");

    let mut stdout = stdout();

    // Hide cursor
    stdout.execute(Hide).unwrap();

    let mut frame_idx = 0;
    let start_time = Instant::now();
    let timeout = duration.map(Duration::from_secs);

    while running.load(Ordering::SeqCst) {
        // Check if duration exceeded
        if let Some(max_duration) = timeout {
            if start_time.elapsed() >= max_duration {
                break;
            }
        }

        // Move to beginning of line and clear it
        stdout.execute(MoveToColumn(0)).unwrap();
        stdout.execute(Clear(ClearType::CurrentLine)).unwrap();

        // Print spinner frame and message
        print!("{} {}", frames[frame_idx], message);
        stdout.flush().unwrap();

        // Next frame
        frame_idx = (frame_idx + 1) % frames.len();

        // Sleep between frames (80ms for smooth animation)
        thread::sleep(Duration::from_millis(80));
    }

    // Clean up: clear line, show cursor
    stdout.execute(MoveToColumn(0)).unwrap();
    stdout.execute(Clear(ClearType::CurrentLine)).unwrap();
    stdout.execute(Show).unwrap();
    stdout.flush().unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_spinner_style_frames() {
        let style = SpinnerStyle::Dots;
        let frames = style.frames();
        assert!(!frames.is_empty());
        assert!(frames.contains(&"⠋"));
    }

    #[test]
    fn test_spinner_style_from_str() {
        assert_eq!("dots".parse::<SpinnerStyle>().unwrap(), SpinnerStyle::Dots);
        assert_eq!("DOTS".parse::<SpinnerStyle>().unwrap(), SpinnerStyle::Dots);
        assert_eq!("moon".parse::<SpinnerStyle>().unwrap(), SpinnerStyle::Moon);
        assert_eq!(
            "invalid".parse::<SpinnerStyle>().unwrap(),
            SpinnerStyle::Dots
        );
    }

    #[test]
    fn test_spinner_state_creation() {
        let spinner = SpinnerState::new(SpinnerStyle::Dots);
        assert_eq!(spinner.frame_index(), 0);
        assert_eq!(spinner.style(), SpinnerStyle::Dots);
    }

    #[test]
    fn test_spinner_state_advance() {
        let mut spinner = SpinnerState::new(SpinnerStyle::Dots);
        assert_eq!(spinner.frame_index(), 0);

        spinner.advance();
        assert_eq!(spinner.frame_index(), 1);

        // Advance to wrap around
        for _ in 0..10 {
            spinner.advance();
        }
        assert_eq!(spinner.frame_index(), 1); // 10 + 1 = 11, 11 % 10 = 1
    }

    #[test]
    fn test_spinner_state_reset() {
        let mut spinner = SpinnerState::new(SpinnerStyle::Dots);
        spinner.advance();
        spinner.advance();
        assert_eq!(spinner.frame_index(), 2);

        spinner.reset();
        assert_eq!(spinner.frame_index(), 0);
    }

    #[test]
    fn test_spinner_current_frame() {
        let spinner = SpinnerState::new(SpinnerStyle::Dots);
        assert_eq!(spinner.current_frame(), "⠋");
    }

    #[test]
    fn test_spinner_frame_count() {
        let spinner = SpinnerState::new(SpinnerStyle::Dots);
        assert_eq!(spinner.frame_count(), 10);

        let spinner = SpinnerState::new(SpinnerStyle::Line);
        assert_eq!(spinner.frame_count(), 4);
    }

    #[test]
    fn test_spinner_with_color() {
        let spinner = SpinnerState::new(SpinnerStyle::Dots).with_color(Color::Green);
        assert_eq!(spinner.color(), Some(Color::Green));
    }

    #[test]
    fn test_spinner_with_interval() {
        let spinner =
            SpinnerState::new(SpinnerStyle::Dots).with_interval(Duration::from_millis(100));
        assert_eq!(spinner.interval, Duration::from_millis(100));
    }

    #[test]
    fn test_spinner_tick_no_time_elapsed() {
        let mut spinner = SpinnerState::new(SpinnerStyle::Dots);
        // Immediately after creation, tick should return false
        // (not enough time has elapsed)
        let changed = spinner.tick();
        assert!(!changed);
        assert_eq!(spinner.frame_index(), 0);
    }

    #[test]
    fn test_spinner_widget_creation() {
        let spinner = SpinnerState::new(SpinnerStyle::Dots);
        let widget = spinner.widget("Loading...");
        assert_eq!(widget.message, "Loading...");
    }

    #[test]
    fn test_spinner_default() {
        let spinner = SpinnerState::default();
        assert_eq!(spinner.style(), SpinnerStyle::Dots);
        assert_eq!(spinner.frame_index(), 0);
    }

    #[test]
    fn test_spinner_from_style_str() {
        let spinner = SpinnerState::from_style_str("moon");
        assert_eq!(spinner.style(), SpinnerStyle::Moon);
    }

    // Widget rendering tests
    #[test]
    fn test_spinner_widget_render() {
        let spinner = SpinnerState::new(SpinnerStyle::Dots);
        let area = Rect::new(0, 0, 20, 1);
        let mut buf = Buffer::empty(area);

        let widget = spinner.widget("Test");
        widget.render(area, &mut buf);

        // Check that the first character is the spinner frame
        let cell = buf.cell((0, 0)).unwrap();
        assert_eq!(cell.symbol(), "⠋");
    }

    #[test]
    fn test_spinner_state_as_widget() {
        let spinner = SpinnerState::new(SpinnerStyle::Dots);
        let area = Rect::new(0, 0, 10, 1);
        let mut buf = Buffer::empty(area);

        // Render spinner state directly as widget
        (&spinner).render(area, &mut buf);

        let cell = buf.cell((0, 0)).unwrap();
        assert_eq!(cell.symbol(), "⠋");
    }
}
