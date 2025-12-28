//! Design system components - spacing, layout, styling utilities, and themes

pub mod colors;
pub mod spacing;
pub mod theme;

#[allow(unused_imports)]
pub use colors::{palette, Color, Palette, CHART_COLORS, GRADIENT_PRESETS};
#[allow(unused_imports)]
pub use spacing::{sp, spm, Spacing, SpacingConfig, SpacingLevel};
