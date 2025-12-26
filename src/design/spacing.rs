//! Spacing scale system for termgfx
//!
//! Provides consistent spacing tokens for padding, margins, and grid alignment.
//! Based on a modular scale: xs(1), sm(2), md(4), lg(8), xl(16), xxl(32)

use std::fmt;

/// Spacing scale tokens - standardized spacing values
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum SpacingSize {
    /// Extra small: 1 unit
    XS = 1,
    /// Small: 2 units
    SM = 2,
    /// Medium: 4 units
    MD = 4,
    /// Large: 8 units
    LG = 8,
    /// Extra large: 16 units
    XL = 16,
    /// Double extra large: 32 units
    XXL = 32,
}

impl SpacingSize {
    /// Get the numeric value of the spacing size
    pub fn value(self) -> u32 {
        self as u32
    }

    /// Get all spacing sizes in order
    pub fn all() -> &'static [SpacingSize] {
        &[
            SpacingSize::XS,
            SpacingSize::SM,
            SpacingSize::MD,
            SpacingSize::LG,
            SpacingSize::XL,
            SpacingSize::XXL,
        ]
    }

    /// Get the next larger spacing size, if available
    pub fn next_larger(self) -> Option<SpacingSize> {
        match self {
            SpacingSize::XS => Some(SpacingSize::SM),
            SpacingSize::SM => Some(SpacingSize::MD),
            SpacingSize::MD => Some(SpacingSize::LG),
            SpacingSize::LG => Some(SpacingSize::XL),
            SpacingSize::XL => Some(SpacingSize::XXL),
            SpacingSize::XXL => None,
        }
    }

    /// Get the next smaller spacing size, if available
    pub fn next_smaller(self) -> Option<SpacingSize> {
        match self {
            SpacingSize::XS => None,
            SpacingSize::SM => Some(SpacingSize::XS),
            SpacingSize::MD => Some(SpacingSize::SM),
            SpacingSize::LG => Some(SpacingSize::MD),
            SpacingSize::XL => Some(SpacingSize::LG),
            SpacingSize::XXL => Some(SpacingSize::XL),
        }
    }
}

impl fmt::Display for SpacingSize {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SpacingSize::XS => write!(f, "xs"),
            SpacingSize::SM => write!(f, "sm"),
            SpacingSize::MD => write!(f, "md"),
            SpacingSize::LG => write!(f, "lg"),
            SpacingSize::XL => write!(f, "xl"),
            SpacingSize::XXL => write!(f, "xxl"),
        }
    }
}

/// Unified spacing scale struct - single source of truth for all spacing values
#[derive(Debug, Clone)]
pub struct SpacingScale {
    xs: u32,
    sm: u32,
    md: u32,
    lg: u32,
    xl: u32,
    xxl: u32,
}

impl SpacingScale {
    /// Create a new spacing scale with default values (1, 2, 4, 8, 16, 32)
    pub fn new() -> Self {
        Self {
            xs: 1,
            sm: 2,
            md: 4,
            lg: 8,
            xl: 16,
            xxl: 32,
        }
    }

    /// Create a custom spacing scale with custom base values
    pub fn custom(xs: u32, sm: u32, md: u32, lg: u32, xl: u32, xxl: u32) -> Self {
        Self { xs, sm, md, lg, xl, xxl }
    }

    /// Get spacing value by size
    pub fn get(&self, size: SpacingSize) -> u32 {
        match size {
            SpacingSize::XS => self.xs,
            SpacingSize::SM => self.sm,
            SpacingSize::MD => self.md,
            SpacingSize::LG => self.lg,
            SpacingSize::XL => self.xl,
            SpacingSize::XXL => self.xxl,
        }
    }

    /// Combine two spacing values (e.g., for multi-direction padding)
    pub fn combine(&self, size1: SpacingSize, size2: SpacingSize) -> u32 {
        self.get(size1) + self.get(size2)
    }
}

impl Default for SpacingScale {
    fn default() -> Self {
        Self::new()
    }
}

/// Padding configuration for all four sides
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Padding {
    pub top: u32,
    pub right: u32,
    pub bottom: u32,
    pub left: u32,
}

impl Padding {
    /// Create padding with the same value on all sides
    pub fn uniform(value: u32) -> Self {
        Self {
            top: value,
            right: value,
            bottom: value,
            left: value,
        }
    }

