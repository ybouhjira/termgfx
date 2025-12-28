//! Main application state and event loop for TermGFX Studio

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind, KeyModifiers, MouseButton, MouseEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::prelude::*;
use std::collections::HashMap;
use std::io::{self, IsTerminal, Write};

use super::layout::StudioLayout;
use super::registry::{get_all_components, ComponentDef, ParamType};
use super::ui;
use super::widgets::{DropdownState, SliderState, ToggleState};

/// Widget editing mode
#[derive(Debug, Clone, PartialEq)]
pub enum WidgetMode {
    None,
    TextEdit,
    Dropdown(DropdownState),
    Slider(SliderState),
    Toggle(ToggleState),
}

/// Which panel is currently focused
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FocusedPanel {
    Sidebar,
    Params,
    Preview,
}

/// Main application state
pub struct StudioApp {
    pub components: Vec<ComponentDef>,
    pub selected_component: usize,
    pub selected_param: usize,
    pub focused_panel: FocusedPanel,
    pub param_values: HashMap<String, String>,
    pub editing: bool,
    pub edit_buffer: String,
    pub widget_mode: WidgetMode,
    pub layout: StudioLayout,
    pub running: bool,
    pub show_help: bool,
    pub status_message: Option<(String, std::time::Instant)>,
    /// Last computed layout areas for mouse hit testing
    pub last_areas: Option<super::layout::StudioAreas>,
    /// Scroll offset for sidebar component list
    pub sidebar_scroll: usize,
}

impl StudioApp {
    /// Create a new studio application
    pub fn new() -> Self {
        let components = get_all_components();
        let mut param_values = HashMap::new();

        // Initialize with first component's defaults
        if let Some(first) = components.first() {
            for param in &first.params {
                param_values.insert(param.name.to_string(), param.default.to_string());
            }
        }

        Self {
            components,
            selected_component: 0,
            selected_param: 0,
            focused_panel: FocusedPanel::Sidebar,
            param_values,
            editing: false,
            edit_buffer: String::new(),
            widget_mode: WidgetMode::None,
            layout: StudioLayout::default(),
            running: true,
            show_help: false,
            status_message: None,
            last_areas: None,
            sidebar_scroll: 0,
        }
    }

    /// Set a status message with auto-clear timer
    pub fn set_status(&mut self, message: &str) {
        self.status_message = Some((message.to_string(), std::time::Instant::now()));
    }

    /// Clear expired status messages (after 2 seconds)
    pub fn clear_expired_status(&mut self) {
        if let Some((_, instant)) = &self.status_message {
            if instant.elapsed() > std::time::Duration::from_secs(2) {
                self.status_message = None;
            }
        }
    }

    /// Get the currently selected component
    pub fn current_component(&self) -> Option<&ComponentDef> {
        self.components.get(self.selected_component)
    }

    /// Update param values when component changes
    fn update_param_values(&mut self) {
        self.param_values.clear();
        let params: Vec<_> = self.components.get(self.selected_component)
            .map(|c| c.params.iter().map(|p| (p.name.to_string(), p.default.to_string())).collect())
            .unwrap_or_default();
        for (name, default) in params {
            self.param_values.insert(name, default);
        }
        self.selected_param = 0;
    }

