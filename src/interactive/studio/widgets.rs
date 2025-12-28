//! Custom widgets for TermGFX Studio parameter editing
//!
//! Provides enhanced UX widgets beyond basic text input:
//! - Dropdown: Select from enum values with visual menu
//! - Slider: Adjust numeric values with visual bar
//! - Toggle: Boolean toggle with visual indicator

use ratatui::{
    prelude::*,
    widgets::{Block, Borders, Clear},
};

// ============================================================================
// Dropdown Widget
// ============================================================================

/// State for the dropdown widget
#[derive(Debug, Default, Clone, PartialEq)]
pub struct DropdownState {
    pub is_open: bool,
    pub selected_index: usize,
    pub hover_index: usize,
}

impl DropdownState {
    pub fn new(selected_index: usize) -> Self {
        Self {
            is_open: false,
            selected_index,
            hover_index: selected_index,
        }
    }

    pub fn toggle(&mut self) {
        self.is_open = !self.is_open;
        if self.is_open {
            self.hover_index = self.selected_index;
        }
    }

    pub fn select_current(&mut self) {
        self.selected_index = self.hover_index;
        self.is_open = false;
    }

    pub fn move_up(&mut self, max: usize) {
        if self.hover_index > 0 {
            self.hover_index -= 1;
        } else {
            self.hover_index = max.saturating_sub(1);
        }
    }

    pub fn move_down(&mut self, max: usize) {
        if self.hover_index < max.saturating_sub(1) {
            self.hover_index += 1;
        } else {
            self.hover_index = 0;
        }
    }
}

/// Dropdown widget for selecting from a list of options
pub struct Dropdown<'a> {
    options: &'a [&'a str],
    label: &'a str,
    focused: bool,
}

impl<'a> Dropdown<'a> {
    pub fn new(options: &'a [&'a str], label: &'a str) -> Self {
        Self {
            options,
            label,
            focused: false,
        }
    }

    pub fn focused(mut self, focused: bool) -> Self {
        self.focused = focused;
        self
    }
}

impl<'a> StatefulWidget for Dropdown<'a> {
    type State = DropdownState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        if area.width < 10 || area.height < 1 {
            return;
        }

        // Render the label
        let label_width = self.label.len() as u16 + 2;
        let label_area = Rect::new(area.x, area.y, label_width.min(area.width), 1);
        let label_style = Style::default().fg(Color::Cyan);
        buf.set_string(label_area.x, label_area.y, self.label, label_style);
        buf.set_string(label_area.right(), label_area.y, ": ", Style::default());

        // Calculate dropdown button area
        let button_x = label_area.right() + 2;
        let button_width = area.width.saturating_sub(button_x - area.x).min(20);
        let button_area = Rect::new(button_x, area.y, button_width, 1);

        // Get selected option
        let selected_text = self
            .options
            .get(state.selected_index)
            .copied()
            .unwrap_or("");

        // Render button
        let button_style = if self.focused {
            Style::default().fg(Color::Yellow).bold()
        } else {
            Style::default().fg(Color::White)
        };

        let arrow = if state.is_open { "▲" } else { "▼" };
        let button_text = format!("[{} {}]", selected_text, arrow);
        buf.set_string(button_area.x, button_area.y, &button_text, button_style);

        // Render dropdown menu if open
        if state.is_open {
            let menu_height = (self.options.len() as u16).min(8);
            let menu_width = button_width + 2;

            // Position below button
            let menu_y = button_area.y + 1;
            let menu_area = Rect::new(button_area.x, menu_y, menu_width, menu_height + 2);

            // Clear area behind menu
            Clear.render(menu_area, buf);

            // Draw border
            let block = Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Cyan));
            block.render(menu_area, buf);

            // Render options
            let inner = Rect::new(menu_area.x + 1, menu_area.y + 1, menu_width - 2, menu_height);
            for (i, option) in self.options.iter().enumerate().take(menu_height as usize) {
                let y = inner.y + i as u16;
                let is_hover = i == state.hover_index;
                let is_selected = i == state.selected_index;

                let style = if is_hover {
                    Style::default().bg(Color::Blue).fg(Color::White)
                } else if is_selected {
                    Style::default().fg(Color::Green)
                } else {
                    Style::default()
                };

                let marker = if is_selected { "◀" } else { " " };
                let text = format!(" {:<width$}{}", option, marker, width = (inner.width - 2) as usize);
                buf.set_string(inner.x, y, &text, style);
            }
        }
    }
}

// ============================================================================
// Slider Widget
// ============================================================================

/// State for the slider widget
#[derive(Debug, Clone, PartialEq)]
pub struct SliderState {
    pub value: f64,
    pub min: f64,
    pub max: f64,
    pub step: f64,
}

impl SliderState {
    pub fn new(value: f64, min: f64, max: f64) -> Self {
        Self {
            value: value.clamp(min, max),
            min,
            max,
            step: (max - min) / 20.0,
        }
    }

    pub fn increment(&mut self) {
        self.value = (self.value + self.step).min(self.max);
    }

    pub fn decrement(&mut self) {
        self.value = (self.value - self.step).max(self.min);
    }

    pub fn set_value(&mut self, value: f64) {
        self.value = value.clamp(self.min, self.max);
    }

    pub fn percentage(&self) -> f64 {
        if self.max == self.min {
            0.0
        } else {
            (self.value - self.min) / (self.max - self.min)
        }
    }
}

/// Slider widget for adjusting numeric values
pub struct Slider<'a> {
    label: &'a str,
    focused: bool,
    width: u16,
}

