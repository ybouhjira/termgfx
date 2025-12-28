//! Unified color palette system for termgfx
//!
//! Provides a centralized color palette with semantic color names,
//! RGB/Hex conversion utilities, gradient generation, and accessibility checking.
//!
//! # Example
//! ```rust
//! use termgfx::design::colors::{Color, Palette, palette};
//!
//! // Get the default palette
//! let p = palette();
//! let success_color = p.success;
//!
//! // Create a color from hex
//! let custom = Color::from_hex("#FF5733").unwrap();
//!
//! // Generate a gradient
//! let gradient = p.gradient(&p.primary, &p.secondary, 5);
//! ```

#![allow(dead_code)]

use owo_colors::Style;
use serde::{Deserialize, Serialize};

/// RGB Color representation
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {
    /// Create a new color from RGB values
    pub const fn new(r: u8, g: u8, b: u8) -> Self {
        Color { r, g, b }
    }

    /// Create a color from a hex string (e.g., "#FF5733" or "FF5733")
    pub fn from_hex(hex: &str) -> Option<Self> {
        let hex = hex.trim_start_matches('#');
        if hex.len() != 6 {
            return None;
        }
        let r = u8::from_str_radix(&hex[0..2], 16).ok()?;
        let g = u8::from_str_radix(&hex[2..4], 16).ok()?;
        let b = u8::from_str_radix(&hex[4..6], 16).ok()?;
        Some(Color::new(r, g, b))
    }

    /// Convert to hex string with # prefix
    pub fn to_hex(self) -> String {
        format!("#{:02X}{:02X}{:02X}", self.r, self.g, self.b)
    }

    /// Get the approximate XTERM 256-color index
    pub fn to_xterm(self) -> u8 {
        // Simple approximation using the 6x6x6 color cube
        let r_idx = (self.r as u16 * 5 / 255) as u8;
        let g_idx = (self.g as u16 * 5 / 255) as u8;
        let b_idx = (self.b as u16 * 5 / 255) as u8;
        16 + 36 * r_idx + 6 * g_idx + b_idx
    }

    /// Get an owo-colors Style for this color (foreground)
    pub fn style(&self) -> Style {
        Style::new().truecolor(self.r, self.g, self.b)
    }

    /// Get an owo-colors Style for this color (background)
    pub fn bg_style(&self) -> Style {
        Style::new().on_truecolor(self.r, self.g, self.b)
    }

    /// Calculate relative luminance (for accessibility)
    pub fn luminance(&self) -> f64 {
        let r = Self::srgb_to_linear(self.r);
        let g = Self::srgb_to_linear(self.g);
        let b = Self::srgb_to_linear(self.b);
        0.2126 * r + 0.7152 * g + 0.0722 * b
    }

    fn srgb_to_linear(c: u8) -> f64 {
        let c = c as f64 / 255.0;
        if c <= 0.03928 {
            c / 12.92
        } else {
            ((c + 0.055) / 1.055).powf(2.4)
        }
    }

    /// Calculate contrast ratio with another color (WCAG)
    pub fn contrast_ratio(&self, other: &Color) -> f64 {
        let l1 = self.luminance();
        let l2 = other.luminance();
        let (lighter, darker) = if l1 > l2 { (l1, l2) } else { (l2, l1) };
        (lighter + 0.05) / (darker + 0.05)
    }

    /// Check if contrast meets WCAG AA standard (4.5:1 for normal text)
    pub fn meets_aa(&self, other: &Color) -> bool {
        self.contrast_ratio(other) >= 4.5
    }

    /// Check if contrast meets WCAG AAA standard (7:1 for normal text)
    pub fn meets_aaa(&self, other: &Color) -> bool {
        self.contrast_ratio(other) >= 7.0
    }

    /// Lighten the color by a percentage (0.0 to 1.0)
    pub fn lighten(&self, amount: f64) -> Self {
        let amount = amount.clamp(0.0, 1.0);
        Color::new(
            (self.r as f64 + (255.0 - self.r as f64) * amount) as u8,
            (self.g as f64 + (255.0 - self.g as f64) * amount) as u8,
            (self.b as f64 + (255.0 - self.b as f64) * amount) as u8,
        )
    }

    /// Darken the color by a percentage (0.0 to 1.0)
    pub fn darken(&self, amount: f64) -> Self {
        let amount = amount.clamp(0.0, 1.0);
        Color::new(
            (self.r as f64 * (1.0 - amount)) as u8,
            (self.g as f64 * (1.0 - amount)) as u8,
            (self.b as f64 * (1.0 - amount)) as u8,
        )
    }

    /// Blend with another color
    pub fn blend(&self, other: &Color, ratio: f64) -> Self {
        let ratio = ratio.clamp(0.0, 1.0);
        Color::new(
            (self.r as f64 * (1.0 - ratio) + other.r as f64 * ratio) as u8,
            (self.g as f64 * (1.0 - ratio) + other.g as f64 * ratio) as u8,
            (self.b as f64 * (1.0 - ratio) + other.b as f64 * ratio) as u8,
        )
    }
}