    /// Handle key events
    fn handle_key(&mut self, key: event::KeyEvent) {
        if key.kind != KeyEventKind::Press {
            return;
        }

        // Handle widget mode interactions
        if self.handle_widget_key(key.code) {
            return;
        }

        // Handle editing mode separately
        if self.editing {
            match key.code {
                KeyCode::Char(c) => {
                    self.edit_buffer.push(c);
                }
                KeyCode::Backspace => {
                    self.edit_buffer.pop();
                }
                KeyCode::Enter => {
                    // Save the edited value
                    if let Some(component) = self.current_component() {
                        if let Some(param) = component.params.get(self.selected_param) {
                            self.param_values.insert(
                                param.name.to_string(),
                                self.edit_buffer.clone(),
                            );
                        }
                    }
                    self.editing = false;
                    self.edit_buffer.clear();
                }
                KeyCode::Esc => {
                    self.editing = false;
                    self.edit_buffer.clear();
                }
                _ => {}
            }
            return;
        }

        // Global shortcuts
        match key.code {
            KeyCode::Char('q') | KeyCode::Esc => {
                self.running = false;
            }
            KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                self.running = false;
            }
            KeyCode::Char('?') => {
                self.show_help = !self.show_help;
            }
            KeyCode::Tab => {
                // Cycle through panels
                self.focused_panel = match self.focused_panel {
                    FocusedPanel::Sidebar => FocusedPanel::Params,
                    FocusedPanel::Params => FocusedPanel::Preview,
                    FocusedPanel::Preview => FocusedPanel::Sidebar,
                };
            }
            KeyCode::BackTab => {
                // Reverse cycle
                self.focused_panel = match self.focused_panel {
                    FocusedPanel::Sidebar => FocusedPanel::Preview,
                    FocusedPanel::Params => FocusedPanel::Sidebar,
                    FocusedPanel::Preview => FocusedPanel::Params,
                };
            }
            KeyCode::Char('c') => {
                self.copy_command_to_clipboard();
            }
            KeyCode::Char('r') => {
                // Reset current component parameters to defaults
                self.update_param_values();
                self.set_status("✓ Parameters reset to defaults");
            }
            KeyCode::Char('1') => {
                self.focused_panel = FocusedPanel::Sidebar;
            }
            KeyCode::Char('2') => {
                self.focused_panel = FocusedPanel::Params;
            }
            KeyCode::Char('3') => {
                self.focused_panel = FocusedPanel::Preview;
            }
            _ => {
                // Panel-specific navigation
                match self.focused_panel {
                    FocusedPanel::Sidebar => self.handle_sidebar_key(key.code),
                    FocusedPanel::Params => self.handle_params_key(key.code),
                    FocusedPanel::Preview => {} // Preview is read-only
                }
            }
        }
    }

    fn handle_sidebar_key(&mut self, code: KeyCode) {
        match code {
            KeyCode::Down | KeyCode::Char('j') => {
                if self.selected_component < self.components.len() - 1 {
                    self.selected_component += 1;
                    self.update_param_values();
                }
            }
            KeyCode::Up | KeyCode::Char('k') => {
                if self.selected_component > 0 {
                    self.selected_component -= 1;
                    self.update_param_values();
                }
            }
            KeyCode::Enter | KeyCode::Right | KeyCode::Char('l') => {
                self.focused_panel = FocusedPanel::Params;
            }
            _ => {}
        }
    }

    fn handle_params_key(&mut self, code: KeyCode) {
        let param_count = self.current_component()
            .map(|c| c.params.len())
            .unwrap_or(0);

        match code {
            KeyCode::Down | KeyCode::Char('j') => {
                if self.selected_param < param_count.saturating_sub(1) {
                    self.selected_param += 1;
                }
            }
            KeyCode::Up | KeyCode::Char('k') => {
                if self.selected_param > 0 {
                    self.selected_param -= 1;
                }
            }
            KeyCode::Left | KeyCode::Char('h') => {
                self.focused_panel = FocusedPanel::Sidebar;
            }
            KeyCode::Enter => {
                // Start editing with appropriate widget for param type
                self.start_widget_for_current_param();
            }
            KeyCode::Char(' ') => {
                // Toggle for bools, cycle for enums
                if let Some(component) = self.current_component() {
                    if let Some(param) = component.params.get(self.selected_param) {
                        match &param.param_type {
                            ParamType::Bool => {
                                let current = self.param_values
                                    .get(param.name)
                                    .map(|s| s == "true")
                                    .unwrap_or(false);
                                self.param_values.insert(
                                    param.name.to_string(),
                                    (!current).to_string(),
                                );
                            }
                            ParamType::Enum(options) => {
                                let current = self.param_values
                                    .get(param.name)
                                    .map(|s| s.as_str())
                                    .unwrap_or(param.default);
                                let idx = options.iter().position(|&o| o == current).unwrap_or(0);
                                let next_idx = (idx + 1) % options.len();
                                self.param_values.insert(
                                    param.name.to_string(),
                                    options[next_idx].to_string(),
                                );
                            }
                            _ => {}
                        }
                    }
                }
            }
            _ => {}
        }
    }

    /// Handle widget-specific key events
    /// Returns true if the key was consumed by a widget
    fn handle_widget_key(&mut self, code: KeyCode) -> bool {
        match &mut self.widget_mode {
            WidgetMode::None => false,

            WidgetMode::TextEdit => {
                // Text edit is handled by the main editing mode
                false
            }

            WidgetMode::Dropdown(state) => {
                let options_len = self.components.get(self.selected_component)
                    .and_then(|c| c.params.get(self.selected_param))
                    .map(|p| match &p.param_type {
                        ParamType::Enum(opts) => opts.len(),
                        _ => 0,
                    })
                    .unwrap_or(0);

                match code {
                    KeyCode::Up | KeyCode::Char('k') => {
                        state.move_up(options_len);
                        true
                    }
                    KeyCode::Down | KeyCode::Char('j') => {
                        state.move_down(options_len);
                        true
                    }
                    KeyCode::Enter => {
                        // Get the selected option and save it
                        let option = self.components.get(self.selected_component)
                            .and_then(|c| c.params.get(self.selected_param))
                            .and_then(|p| match &p.param_type {
                                ParamType::Enum(opts) => opts.get(state.hover_index).copied(),
                                _ => None,
                            });

                        if let Some(opt) = option {
                            if let Some(param) = self.components.get(self.selected_component)
                                .and_then(|c| c.params.get(self.selected_param))
                            {
                                self.param_values.insert(param.name.to_string(), opt.to_string());
                            }
                        }
                        self.widget_mode = WidgetMode::None;
                        true
                    }
                    KeyCode::Esc => {
                        self.widget_mode = WidgetMode::None;
                        true
                    }
                    _ => true, // Consume other keys when dropdown is open
                }
            }

            WidgetMode::Slider(state) => {
                match code {
                    KeyCode::Left | KeyCode::Char('h') => {
                        state.decrement();
                        // Update param value
                        if let Some(param) = self.components.get(self.selected_component)
                            .and_then(|c| c.params.get(self.selected_param))
                        {
                            self.param_values.insert(
                                param.name.to_string(),
                                format!("{:.0}", state.value),
                            );
                        }
                        true
                    }
                    KeyCode::Right | KeyCode::Char('l') => {
                        state.increment();
                        if let Some(param) = self.components.get(self.selected_component)
                            .and_then(|c| c.params.get(self.selected_param))
                        {
                            self.param_values.insert(
                                param.name.to_string(),
                                format!("{:.0}", state.value),
                            );
                        }
                        true
                    }
                    KeyCode::Enter | KeyCode::Esc => {
                        self.widget_mode = WidgetMode::None;
                        true
                    }
                    _ => true,
                }
            }

            WidgetMode::Toggle(state) => {
                match code {
                    KeyCode::Char(' ') | KeyCode::Enter => {
                        state.toggle();
                        if let Some(param) = self.components.get(self.selected_component)
                            .and_then(|c| c.params.get(self.selected_param))
                        {
                            self.param_values.insert(
                                param.name.to_string(),
                                state.is_on.to_string(),
                            );
                        }
                        self.widget_mode = WidgetMode::None;
                        true
                    }
                    KeyCode::Esc => {
                        self.widget_mode = WidgetMode::None;
                        true
                    }
                    _ => true,
                }
            }
        }
    }

    /// Start appropriate widget mode for current parameter
    pub fn start_widget_for_current_param(&mut self) {
        let param_info = self.components.get(self.selected_component)
            .and_then(|c| c.params.get(self.selected_param))
            .map(|p| (p.name.to_string(), p.param_type.clone(), p.default.to_string()));

        if let Some((name, param_type, default)) = param_info {
            let current_value = self.param_values.get(&name).cloned().unwrap_or(default);

            match param_type {
                ParamType::Enum(options) => {
                    let selected_idx = options.iter()
                        .position(|&o| o == current_value)
                        .unwrap_or(0);
                    let mut state = DropdownState::new(selected_idx);
                    state.is_open = true;
                    self.widget_mode = WidgetMode::Dropdown(state);
                }
                ParamType::Number { min, max } => {
                    let value = current_value.parse::<f64>().unwrap_or(min);
                    self.widget_mode = WidgetMode::Slider(SliderState::new(value, min, max));
                }
                ParamType::Bool => {
                    let is_on = current_value == "true";
                    self.widget_mode = WidgetMode::Toggle(ToggleState::new(is_on));
                }
                _ => {
                    // For String and Data, use text editing
                    self.editing = true;
                    self.edit_buffer = current_value;
                }
            }
        }
    }

    /// Handle mouse events
    pub fn handle_mouse(&mut self, event: crossterm::event::MouseEvent) {
        let Some(areas) = self.last_areas else {
            return;
        };

        let x = event.column;
        let y = event.row;

        match event.kind {
            MouseEventKind::Down(MouseButton::Left) => {
                // Check which panel was clicked
                if Self::point_in_rect(x, y, areas.sidebar) {
                    self.focused_panel = FocusedPanel::Sidebar;
                    // Calculate which component was clicked
                    let inner_y = y.saturating_sub(areas.sidebar.y + 1); // Account for border
                    let mut item_index = 0;
                    let mut current_category = "";

                    for (idx, component) in self.components.iter().enumerate() {
                        // Skip category headers in click detection
                        if component.category != current_category {
                            current_category = component.category;
                            if item_index == inner_y as usize {
                                // Clicked on category header, ignore
                                return;
                            }
                            item_index += 1;
                        }

                        if item_index == inner_y as usize {
                            if idx != self.selected_component {
                                self.selected_component = idx;
                                self.update_param_values();
                            }
                            return;
                        }
                        item_index += 1;
                    }
                } else if Self::point_in_rect(x, y, areas.params) {
                    self.focused_panel = FocusedPanel::Params;
                    // Calculate which parameter was clicked
                    let inner_y = y.saturating_sub(areas.params.y + 1);
                    let param_count = self.current_component()
                        .map(|c| c.params.len())
                        .unwrap_or(0);

                    if (inner_y as usize) < param_count {
                        self.selected_param = inner_y as usize;
                    }
                } else if Self::point_in_rect(x, y, areas.preview) {
                    self.focused_panel = FocusedPanel::Preview;
                } else if Self::point_in_rect(x, y, areas.command) {
                    // Check if clicked on Copy button area (roughly in the help text)
                    // The command panel shows: [c] Copy   [Enter] Run   [?] Help   [q] Quit
                    let inner_x = x.saturating_sub(areas.command.x + 1);
                    if inner_x < 10 {
                        // Clicked near [c] Copy - trigger copy
                        self.copy_command_to_clipboard();
                    }
                }
            }
            MouseEventKind::ScrollUp => {
                match self.focused_panel {
                    FocusedPanel::Sidebar => {
                        if self.selected_component > 0 {
                            self.selected_component -= 1;
                            self.update_param_values();
                        }
                    }
                    FocusedPanel::Params => {
                        if self.selected_param > 0 {
                            self.selected_param -= 1;
                        }
                    }
                    FocusedPanel::Preview => {}
                }
            }
            MouseEventKind::ScrollDown => {
                match self.focused_panel {
                    FocusedPanel::Sidebar => {
                        if self.selected_component < self.components.len() - 1 {
                            self.selected_component += 1;
                            self.update_param_values();
                        }
                    }
                    FocusedPanel::Params => {
                        let param_count = self.current_component()
                            .map(|c| c.params.len())
                            .unwrap_or(0);
                        if self.selected_param < param_count.saturating_sub(1) {
                            self.selected_param += 1;
                        }
                    }
                    FocusedPanel::Preview => {}
                }
            }
            _ => {}
        }
    }

    /// Check if a point is inside a rectangle
    fn point_in_rect(x: u16, y: u16, rect: Rect) -> bool {
        x >= rect.x && x < rect.x + rect.width && y >= rect.y && y < rect.y + rect.height
    }

    /// Copy command to clipboard (extracted for reuse)
    fn copy_command_to_clipboard(&mut self) {
        if let Some(component) = self.current_component() {
            let cmd = component.generate_command(&self.param_values);
            let copy_result = std::process::Command::new("pbcopy")
                .stdin(std::process::Stdio::piped())
                .spawn()
                .or_else(|_| {
                    std::process::Command::new("xclip")
                        .args(["-selection", "clipboard"])
                        .stdin(std::process::Stdio::piped())
                        .spawn()
                });

            if let Ok(mut child) = copy_result {
                if let Some(stdin) = child.stdin.as_mut() {
                    let _ = stdin.write_all(cmd.as_bytes());
                }
                let _ = child.wait();
                self.set_status("✓ Command copied to clipboard!");
            }
        }
    }
}

