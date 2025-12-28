//! Layout management for the studio TUI

use ratatui::layout::{Constraint, Direction, Layout, Rect};

/// Drag state for resizable dividers
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum DragState {
    #[default]
    None,
    /// Dragging the vertical divider between sidebar and content
    SidebarDivider,
    /// Dragging the horizontal divider between params and preview
    ParamsDivider,
}

/// Panel sizes and constraints
pub struct StudioLayout {
    pub sidebar_width: u16,
    pub min_sidebar_width: u16,
    pub max_sidebar_width: u16,
    /// Height ratio for params panel (0.0-1.0 of content area)
    pub params_ratio: f32,
    pub min_params_ratio: f32,
    pub max_params_ratio: f32,
    /// Current drag state
    pub drag_state: DragState,
}

impl Default for StudioLayout {
    fn default() -> Self {
        Self {
            sidebar_width: 20,
            min_sidebar_width: 15,
            max_sidebar_width: 40,
            params_ratio: 0.25,
            min_params_ratio: 0.1,
            max_params_ratio: 0.6,
            drag_state: DragState::None,
        }
    }
}

impl StudioLayout {
    /// Split the terminal into main regions
    pub fn split(&self, area: Rect) -> StudioAreas {
        // Main horizontal split: sidebar | content
        let main_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Length(self.sidebar_width), Constraint::Min(40)])
            .split(area);

        // Sidebar: component list
        let sidebar = main_chunks[0];

        // Content area: vertical split into params, preview, command
        let content_height = main_chunks[1].height.saturating_sub(5); // Reserve command panel
        let params_height = ((content_height as f32) * self.params_ratio).max(3.0) as u16;

        let content_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(params_height), // Parameters panel (resizable)
                Constraint::Min(10),               // Preview panel (takes remaining)
                Constraint::Length(5),             // Command panel
            ])
            .split(main_chunks[1]);

        StudioAreas {
            sidebar,
            params: content_chunks[0],
            preview: content_chunks[1],
            command: content_chunks[2],
            // Store divider positions for hit testing
            sidebar_divider_x: sidebar.x + sidebar.width,
            params_divider_y: content_chunks[0].y + content_chunks[0].height,
        }
    }

    /// Increase sidebar width
    pub fn grow_sidebar(&mut self) {
        if self.sidebar_width < self.max_sidebar_width {
            self.sidebar_width += 2;
        }
    }

    /// Decrease sidebar width
    pub fn shrink_sidebar(&mut self) {
        if self.sidebar_width > self.min_sidebar_width {
            self.sidebar_width -= 2;
        }
    }

    /// Increase params panel ratio
    pub fn grow_params(&mut self) {
        if self.params_ratio < self.max_params_ratio {
            self.params_ratio += 0.05;
        }
    }

    /// Decrease params panel ratio
    pub fn shrink_params(&mut self) {
        if self.params_ratio > self.min_params_ratio {
            self.params_ratio -= 0.05;
        }
    }

    /// Set sidebar width directly (for mouse drag)
    pub fn set_sidebar_width(&mut self, width: u16) {
        self.sidebar_width = width.clamp(self.min_sidebar_width, self.max_sidebar_width);
    }

    /// Set params ratio from absolute Y position (for mouse drag)
    pub fn set_params_height_from_y(&mut self, y: u16, content_top: u16, content_height: u16) {
        if content_height == 0 {
            return;
        }
        let relative_y = y.saturating_sub(content_top);
        let ratio = (relative_y as f32) / (content_height as f32);
        self.params_ratio = ratio.clamp(self.min_params_ratio, self.max_params_ratio);
    }

    /// Reset layout to defaults
    pub fn reset(&mut self) {
        self.sidebar_width = 20;
        self.params_ratio = 0.25;
        self.drag_state = DragState::None;
    }

    /// Check if a point is near the sidebar divider (within 1 pixel)
    pub fn is_on_sidebar_divider(&self, x: u16, divider_x: u16) -> bool {
        x == divider_x || x == divider_x.saturating_sub(1)
    }

    /// Check if a point is near the params divider (within 1 pixel)
    pub fn is_on_params_divider(&self, y: u16, divider_y: u16) -> bool {
        y == divider_y || y == divider_y.saturating_sub(1)
    }
}

