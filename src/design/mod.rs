//! Design system components - spacing, layout, and styling utilities

pub mod presets;
pub mod spacing;

pub use presets::{color_to_style, ColorPalette, Spacing, StylePreset, TextTransform, Typography};
pub use spacing::{GridAlignment, Margin, Padding, SpacingScale, SpacingSize};
