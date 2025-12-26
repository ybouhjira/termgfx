//! Theme system for termgfx
//!
//! Provides a configurable theme system with built-in presets for terminal styling.
//! Supports loading/saving themes from JSON and environment variable configuration.
//!
//! # Example
//!
//! ```rust
//! use termgfx::design::theme::{Theme, ThemePreset};
//!
//! let theme = Theme::load_preset(ThemePreset::Dark);
//! println!("Primary color: {}", theme.colors.primary);
//! ```

use serde::{Deserialize, Serialize};
use std::path::Path;
use std::fs;
use std::env;
use anyhow::{Result, Context};

/// Color configuration for a theme
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Colors {
    /// Primary brand color
    pub primary: String,
    /// Secondary accent color
    pub secondary: String,
    /// Success/positive color
    pub success: String,
    /// Warning/caution color
    pub warning: String,
    /// Danger/error color
    pub danger: String,
    /// Information color
    pub info: String,
    /// Background color
    pub background: String,
    /// Foreground/text color
    pub foreground: String,
    /// Subtle background for containers
    pub surface: String,
    /// Border/divider color
    pub border: String,
}

/// Spacing configuration for a theme
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Spacing {
    /// Extra small spacing (2 units)
    pub xs: usize,
    /// Small spacing (4 units)
    pub sm: usize,
    /// Medium spacing (8 units)
    pub md: usize,
    /// Large spacing (16 units)
    pub lg: usize,
    /// Extra large spacing (24 units)
    pub xl: usize,
    /// 2x extra large spacing (32 units)
    pub xxl: usize,
}

/// Typography configuration for a theme
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Typography {
    /// Small font size (8pt)
    pub small: usize,
    /// Base/normal font size (12pt)
    pub base: usize,
    /// Large font size (16pt)
    pub large: usize,
    /// Extra large font size (20pt)
    pub extra_large: usize,
    /// Heading 1 font size (32pt)
    pub heading1: usize,
    /// Heading 2 font size (24pt)
    pub heading2: usize,
}

/// Complete theme configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Theme {
    /// Theme name/identifier
    pub name: String,
    /// Theme description
    pub description: String,
    /// Color palette
    pub colors: Colors,
    /// Spacing system
    pub spacing: Spacing,
    /// Typography system
    pub typography: Typography,
    /// Whether this is a dark theme
    pub is_dark: bool,
}

/// Available built-in theme presets
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ThemePreset {
    /// Dark theme (default) - comfortable for extended use
    Dark,
    /// Light theme - high contrast
    Light,
    /// Nord - Arctic, north-bluish color palette
    Nord,
    /// Dracula - Dark, sophisticated theme
    Dracula,
    /// Monokai - Dark theme with vibrant accent colors
    Monokai,
    /// Solarized - Precision colors for machines and people
    Solarized,
    /// Gruvbox - Retro groove color scheme
    Gruvbox,
}

impl ThemePreset {
    /// Get all available presets
    pub fn all() -> Vec<ThemePreset> {
        vec![
            ThemePreset::Dark,
            ThemePreset::Light,
            ThemePreset::Nord,
            ThemePreset::Dracula,
            ThemePreset::Monokai,
            ThemePreset::Solarized,
            ThemePreset::Gruvbox,
        ]
    }

    /// Parse from string (case-insensitive)
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "dark" => Some(ThemePreset::Dark),
            "light" => Some(ThemePreset::Light),
            "nord" => Some(ThemePreset::Nord),
            "dracula" => Some(ThemePreset::Dracula),
            "monokai" => Some(ThemePreset::Monokai),
            "solarized" => Some(ThemePreset::Solarized),
            "gruvbox" => Some(ThemePreset::Gruvbox),
            _ => None,
        }
    }

    /// Get the preset as a string
    pub fn as_str(&self) -> &'static str {
        match self {
            ThemePreset::Dark => "dark",
            ThemePreset::Light => "light",
            ThemePreset::Nord => "nord",
            ThemePreset::Dracula => "dracula",
            ThemePreset::Monokai => "monokai",
            ThemePreset::Solarized => "solarized",
            ThemePreset::Gruvbox => "gruvbox",
        }
    }
}

impl Theme {
    /// Load a theme from a preset
    pub fn load_preset(preset: ThemePreset) -> Self {
        match preset {
            ThemePreset::Dark => Self::dark(),
            ThemePreset::Light => Self::light(),
            ThemePreset::Nord => Self::nord(),
            ThemePreset::Dracula => Self::dracula(),
            ThemePreset::Monokai => Self::monokai(),
            ThemePreset::Solarized => Self::solarized(),
            ThemePreset::Gruvbox => Self::gruvbox(),
        }
    }