    /// Create padding with different values (top/bottom, left/right)
    pub fn symmetric(vertical: u32, horizontal: u32) -> Self {
        Self {
            top: vertical,
            right: horizontal,
            bottom: vertical,
            left: horizontal,
        }
    }

    /// Create padding with individual values
    pub fn custom(top: u32, right: u32, bottom: u32, left: u32) -> Self {
        Self { top, right, bottom, left }
    }

    /// Create padding from a spacing size (uniform)
    pub fn from_size(scale: &SpacingScale, size: SpacingSize) -> Self {
        let value = scale.get(size);
        Self::uniform(value)
    }

    /// Create padding with different sizes for vertical and horizontal
    pub fn symmetric_sizes(scale: &SpacingScale, vertical: SpacingSize, horizontal: SpacingSize) -> Self {
        Self::symmetric(scale.get(vertical), scale.get(horizontal))
    }

    /// Get total horizontal padding (left + right)
    pub fn horizontal_total(&self) -> u32 {
        self.left + self.right
    }

    /// Get total vertical padding (top + bottom)
    pub fn vertical_total(&self) -> u32 {
        self.top + self.bottom
    }

    /// Get total padding on all sides
    pub fn total(&self) -> u32 {
        self.horizontal_total() + self.vertical_total()
    }
}

impl Default for Padding {
    fn default() -> Self {
        Self::uniform(0)
    }
}

/// Margin configuration for all four sides
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Margin {
    pub top: u32,
    pub right: u32,
    pub bottom: u32,
    pub left: u32,
}

impl Margin {
    /// Create margin with the same value on all sides
    pub fn uniform(value: u32) -> Self {
        Self {
            top: value,
            right: value,
            bottom: value,
            left: value,
        }
    }

    /// Create margin with different values (top/bottom, left/right)
    pub fn symmetric(vertical: u32, horizontal: u32) -> Self {
        Self {
            top: vertical,
            right: horizontal,
            bottom: vertical,
            left: horizontal,
        }
    }

    /// Create margin with individual values
    pub fn custom(top: u32, right: u32, bottom: u32, left: u32) -> Self {
        Self { top, right, bottom, left }
    }

    /// Create margin from a spacing size (uniform)
    pub fn from_size(scale: &SpacingScale, size: SpacingSize) -> Self {
        let value = scale.get(size);
        Self::uniform(value)
    }

    /// Create margin with different sizes for vertical and horizontal
    pub fn symmetric_sizes(scale: &SpacingScale, vertical: SpacingSize, horizontal: SpacingSize) -> Self {
        Self::symmetric(scale.get(vertical), scale.get(horizontal))
    }

    /// Get total horizontal margin (left + right)
    pub fn horizontal_total(&self) -> u32 {
        self.left + self.right
    }

    /// Get total vertical margin (top + bottom)
    pub fn vertical_total(&self) -> u32 {
        self.top + self.bottom
    }

    /// Get total margin on all sides
    pub fn total(&self) -> u32 {
        self.horizontal_total() + self.vertical_total()
    }
}

impl Default for Margin {
    fn default() -> Self {
        Self::uniform(0)
    }
}

/// Grid alignment utilities for consistent layout
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GridAlignment {
    /// Number of columns in the grid
    pub columns: u32,
    /// Spacing between columns
    pub column_gap: u32,
    /// Spacing between rows
    pub row_gap: u32,
    /// Padding inside each grid cell
    pub cell_padding: Padding,
}

impl GridAlignment {
    /// Create a grid alignment configuration
    pub fn new(columns: u32, column_gap: u32, row_gap: u32) -> Self {
        Self {
            columns,
            column_gap,
            row_gap,
            cell_padding: Padding::default(),
        }
    }

    /// Create a grid with padding inside cells
    pub fn with_padding(columns: u32, column_gap: u32, row_gap: u32, cell_padding: Padding) -> Self {
        Self {
            columns,
            column_gap,
            row_gap,
            cell_padding,
        }
    }

    /// Create a grid from spacing scale values
    pub fn from_scale(scale: &SpacingScale, columns: u32, gap: SpacingSize, padding: SpacingSize) -> Self {
        let gap_value = scale.get(gap);
        let padding_value = scale.get(padding);
        Self {
            columns,
            column_gap: gap_value,
            row_gap: gap_value,
            cell_padding: Padding::uniform(padding_value),
        }
    }