impl<'a> Slider<'a> {
    pub fn new(label: &'a str) -> Self {
        Self {
            label,
            focused: false,
            width: 20,
        }
    }

    pub fn focused(mut self, focused: bool) -> Self {
        self.focused = focused;
        self
    }

    pub fn width(mut self, width: u16) -> Self {
        self.width = width;
        self
    }
}

impl<'a> StatefulWidget for Slider<'a> {
    type State = SliderState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        if area.width < 15 || area.height < 1 {
            return;
        }

        // Render label
        let label_width = self.label.len() as u16 + 2;
        let label_style = Style::default().fg(Color::Cyan);
        buf.set_string(area.x, area.y, self.label, label_style);
        buf.set_string(area.x + label_width - 2, area.y, ": ", Style::default());

        // Calculate slider area
        let slider_x = area.x + label_width;
        let available_width = area.width.saturating_sub(label_width);
        let slider_width = self.width.min(available_width);

        // Value display
        let value_str = format!("{:.0}", state.value);
        let value_width = value_str.len() as u16 + 2;

        // Track area
        let track_width = slider_width.saturating_sub(value_width + 4);
        if track_width < 5 {
            return;
        }

        // Render value
        let value_style = if self.focused {
            Style::default().fg(Color::Yellow).bold()
        } else {
            Style::default().fg(Color::White)
        };
        buf.set_string(slider_x, area.y, &format!("[{}] ", value_str), value_style);

        // Render track
        let track_x = slider_x + value_width + 3;
        let filled_width = ((track_width as f64) * state.percentage()) as u16;
        let empty_width = track_width.saturating_sub(filled_width);

        let filled_style = Style::default().fg(Color::Green);
        let empty_style = Style::default().fg(Color::DarkGray);
        let knob_style = if self.focused {
            Style::default().fg(Color::Yellow).bold()
        } else {
            Style::default().fg(Color::White)
        };

        // Draw track
        let filled_chars = "─".repeat(filled_width as usize);
        let empty_chars = "─".repeat(empty_width.saturating_sub(1) as usize);

        buf.set_string(track_x, area.y, &filled_chars, filled_style);
        buf.set_string(track_x + filled_width, area.y, "●", knob_style);
        buf.set_string(track_x + filled_width + 1, area.y, &empty_chars, empty_style);
    }
}

// ============================================================================
// Toggle Widget
// ============================================================================

/// State for the toggle widget
#[derive(Debug, Default, Clone, PartialEq)]
pub struct ToggleState {
    pub is_on: bool,
}

impl ToggleState {
    pub fn new(is_on: bool) -> Self {
        Self { is_on }
    }

    pub fn toggle(&mut self) {
        self.is_on = !self.is_on;
    }
}

/// Toggle widget for boolean values
pub struct Toggle<'a> {
    label: &'a str,
    focused: bool,
}

impl<'a> Toggle<'a> {
    pub fn new(label: &'a str) -> Self {
        Self {
            label,
            focused: false,
        }
    }

    pub fn focused(mut self, focused: bool) -> Self {
        self.focused = focused;
        self
    }
}

impl<'a> StatefulWidget for Toggle<'a> {
    type State = ToggleState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        if area.width < 15 || area.height < 1 {
            return;
        }

        // Render label
        let label_width = self.label.len() as u16 + 2;
        let label_style = Style::default().fg(Color::Cyan);
        buf.set_string(area.x, area.y, self.label, label_style);
        buf.set_string(area.x + label_width - 2, area.y, ": ", Style::default());

        // Render toggle
        let toggle_x = area.x + label_width;
        let (toggle_text, text_style) = if state.is_on {
            let style = if self.focused {
                Style::default().fg(Color::Green).bold()
            } else {
                Style::default().fg(Color::Green)
            };
            ("[●○] ON ", style)
        } else {
            let style = if self.focused {
                Style::default().fg(Color::Red).bold()
            } else {
                Style::default().fg(Color::DarkGray)
            };
            ("[○●] OFF", style)
        };

        buf.set_string(toggle_x, area.y, toggle_text, text_style);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dropdown_state_toggle() {
        let mut state = DropdownState::new(0);
        assert!(!state.is_open);
        state.toggle();
        assert!(state.is_open);
        state.toggle();
        assert!(!state.is_open);
    }

    #[test]
    fn test_dropdown_state_navigation() {
        let mut state = DropdownState::new(1);
        state.is_open = true;

        state.move_down(4);
        assert_eq!(state.hover_index, 2);

        state.move_up(4);
        assert_eq!(state.hover_index, 1);

        // Wrap around
        state.hover_index = 0;
        state.move_up(4);
        assert_eq!(state.hover_index, 3);

        state.move_down(4);
        assert_eq!(state.hover_index, 0);
    }

    #[test]
    fn test_dropdown_state_select() {
        let mut state = DropdownState::new(0);
        state.is_open = true;
        state.hover_index = 2;

        state.select_current();
        assert_eq!(state.selected_index, 2);
        assert!(!state.is_open);
    }

    #[test]
    fn test_slider_state() {
        let mut state = SliderState::new(50.0, 0.0, 100.0);
        assert_eq!(state.percentage(), 0.5);

        state.increment();
        assert!(state.value > 50.0);

        state.decrement();
        state.decrement();
        assert!(state.value < 50.0);

        state.set_value(200.0);
        assert_eq!(state.value, 100.0); // Clamped to max
    }

    #[test]
    fn test_toggle_state() {
        let mut state = ToggleState::new(false);
        assert!(!state.is_on);

        state.toggle();
        assert!(state.is_on);

        state.toggle();
        assert!(!state.is_on);
    }
}