    /// Load theme from environment variable
    ///
    /// Looks for `TERMGFX_THEME` environment variable.
    /// Falls back to Dark theme if not set or invalid.
    pub fn from_env() -> Self {
        let theme_name = env::var("TERMGFX_THEME").unwrap_or_default();
        ThemePreset::from_str(&theme_name)
            .map(Self::load_preset)
            .unwrap_or_else(Self::dark)
    }

    /// Load theme from JSON file
    pub fn load_from_file<P: AsRef<Path>>(path: P) -> Result<Self> {
        let content = fs::read_to_string(path)
            .context("Failed to read theme file")?;
        serde_json::from_str(&content)
            .context("Failed to parse theme JSON")
    }

    /// Save theme to JSON file
    pub fn save_to_file<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        let json = serde_json::to_string_pretty(self)
            .context("Failed to serialize theme")?;
        fs::write(path, json)
            .context("Failed to write theme file")?;
        Ok(())
    }

    /// Create default spacing configuration
    fn default_spacing() -> Spacing {
        Spacing {
            xs: 2,
            sm: 4,
            md: 8,
            lg: 16,
            xl: 24,
            xxl: 32,
        }
    }

    /// Create default typography configuration
    fn default_typography() -> Typography {
        Typography {
            small: 8,
            base: 12,
            large: 16,
            extra_large: 20,
            heading1: 32,
            heading2: 24,
        }
    }

    // ========================================================================
    // Theme Presets
    // ========================================================================

    /// Dark theme - default, comfortable for extended terminal use
    pub fn dark() -> Self {
        Theme {
            name: "dark".to_string(),
            description: "Dark theme - comfortable for extended use".to_string(),
            colors: Colors {
                primary: "#6366F1".to_string(),      // Indigo
                secondary: "#EC4899".to_string(),    // Pink
                success: "#10B981".to_string(),      // Emerald
                warning: "#F59E0B".to_string(),      // Amber
                danger: "#EF4444".to_string(),       // Red
                info: "#3B82F6".to_string(),         // Blue
                background: "#0F172A".to_string(),   // Slate 900
                foreground: "#F1F5F9".to_string(),   // Slate 100
                surface: "#1E293B".to_string(),      // Slate 800
                border: "#334155".to_string(),       // Slate 700
            },
            spacing: Self::default_spacing(),
            typography: Self::default_typography(),
            is_dark: true,
        }
    }

    /// Light theme - high contrast, suitable for daytime use
    pub fn light() -> Self {
        Theme {
            name: "light".to_string(),
            description: "Light theme - high contrast".to_string(),
            colors: Colors {
                primary: "#4F46E5".to_string(),      // Indigo 600
                secondary: "#DB2777".to_string(),    // Pink 600
                success: "#059669".to_string(),      // Emerald 600
                warning: "#D97706".to_string(),      // Amber 600
                danger: "#DC2626".to_string(),       // Red 600
                info: "#2563EB".to_string(),         // Blue 600
                background: "#FFFFFF".to_string(),   // White
                foreground: "#1F2937".to_string(),   // Gray 900
                surface: "#F3F4F6".to_string(),      // Gray 100
                border: "#D1D5DB".to_string(),       // Gray 300
            },
            spacing: Self::default_spacing(),
            typography: Self::default_typography(),
            is_dark: false,
        }
    }

    /// Nord theme - Arctic, north-bluish color palette
    /// https://www.nordtheme.com/
    pub fn nord() -> Self {
        Theme {
            name: "nord".to_string(),
            description: "Nord - Arctic, north-bluish palette".to_string(),
            colors: Colors {
                primary: "#88C0D0".to_string(),      // Nord 8 - Frost cyan
                secondary: "#81A1C1".to_string(),    // Nord 9 - Frost blue
                success: "#A3BE8C".to_string(),      // Nord 14 - Aurora green
                warning: "#EBCB8B".to_string(),      // Nord 13 - Aurora yellow
                danger: "#BF616A".to_string(),       // Nord 11 - Aurora red
                info: "#5E81AC".to_string(),         // Nord 10 - Frost deep blue
                background: "#2E3440".to_string(),   // Nord 0 - Polar night darkest
                foreground: "#ECEFF4".to_string(),   // Nord 6 - Snow storm lightest
                surface: "#3B4252".to_string(),      // Nord 1 - Polar night
                border: "#434C5E".to_string(),       // Nord 2 - Polar night lighter
            },
            spacing: Self::default_spacing(),
            typography: Self::default_typography(),
            is_dark: true,
        }
    }

    /// Dracula theme - Dark, sophisticated color palette
    /// https://draculatheme.com/
    pub fn dracula() -> Self {
        Theme {
            name: "dracula".to_string(),
            description: "Dracula - Dark, sophisticated palette".to_string(),
            colors: Colors {
                primary: "#8BE9FD".to_string(),      // Cyan
                secondary: "#FF79C6".to_string(),    // Pink
                success: "#50FA7B".to_string(),      // Green
                warning: "#F1FA8C".to_string(),      // Yellow
                danger: "#FF5555".to_string(),       // Red
                info: "#61AFEF".to_string(),         // Blue
                background: "#282A36".to_string(),   // Background
                foreground: "#F8F8F2".to_string(),   // Foreground
                surface: "#44475A".to_string(),      // Current line
                border: "#6272A4".to_string(),       // Comment
            },
            spacing: Self::default_spacing(),
            typography: Self::default_typography(),
            is_dark: true,
        }
    }

    /// Monokai theme - Dark with vibrant neon accents
    pub fn monokai() -> Self {
        Theme {
            name: "monokai".to_string(),
            description: "Monokai - Dark with vibrant accents".to_string(),
            colors: Colors {
                primary: "#66D9EF".to_string(),      // Blue
                secondary: "#F92672".to_string(),    // Magenta
                success: "#A6E22E".to_string(),      // Green
                warning: "#E6DB74".to_string(),      // Yellow
                danger: "#F92672".to_string(),       // Red/Magenta
                info: "#66D9EF".to_string(),         // Cyan
                background: "#272822".to_string(),   // Dark background
                foreground: "#F8F8F2".to_string(),   // Light foreground
                surface: "#3E3D32".to_string(),      // Surface
                border: "#49483E".to_string(),       // Border
            },
            spacing: Self::default_spacing(),
            typography: Self::default_typography(),
            is_dark: true,
        }
    }

    /// Solarized theme - Precision colors for machines and people
    /// https://ethanschoonover.com/solarized/
    pub fn solarized() -> Self {
        Theme {
            name: "solarized".to_string(),
            description: "Solarized - Precision colors".to_string(),
            colors: Colors {
                primary: "#268BD2".to_string(),      // Blue
                secondary: "#D33682".to_string(),    // Magenta
                success: "#859900".to_string(),      // Green
                warning: "#B58900".to_string(),      // Yellow
                danger: "#DC322F".to_string(),       // Red
                info: "#2AA198".to_string(),         // Cyan
                background: "#002B36".to_string(),   // Base 03 - dark background
                foreground: "#839496".to_string(),   // Base 0 - normal text
                surface: "#073642".to_string(),      // Base 02 - emphasized text
                border: "#586E75".to_string(),       // Base 01 - comments
            },
            spacing: Self::default_spacing(),
            typography: Self::default_typography(),
            is_dark: true,
        }
    }

    /// Gruvbox theme - Retro groove color scheme
    /// https://github.com/morhetz/gruvbox
    pub fn gruvbox() -> Self {
        Theme {
            name: "gruvbox".to_string(),
            description: "Gruvbox - Retro groove palette".to_string(),
            colors: Colors {
                primary: "#83A598".to_string(),      // Aqua
                secondary: "#D3869B".to_string(),    // Purple
                success: "#B8BB26".to_string(),      // Green
                warning: "#FABD2F".to_string(),      // Yellow
                danger: "#FB4934".to_string(),       // Red
                info: "#8EC07C".to_string(),         // Green light
                background: "#282828".to_string(),   // Dark background
                foreground: "#EBDBB2".to_string(),   // Light foreground
                surface: "#3C3836".to_string(),      // Darker background
                border: "#504945".to_string(),       // Border
            },
            spacing: Self::default_spacing(),
            typography: Self::default_typography(),
            is_dark: true,
        }
    }

    /// Get current theme from environment or default
    pub fn current() -> Self {
        Self::from_env()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_theme_preset_from_str() {
        assert_eq!(ThemePreset::from_str("dark"), Some(ThemePreset::Dark));
        assert_eq!(ThemePreset::from_str("Dark"), Some(ThemePreset::Dark));
        assert_eq!(ThemePreset::from_str("DARK"), Some(ThemePreset::Dark));
        assert_eq!(ThemePreset::from_str("nord"), Some(ThemePreset::Nord));
        assert_eq!(ThemePreset::from_str("invalid"), None);
    }

    #[test]
    fn test_theme_preset_as_str() {
        assert_eq!(ThemePreset::Dark.as_str(), "dark");
        assert_eq!(ThemePreset::Light.as_str(), "light");
        assert_eq!(ThemePreset::Nord.as_str(), "nord");
    }

    #[test]
    fn test_theme_preset_all() {
        let presets = ThemePreset::all();
        assert_eq!(presets.len(), 7);
        assert!(presets.contains(&ThemePreset::Dark));
        assert!(presets.contains(&ThemePreset::Dracula));
    }

    #[test]
    fn test_load_dark_theme() {
        let theme = Theme::load_preset(ThemePreset::Dark);
        assert_eq!(theme.name, "dark");
        assert!(theme.is_dark);
        assert!(!theme.colors.primary.is_empty());
    }

    #[test]
    fn test_load_light_theme() {
        let theme = Theme::load_preset(ThemePreset::Light);
        assert_eq!(theme.name, "light");
        assert!(!theme.is_dark);
    }

    #[test]
    fn test_load_all_presets() {
        for preset in ThemePreset::all() {
            let theme = Theme::load_preset(preset);
            assert!(!theme.name.is_empty());
            assert!(!theme.description.is_empty());
            assert!(!theme.colors.primary.is_empty());
        }
    }

    #[test]
    fn test_theme_serialization() {
        let theme = Theme::dark();
        let json = serde_json::to_string(&theme).unwrap();
        let deserialized: Theme = serde_json::from_str(&json).unwrap();
        assert_eq!(theme.name, deserialized.name);
        assert_eq!(theme.colors.primary, deserialized.colors.primary);
    }

    #[test]
    fn test_theme_colors_are_valid_hex() {
        let theme = Theme::dark();
        assert!(theme.colors.primary.starts_with('#'));
        assert_eq!(theme.colors.primary.len(), 7); // #RRGGBB
    }

    #[test]
    fn test_default_spacing() {
        let spacing = Theme::default_spacing();
        assert_eq!(spacing.xs, 2);
        assert_eq!(spacing.sm, 4);
        assert_eq!(spacing.md, 8);
        assert_eq!(spacing.lg, 16);
        assert_eq!(spacing.xl, 24);
        assert_eq!(spacing.xxl, 32);
    }

    #[test]
    fn test_default_typography() {
        let typography = Theme::default_typography();
        assert_eq!(typography.small, 8);
        assert_eq!(typography.base, 12);
        assert_eq!(typography.large, 16);
        assert_eq!(typography.extra_large, 20);
        assert_eq!(typography.heading1, 32);
        assert_eq!(typography.heading2, 24);
    }

    #[test]
    fn test_theme_file_roundtrip() {
        use std::fs;
        use tempfile::NamedTempFile;

        let theme = Theme::nord();
        let temp_file = NamedTempFile::new().unwrap();
        let path = temp_file.path();

        // Save theme
        theme.save_to_file(path).unwrap();

        // Load theme
        let loaded = Theme::load_from_file(path).unwrap();

        // Verify
        assert_eq!(theme.name, loaded.name);
        assert_eq!(theme.colors.primary, loaded.colors.primary);
        assert_eq!(theme.is_dark, loaded.is_dark);

        // Cleanup
        fs::remove_file(path).ok();
    }

    #[test]
    fn test_nord_theme_colors() {
        let theme = Theme::nord();
        assert_eq!(theme.colors.primary, "#88C0D0");
        assert_eq!(theme.colors.success, "#A3BE8C");
        assert_eq!(theme.colors.danger, "#BF616A");
    }

    #[test]
    fn test_dracula_theme_colors() {
        let theme = Theme::dracula();
        assert_eq!(theme.colors.primary, "#8BE9FD");
        assert_eq!(theme.colors.success, "#50FA7B");
    }

    #[test]
    fn test_all_themes_have_valid_structure() {
        for preset in ThemePreset::all() {
            let theme = Theme::load_preset(preset);

            // Verify all color fields are populated
            assert!(!theme.colors.primary.is_empty());
            assert!(!theme.colors.secondary.is_empty());
            assert!(!theme.colors.success.is_empty());
            assert!(!theme.colors.warning.is_empty());
            assert!(!theme.colors.danger.is_empty());
            assert!(!theme.colors.info.is_empty());
            assert!(!theme.colors.background.is_empty());
            assert!(!theme.colors.foreground.is_empty());
            assert!(!theme.colors.surface.is_empty());
            assert!(!theme.colors.border.is_empty());

            // Verify spacing is positive
            assert!(theme.spacing.xs > 0);
            assert!(theme.spacing.sm > 0);
            assert!(theme.spacing.md > 0);

            // Verify typography is positive
            assert!(theme.typography.small > 0);
            assert!(theme.typography.base > 0);
            assert!(theme.typography.large > 0);
        }
    }

    #[test]
    fn test_theme_from_env_with_valid_var() {
        env::set_var("TERMGFX_THEME", "nord");
        let theme = Theme::from_env();
        assert_eq!(theme.name, "nord");
        env::remove_var("TERMGFX_THEME");
    }

    #[test]
    fn test_theme_from_env_with_invalid_var() {
        env::set_var("TERMGFX_THEME", "invalid_theme");
        let theme = Theme::from_env();
        assert_eq!(theme.name, "dark"); // Falls back to dark
        env::remove_var("TERMGFX_THEME");
    }

    #[test]
    fn test_theme_from_env_unset() {
        env::remove_var("TERMGFX_THEME");
        let theme = Theme::from_env();
        assert_eq!(theme.name, "dark"); // Falls back to dark
    }
}
