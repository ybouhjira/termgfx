//! Layout management for the studio TUI

use ratatui::layout::{Constraint, Direction, Layout, Rect};

/// Panel sizes and constraints
pub struct StudioLayout {
    pub sidebar_width: u16,
    pub min_sidebar_width: u16,
    pub max_sidebar_width: u16,
}

impl Default for StudioLayout {
    fn default() -> Self {
        Self {
            sidebar_width: 20,
            min_sidebar_width: 15,
            max_sidebar_width: 40,
        }
    }
}

impl StudioLayout {
    /// Split the terminal into main regions
    pub fn split(&self, area: Rect) -> StudioAreas {
        // Main horizontal split: sidebar | content
        let main_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Length(self.sidebar_width),
                Constraint::Min(40),
            ])
            .split(area);

        // Sidebar: component list
        let sidebar = main_chunks[0];

        // Content area: vertical split into params, preview, command
        let content_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(8),  // Parameters panel
                Constraint::Min(10),    // Preview panel (takes remaining)
                Constraint::Length(5),  // Command panel
            ])
            .split(main_chunks[1]);

        StudioAreas {
            sidebar,
            params: content_chunks[0],
            preview: content_chunks[1],
            command: content_chunks[2],
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
}

/// The main layout areas
#[derive(Debug, Clone, Copy)]
pub struct StudioAreas {
    pub sidebar: Rect,
    pub params: Rect,
    pub preview: Rect,
    pub command: Rect,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_layout() {
        let layout = StudioLayout::default();
        assert_eq!(layout.sidebar_width, 20);
    }

    #[test]
    fn test_split_layout() {
        let layout = StudioLayout::default();
        let area = Rect::new(0, 0, 100, 40);
        let areas = layout.split(area);

        assert_eq!(areas.sidebar.width, 20);
        assert!(areas.preview.height > 0);
        assert!(areas.command.height > 0);
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
}