impl Default for Color {
    fn default() -> Self {
        Color::new(255, 255, 255) // White
    }
}

/// Named color constants
pub mod named {
    use super::Color;

    // Basic colors
    pub const BLACK: Color = Color::new(0, 0, 0);
    pub const WHITE: Color = Color::new(255, 255, 255);
    pub const RED: Color = Color::new(255, 0, 0);
    pub const GREEN: Color = Color::new(0, 255, 0);
    pub const BLUE: Color = Color::new(0, 0, 255);
    pub const YELLOW: Color = Color::new(255, 255, 0);
    pub const CYAN: Color = Color::new(0, 255, 255);
    pub const MAGENTA: Color = Color::new(255, 0, 255);

    // GitHub-inspired semantic colors
    pub const SUCCESS: Color = Color::new(63, 185, 80); // #3FB950
    pub const WARNING: Color = Color::new(210, 153, 34); // #D29922
    pub const DANGER: Color = Color::new(248, 81, 73); // #F85149
    pub const INFO: Color = Color::new(88, 166, 255); // #58A6FF

    // Neutral colors
    pub const MUTED: Color = Color::new(139, 148, 158); // #8B949E
    pub const BORDER: Color = Color::new(48, 54, 61); // #30363D
    pub const SURFACE: Color = Color::new(22, 27, 34); // #161B22
    pub const BACKGROUND: Color = Color::new(13, 17, 23); // #0D1117

    // Brand colors
    pub const PRIMARY: Color = Color::new(88, 166, 255); // #58A6FF
    pub const SECONDARY: Color = Color::new(161, 106, 255); // #A16AFF
}

/// Chart color cycle (8 distinct colors for data visualization)
pub const CHART_COLORS: [Color; 8] = [
    Color::new(88, 166, 255),  // Blue
    Color::new(63, 185, 80),   // Green
    Color::new(248, 81, 73),   // Red
    Color::new(210, 153, 34),  // Yellow
    Color::new(161, 106, 255), // Purple
    Color::new(0, 187, 194),   // Cyan
    Color::new(255, 123, 114), // Coral
    Color::new(219, 109, 202), // Pink
];

/// Gradient colors for visual effects
pub const GRADIENT_PRESETS: [(&str, Color, Color); 6] = [
    ("ocean", Color::new(0, 119, 182), Color::new(0, 180, 216)),
    (
        "sunset",
        Color::new(255, 111, 97),
        Color::new(255, 203, 119),
    ),
    ("forest", Color::new(34, 139, 34), Color::new(144, 238, 144)),
    ("berry", Color::new(147, 51, 234), Color::new(219, 39, 119)),
    (
        "midnight",
        Color::new(30, 41, 59),
        Color::new(100, 116, 139),
    ),
    ("fire", Color::new(239, 68, 68), Color::new(245, 158, 11)),
];

/// Complete color palette with semantic meanings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Palette {
    // Brand colors
    pub primary: Color,
    pub secondary: Color,

    // Semantic colors
    pub success: Color,
    pub warning: Color,
    pub danger: Color,
    pub info: Color,

    // Neutral colors
    pub text: Color,
    pub muted: Color,
    pub border: Color,
    pub background: Color,
    pub surface: Color,

    // Chart colors
    pub chart: [Color; 8],
}