    /// Get total width needed for a given number of content units
    /// Returns: (content_width * units) + (gap * (units - 1))
    pub fn total_width(&self, unit_width: u32, units: u32) -> u32 {
        if units == 0 {
            return 0;
        }
        (unit_width * units) + (self.column_gap * (units.saturating_sub(1)))
    }

    /// Get total height needed for a given number of rows
    /// Returns: (row_height * rows) + (gap * (rows - 1))
    pub fn total_height(&self, row_height: u32, rows: u32) -> u32 {
        if rows == 0 {
            return 0;
        }
        (row_height * rows) + (self.row_gap * (rows.saturating_sub(1)))
    }

    /// Calculate width per column given total available width
    /// Returns width available for content in each column (minus gaps and padding)
    pub fn column_width(&self, total_width: u32) -> u32 {
        if self.columns == 0 {
            return 0;
        }
        let gap_total = self.column_gap * (self.columns.saturating_sub(1));
        let padding_total = self.cell_padding.horizontal_total() * self.columns;
        (total_width.saturating_sub(gap_total + padding_total)) / self.columns
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn spacing_size_values() {
        assert_eq!(SpacingSize::XS.value(), 1);
        assert_eq!(SpacingSize::SM.value(), 2);
        assert_eq!(SpacingSize::MD.value(), 4);
        assert_eq!(SpacingSize::LG.value(), 8);
        assert_eq!(SpacingSize::XL.value(), 16);
        assert_eq!(SpacingSize::XXL.value(), 32);
    }

    #[test]
    fn spacing_size_ordering() {
        assert!(SpacingSize::XS < SpacingSize::SM);
        assert!(SpacingSize::SM < SpacingSize::MD);
        assert!(SpacingSize::MD < SpacingSize::LG);
        assert!(SpacingSize::LG < SpacingSize::XL);
        assert!(SpacingSize::XL < SpacingSize::XXL);
    }

    #[test]
    fn spacing_size_navigation() {
        assert_eq!(SpacingSize::XS.next_larger(), Some(SpacingSize::SM));
        assert_eq!(SpacingSize::SM.next_larger(), Some(SpacingSize::MD));
        assert_eq!(SpacingSize::XXL.next_larger(), None);

        assert_eq!(SpacingSize::XS.next_smaller(), None);
        assert_eq!(SpacingSize::SM.next_smaller(), Some(SpacingSize::XS));
        assert_eq!(SpacingSize::XXL.next_smaller(), Some(SpacingSize::XL));
    }

    #[test]
    fn spacing_size_display() {
        assert_eq!(SpacingSize::XS.to_string(), "xs");
        assert_eq!(SpacingSize::SM.to_string(), "sm");
        assert_eq!(SpacingSize::MD.to_string(), "md");
        assert_eq!(SpacingSize::LG.to_string(), "lg");
        assert_eq!(SpacingSize::XL.to_string(), "xl");
        assert_eq!(SpacingSize::XXL.to_string(), "xxl");
    }

    #[test]
    fn spacing_scale_default() {
        let scale = SpacingScale::new();
        assert_eq!(scale.get(SpacingSize::XS), 1);
        assert_eq!(scale.get(SpacingSize::SM), 2);
        assert_eq!(scale.get(SpacingSize::MD), 4);
        assert_eq!(scale.get(SpacingSize::LG), 8);
        assert_eq!(scale.get(SpacingSize::XL), 16);
        assert_eq!(scale.get(SpacingSize::XXL), 32);
    }

    #[test]
    fn spacing_scale_custom() {
        let scale = SpacingScale::custom(2, 4, 8, 16, 32, 64);
        assert_eq!(scale.get(SpacingSize::XS), 2);
        assert_eq!(scale.get(SpacingSize::SM), 4);
        assert_eq!(scale.get(SpacingSize::XXL), 64);
    }

    #[test]
    fn spacing_scale_combine() {
        let scale = SpacingScale::new();
        assert_eq!(scale.combine(SpacingSize::XS, SpacingSize::SM), 3);
        assert_eq!(scale.combine(SpacingSize::MD, SpacingSize::LG), 12);
        assert_eq!(scale.combine(SpacingSize::XL, SpacingSize::XXL), 48);
    }

    #[test]
    fn padding_uniform() {
        let padding = Padding::uniform(4);
        assert_eq!(padding.top, 4);
        assert_eq!(padding.right, 4);
        assert_eq!(padding.bottom, 4);
        assert_eq!(padding.left, 4);
    }

    #[test]
    fn padding_symmetric() {
        let padding = Padding::symmetric(2, 4);
        assert_eq!(padding.top, 2);
        assert_eq!(padding.right, 4);
        assert_eq!(padding.bottom, 2);
        assert_eq!(padding.left, 4);
    }

    #[test]
    fn padding_totals() {
        let padding = Padding::custom(1, 2, 3, 4);
        assert_eq!(padding.horizontal_total(), 6); // 2 + 4
        assert_eq!(padding.vertical_total(), 4); // 1 + 3
        assert_eq!(padding.total(), 10); // 1 + 2 + 3 + 4
    }

    #[test]
    fn padding_from_size() {
        let scale = SpacingScale::new();
        let padding = Padding::from_size(&scale, SpacingSize::MD);
        let expected = Padding::uniform(4);
        assert_eq!(padding, expected);
    }

    #[test]
    fn padding_symmetric_sizes() {
        let scale = SpacingScale::new();
        let padding = Padding::symmetric_sizes(&scale, SpacingSize::SM, SpacingSize::LG);
        assert_eq!(padding.top, 2);
        assert_eq!(padding.right, 8);
        assert_eq!(padding.left, 8);
        assert_eq!(padding.bottom, 2);
    }

    #[test]
    fn margin_uniform() {
        let margin = Margin::uniform(8);
        assert_eq!(margin.top, 8);
        assert_eq!(margin.right, 8);
        assert_eq!(margin.bottom, 8);
        assert_eq!(margin.left, 8);
    }

    #[test]
    fn margin_symmetric() {
        let margin = Margin::symmetric(4, 8);
        assert_eq!(margin.top, 4);
        assert_eq!(margin.right, 8);
        assert_eq!(margin.bottom, 4);
        assert_eq!(margin.left, 8);
    }

    #[test]
    fn margin_totals() {
        let margin = Margin::custom(2, 4, 6, 8);
        assert_eq!(margin.horizontal_total(), 12); // 4 + 8
        assert_eq!(margin.vertical_total(), 8); // 2 + 6
        assert_eq!(margin.total(), 20); // 2 + 4 + 6 + 8
    }

    #[test]
    fn grid_alignment_basic() {
        let grid = GridAlignment::new(3, 2, 1);
        assert_eq!(grid.columns, 3);
        assert_eq!(grid.column_gap, 2);
        assert_eq!(grid.row_gap, 1);
    }

    #[test]
    fn grid_total_width() {
        let grid = GridAlignment::new(3, 2, 0);
        // 3 items of width 10 with 2 unit gaps: 10 + 10 + 10 + 2 + 2 = 34
        assert_eq!(grid.total_width(10, 3), 34);
    }

    #[test]
    fn grid_total_height() {
        let grid = GridAlignment::new(0, 0, 2);
        // 4 rows of height 5 with 2 unit gaps: 5 + 5 + 5 + 5 + 2 + 2 + 2 = 26
        assert_eq!(grid.total_height(5, 4), 26);
    }

    #[test]
    fn grid_zero_items() {
        let grid = GridAlignment::new(3, 2, 1);
        assert_eq!(grid.total_width(10, 0), 0);
        assert_eq!(grid.total_height(5, 0), 0);
    }

    #[test]
    fn grid_single_item() {
        let grid = GridAlignment::new(1, 2, 1);
        assert_eq!(grid.total_width(10, 1), 10);
        assert_eq!(grid.total_height(5, 1), 5);
    }

    #[test]
    fn grid_column_width() {
        let padding = Padding::uniform(1);
        let grid = GridAlignment::with_padding(3, 1, 0, padding);
        // Total: 20, 3 columns with padding 1 on each side (2 per column)
        // Gaps between columns: 1 + 1 = 2
        // Available: 20 - 6 (padding) - 2 (gaps) = 12, per column: 12/3 = 4
        assert_eq!(grid.column_width(20), 4);
    }

    #[test]
    fn grid_from_scale() {
        let scale = SpacingScale::new();
        let grid = GridAlignment::from_scale(&scale, 4, SpacingSize::MD, SpacingSize::SM);
        assert_eq!(grid.columns, 4);
        assert_eq!(grid.column_gap, 4); // SpacingSize::MD = 4
        assert_eq!(grid.row_gap, 4);
        assert_eq!(grid.cell_padding.top, 2); // SpacingSize::SM = 2
    }
}
