//! Main application state and event loop for TermGFX Studio

use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind, KeyModifiers},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::prelude::*;
use std::collections::HashMap;
use std::io::{self, IsTerminal, Write};

use super::layout::StudioLayout;
use super::registry::{get_all_components, ComponentDef, ParamType};
use super::ui;

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
    pub layout: StudioLayout,
    pub running: bool,
    pub show_help: bool,
    pub copied_message: Option<String>,
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
            layout: StudioLayout::default(),
            running: true,
            show_help: false,
            copied_message: None,
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
                // Copy command to clipboard
                if let Some(component) = self.current_component() {
                    let cmd = component.generate_command(&self.param_values);
                    // Try to copy to clipboard using pbcopy on macOS
                    if let Ok(mut child) = std::process::Command::new("pbcopy")
                        .stdin(std::process::Stdio::piped())
                        .spawn()
                    {
                        if let Some(stdin) = child.stdin.as_mut() {
                            let _ = stdin.write_all(cmd.as_bytes());
                        }
                        let _ = child.wait();
                        self.copied_message = Some("Copied!".to_string());
                    }
                }
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
                // Start editing the current parameter
                let param_info = self.components.get(self.selected_component)
                    .and_then(|c| c.params.get(self.selected_param))
                    .map(|p| (p.name.to_string(), p.default.to_string()));

                if let Some((name, default)) = param_info {
                    self.editing = true;
                    self.edit_buffer = self.param_values
                        .get(&name)
                        .cloned()
                        .unwrap_or(default);
                }
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

    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = StudioApp::new();

    // Main loop
    while app.running {
        // Render
        terminal.draw(|frame| {
            let areas = app.layout.split(frame.area());
            ui::render(frame, &app, areas);
        })?;

        // Handle events
        if event::poll(std::time::Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                app.handle_key(key);
            }
        }

        // Clear copied message after a short time
        if app.copied_message.is_some() {
            app.copied_message = None;
        }
    }

    // Cleanup
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;

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
}