impl Default for Palette {
    fn default() -> Self {
        Palette {
            primary: named::PRIMARY,
            secondary: named::SECONDARY,
            success: named::SUCCESS,
            warning: named::WARNING,
            danger: named::DANGER,
            info: named::INFO,
            text: named::WHITE,
            muted: named::MUTED,
            border: named::BORDER,
            background: named::BACKGROUND,
            surface: named::SURFACE,
            chart: CHART_COLORS,
        }
    }
}

impl Palette {
    /// Create a new palette with custom colors
    pub fn new() -> Self {
        Self::default()
    }

    /// Create a light mode palette
    pub fn light() -> Self {
        Palette {
            primary: Color::from_hex("#0366D6").unwrap(),
            secondary: Color::from_hex("#6F42C1").unwrap(),
            success: Color::from_hex("#28A745").unwrap(),
            warning: Color::from_hex("#FFC107").unwrap(),
            danger: Color::from_hex("#DC3545").unwrap(),
            info: Color::from_hex("#17A2B8").unwrap(),
            text: Color::from_hex("#24292E").unwrap(),
            muted: Color::from_hex("#6A737D").unwrap(),
            border: Color::from_hex("#E1E4E8").unwrap(),
            background: Color::from_hex("#FFFFFF").unwrap(),
            surface: Color::from_hex("#F6F8FA").unwrap(),
            chart: CHART_COLORS,
        }
    }

    /// Generate a gradient between two colors
    pub fn gradient(&self, from: &Color, to: &Color, steps: usize) -> Vec<Color> {
        if steps == 0 {
            return vec![];
        }
        if steps == 1 {
            return vec![*from];
        }
        (0..steps)
            .map(|i| from.blend(to, i as f64 / (steps - 1) as f64))
            .collect()
    }

    /// Get a color by semantic name
    pub fn semantic(&self, name: &str) -> Option<&Color> {
        match name.to_lowercase().as_str() {
            "primary" => Some(&self.primary),
            "secondary" => Some(&self.secondary),
            "success" => Some(&self.success),
            "warning" => Some(&self.warning),
            "danger" | "error" => Some(&self.danger),
            "info" => Some(&self.info),
            "text" => Some(&self.text),
            "muted" => Some(&self.muted),
            "border" => Some(&self.border),
            "background" | "bg" => Some(&self.background),
            "surface" => Some(&self.surface),
            _ => None,
        }
    }

    /// Get a chart color by index (cycles through available colors)
    pub fn chart_color(&self, index: usize) -> Color {
        self.chart[index % self.chart.len()]
    }

    /// Get a gradient preset by name
    pub fn gradient_preset(&self, name: &str) -> Option<Vec<Color>> {
        GRADIENT_PRESETS
            .iter()
            .find(|(n, _, _)| *n == name.to_lowercase())
            .map(|(_, from, to)| self.gradient(from, to, 5))
    }
}