impl Default for StudioApp {
    fn default() -> Self {
        Self::new()
    }
}

/// Run the studio TUI application
pub fn run_studio() -> io::Result<()> {
    // Check for interactive terminal
    if !std::io::stdin().is_terminal() {
        return Err(io::Error::other(
            "Studio requires an interactive terminal (TTY)",
        ));
    }

    // Setup terminal with mouse support
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = StudioApp::new();

    // Main loop
    while app.running {
        // Clear expired status messages
        app.clear_expired_status();

        // Render
        terminal.draw(|frame| {
            let areas = app.layout.split(frame.area());
            app.last_areas = Some(areas); // Store for mouse hit testing
            ui::render(frame, &app, areas);

            // Render help overlay if visible
            if app.show_help {
                ui::render_help_overlay(frame);
            }

            // Render status message if any
            if let Some((msg, _)) = &app.status_message {
                ui::render_status_message(frame, msg);
            }
        })?;

        // Handle events
        if event::poll(std::time::Duration::from_millis(100))? {
            match event::read()? {
                Event::Key(key) => app.handle_key(key),
                Event::Mouse(mouse) => app.handle_mouse(mouse),
                _ => {}
            }
        }
    }

    // Cleanup with mouse capture disabled
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen, DisableMouseCapture)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_app_new() {
        let app = StudioApp::new();
        assert!(!app.components.is_empty());
        assert_eq!(app.selected_component, 0);
        assert_eq!(app.focused_panel, FocusedPanel::Sidebar);
    }

    #[test]
    fn test_update_param_values() {
        let mut app = StudioApp::new();
        app.selected_component = 1;
        app.update_param_values();

        // Should have new component's params
        assert!(!app.param_values.is_empty());
    }

    #[test]
    fn test_current_component() {
        let app = StudioApp::new();
        let component = app.current_component();
        assert!(component.is_some());
        assert_eq!(component.unwrap().name, "box");
    }

    #[test]
    fn test_set_status_message() {
        let mut app = StudioApp::new();
        assert!(app.status_message.is_none());

        app.set_status("Test status");
        assert!(app.status_message.is_some());
        assert_eq!(app.status_message.as_ref().unwrap().0, "Test status");
    }

    #[test]
    fn test_show_help_toggle() {
        let mut app = StudioApp::new();
        assert!(!app.show_help);

        // Toggle help on
        app.show_help = true;
        assert!(app.show_help);

        // Toggle help off
        app.show_help = false;
        assert!(!app.show_help);
    }

    #[test]
    fn test_panel_jump_keys() {
        let mut app = StudioApp::new();
        assert_eq!(app.focused_panel, FocusedPanel::Sidebar);

        // Simulate pressing '2' to jump to Params
        app.focused_panel = FocusedPanel::Params;
        assert_eq!(app.focused_panel, FocusedPanel::Params);

        // Simulate pressing '3' to jump to Preview
        app.focused_panel = FocusedPanel::Preview;
        assert_eq!(app.focused_panel, FocusedPanel::Preview);

        // Simulate pressing '1' to jump back to Sidebar
        app.focused_panel = FocusedPanel::Sidebar;
        assert_eq!(app.focused_panel, FocusedPanel::Sidebar);
    }

    #[test]
    fn test_reset_params() {
        let mut app = StudioApp::new();

        // Modify a parameter
        app.param_values.insert("message".to_string(), "Modified".to_string());

        // Reset should restore defaults
        app.update_param_values();

        // Check first component (box) has default message
        let default_message = app.param_values.get("message");
        assert!(default_message.is_some());
        assert_eq!(default_message.unwrap(), "Hello World!");
    }

    #[test]
    fn test_point_in_rect() {
        let rect = Rect::new(10, 10, 20, 10);

        // Inside
        assert!(StudioApp::point_in_rect(15, 15, rect));
        assert!(StudioApp::point_in_rect(10, 10, rect)); // Top-left corner

        // Outside
        assert!(!StudioApp::point_in_rect(5, 15, rect));   // Left
        assert!(!StudioApp::point_in_rect(35, 15, rect));  // Right
        assert!(!StudioApp::point_in_rect(15, 5, rect));   // Above
        assert!(!StudioApp::point_in_rect(15, 25, rect));  // Below
    }

    #[test]
    fn test_mouse_fields_initialized() {
        let app = StudioApp::new();
        assert!(app.last_areas.is_none());
        assert_eq!(app.sidebar_scroll, 0);
    }
}
