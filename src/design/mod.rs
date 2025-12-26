//! Design system components - spacing, layout, styling utilities, presets, and themes

pub mod presets;
pub mod spacing;
pub mod theme;

pub use presets::{color_to_style, ColorPalette, Spacing, StylePreset, TextTransform, Typography};
pub use spacing::{GridAlignment, Margin, Padding, SpacingScale, SpacingSize};
pub use theme::{Theme, ThemePreset, Colors, Spacing as ThemeSpacing, Typography as ThemeTypography};