/// Get the default palette
pub fn palette() -> Palette {
    Palette::default()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_color_from_hex() {
        let color = Color::from_hex("#FF5733").unwrap();
        assert_eq!(color.r, 255);
        assert_eq!(color.g, 87);
        assert_eq!(color.b, 51);

        let color2 = Color::from_hex("00FF00").unwrap();
        assert_eq!(color2.r, 0);
        assert_eq!(color2.g, 255);
        assert_eq!(color2.b, 0);
    }

    #[test]
    fn test_color_to_hex() {
        let color = Color::new(255, 87, 51);
        assert_eq!(color.to_hex(), "#FF5733");
    }

    #[test]
    fn test_color_from_hex_invalid() {
        assert!(Color::from_hex("invalid").is_none());
        assert!(Color::from_hex("#FF").is_none());
        assert!(Color::from_hex("#GGGGGG").is_none());
    }

    #[test]
    fn test_color_to_xterm() {
        let black = Color::new(0, 0, 0);
        let white = Color::new(255, 255, 255);
        assert_eq!(black.to_xterm(), 16); // First color in 6x6x6 cube
        assert_eq!(white.to_xterm(), 231); // Last color in 6x6x6 cube
    }

    #[test]
    fn test_color_luminance() {
        let black = Color::new(0, 0, 0);
        let white = Color::new(255, 255, 255);
        assert!(black.luminance() < 0.01);
        assert!(white.luminance() > 0.99);
    }

    #[test]
    fn test_color_contrast_ratio() {
        let black = Color::new(0, 0, 0);
        let white = Color::new(255, 255, 255);
        let ratio = black.contrast_ratio(&white);
        assert!(ratio > 20.0); // Should be 21:1
    }

    #[test]
    fn test_color_meets_wcag() {
        let black = Color::new(0, 0, 0);
        let white = Color::new(255, 255, 255);
        assert!(black.meets_aa(&white));
        assert!(black.meets_aaa(&white));

        let gray = Color::new(128, 128, 128);
        assert!(!gray.meets_aaa(&white)); // Gray on white doesn't meet AAA
    }

    #[test]
    fn test_color_lighten() {
        let color = Color::new(100, 100, 100);
        let lighter = color.lighten(0.5);
        assert!(lighter.r > color.r);
        assert!(lighter.g > color.g);
        assert!(lighter.b > color.b);
    }

    #[test]
    fn test_color_darken() {
        let color = Color::new(200, 200, 200);
        let darker = color.darken(0.5);
        assert!(darker.r < color.r);
        assert!(darker.g < color.g);
        assert!(darker.b < color.b);
    }

    #[test]
    fn test_color_blend() {
        let red = Color::new(255, 0, 0);
        let blue = Color::new(0, 0, 255);
        let purple = red.blend(&blue, 0.5);
        assert_eq!(purple.r, 127);
        assert_eq!(purple.g, 0);
        assert_eq!(purple.b, 127);
    }

    #[test]
    fn test_palette_default() {
        let p = Palette::default();
        assert_eq!(p.success, named::SUCCESS);
        assert_eq!(p.danger, named::DANGER);
    }

    #[test]
    fn test_palette_gradient() {
        let p = palette();
        let gradient = p.gradient(&p.primary, &p.secondary, 5);
        assert_eq!(gradient.len(), 5);
        assert_eq!(gradient[0], p.primary);
        assert_eq!(gradient[4], p.secondary);
    }

    #[test]
    fn test_palette_gradient_single() {
        let p = palette();
        let gradient = p.gradient(&p.primary, &p.secondary, 1);
        assert_eq!(gradient.len(), 1);
        assert_eq!(gradient[0], p.primary);
    }

    #[test]
    fn test_palette_gradient_empty() {
        let p = palette();
        let gradient = p.gradient(&p.primary, &p.secondary, 0);
        assert!(gradient.is_empty());
    }

    #[test]
    fn test_palette_semantic() {
        let p = palette();
        assert_eq!(p.semantic("success"), Some(&p.success));
        assert_eq!(p.semantic("danger"), Some(&p.danger));
        assert_eq!(p.semantic("error"), Some(&p.danger)); // Alias
        assert_eq!(p.semantic("unknown"), None);
    }

    #[test]
    fn test_palette_chart_color() {
        let p = palette();
        assert_eq!(p.chart_color(0), p.chart[0]);
        assert_eq!(p.chart_color(8), p.chart[0]); // Wraps around
    }

    #[test]
    fn test_palette_gradient_preset() {
        let p = palette();
        let ocean = p.gradient_preset("ocean");
        assert!(ocean.is_some());
        assert_eq!(ocean.unwrap().len(), 5);

        assert!(p.gradient_preset("unknown").is_none());
    }

    #[test]
    fn test_named_colors() {
        assert_eq!(named::BLACK.r, 0);
        assert_eq!(named::WHITE.r, 255);
        assert_eq!(named::SUCCESS.to_hex(), "#3FB950");
    }

    #[test]
    fn test_chart_colors() {
        assert_eq!(CHART_COLORS.len(), 8);
    }

    #[test]
    fn test_palette_light() {
        let p = Palette::light();
        // Light palette should have lighter background
        assert!(p.background.luminance() > 0.9);
    }

    #[test]
    fn test_color_serialization() {
        let color = Color::new(255, 128, 64);
        let json = serde_json::to_string(&color).unwrap();
        let deserialized: Color = serde_json::from_str(&json).unwrap();
        assert_eq!(color, deserialized);
    }
}
