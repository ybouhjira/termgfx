//! Design system components - spacing, layout, styling utilities, and themes

pub mod spacing;
pub mod theme;

pub use spacing::{GridAlignment, Margin, Padding, SpacingScale, SpacingSize};
pub use theme::{Theme, ThemePreset, Colors, Spacing as ThemeSpacing, Typography as ThemeTypography};