/// The main layout areas
#[derive(Debug, Clone, Copy)]
pub struct StudioAreas {
    pub sidebar: Rect,
    pub params: Rect,
    pub preview: Rect,
    pub command: Rect,
    /// X position of vertical divider between sidebar and content
    pub sidebar_divider_x: u16,
    /// Y position of horizontal divider between params and preview
    pub params_divider_y: u16,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_layout() {
        let layout = StudioLayout::default();
        assert_eq!(layout.sidebar_width, 20);
        assert!((layout.params_ratio - 0.25).abs() < 0.01);
    }

    #[test]
    fn test_split_layout() {
        let layout = StudioLayout::default();
        let area = Rect::new(0, 0, 100, 40);
        let areas = layout.split(area);

        assert_eq!(areas.sidebar.width, 20);
        assert!(areas.preview.height > 0);
        assert!(areas.command.height > 0);
        assert_eq!(areas.sidebar_divider_x, 20);
    }

    #[test]
    fn test_resize_sidebar() {
        let mut layout = StudioLayout::default();
        let initial = layout.sidebar_width;

        layout.grow_sidebar();
        assert!(layout.sidebar_width > initial);

        layout.shrink_sidebar();
        layout.shrink_sidebar();
        assert!(layout.sidebar_width < initial);
    }

    #[test]
    fn test_resize_params() {
        let mut layout = StudioLayout::default();
        let initial = layout.params_ratio;

        layout.grow_params();
        assert!(layout.params_ratio > initial);

        layout.shrink_params();
        layout.shrink_params();
        assert!(layout.params_ratio < initial);
    }

    #[test]
    fn test_set_sidebar_width_clamped() {
        let mut layout = StudioLayout::default();

        layout.set_sidebar_width(5); // Below min
        assert_eq!(layout.sidebar_width, layout.min_sidebar_width);

        layout.set_sidebar_width(100); // Above max
        assert_eq!(layout.sidebar_width, layout.max_sidebar_width);

        layout.set_sidebar_width(25); // In range
        assert_eq!(layout.sidebar_width, 25);
    }

    #[test]
    fn test_reset_layout() {
        let mut layout = StudioLayout {
            sidebar_width: 35,
            params_ratio: 0.5,
            drag_state: DragState::SidebarDivider,
            ..Default::default()
        };

        layout.reset();

        assert_eq!(layout.sidebar_width, 20);
        assert!((layout.params_ratio - 0.25).abs() < 0.01);
        assert_eq!(layout.drag_state, DragState::None);
    }

    #[test]
    fn test_divider_hit_detection() {
        let layout = StudioLayout::default();

        // Sidebar divider at x=20
        assert!(layout.is_on_sidebar_divider(20, 20));
        assert!(layout.is_on_sidebar_divider(19, 20));
        assert!(!layout.is_on_sidebar_divider(18, 20));
        assert!(!layout.is_on_sidebar_divider(21, 20));

        // Params divider at y=10
        assert!(layout.is_on_params_divider(10, 10));
        assert!(layout.is_on_params_divider(9, 10));
        assert!(!layout.is_on_params_divider(8, 10));
    }

    #[test]
    fn test_drag_state() {
        let mut layout = StudioLayout::default();
        assert_eq!(layout.drag_state, DragState::None);

        layout.drag_state = DragState::SidebarDivider;
        assert_eq!(layout.drag_state, DragState::SidebarDivider);

        layout.drag_state = DragState::ParamsDivider;
        assert_eq!(layout.drag_state, DragState::ParamsDivider);
    }
}
