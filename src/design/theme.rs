//! Theme system for termgfx - built-in presets and customizable themes

use serde::{Deserialize, Serialize};
use std::path::Path;
use std::fs;
use std::env;
use anyhow::{Result, Context};

/// Color configuration for a theme
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Colors {
    pub primary: String,
    pub secondary: String,
    pub success: String,
    pub warning: String,
    pub danger: String,
    pub info: String,
    pub background: String,
    pub foreground: String,
    pub surface: String,
    pub border: String,
}

/// Spacing configuration for a theme
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Spacing {
    pub xs: usize,
    pub sm: usize,
    pub md: usize,
    pub lg: usize,
    pub xl: usize,
    pub xxl: usize,
}

/// Typography configuration for a theme
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Typography {
    pub small: usize,
    pub base: usize,
    pub large: usize,
    pub extra_large: usize,
    pub heading1: usize,
    pub heading2: usize,
}

/// Complete theme configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Theme {
    pub name: String,
    pub description: String,
    pub colors: Colors,
    pub spacing: Spacing,
    pub typography: Typography,
    pub is_dark: bool,
}

/// Available built-in theme presets
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ThemePreset {
    Dark,
    Light,
    Nord,
    Dracula,
    Monokai,
    Solarized,
    Gruvbox,
}

impl ThemePreset {
    pub fn all() -> Vec<ThemePreset> {
        vec![ThemePreset::Dark, ThemePreset::Light, ThemePreset::Nord, ThemePreset::Dracula,
             ThemePreset::Monokai, ThemePreset::Solarized, ThemePreset::Gruvbox]
    }

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

    pub fn from_env() -> Self {
        let theme_name = env::var("TERMGFX_THEME").unwrap_or_default();
        ThemePreset::from_str(&theme_name).map(Self::load_preset).unwrap_or_else(Self::dark)
    }

    pub fn load_from_file<P: AsRef<Path>>(path: P) -> Result<Self> {
        let content = fs::read_to_string(path).context("Failed to read theme file")?;
        serde_json::from_str(&content).context("Failed to parse theme JSON")
    }

    pub fn save_to_file<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        let json = serde_json::to_string_pretty(self).context("Failed to serialize theme")?;
        fs::write(path, json).context("Failed to write theme file")?;
        Ok(())
    }

    fn default_spacing() -> Spacing {
        Spacing { xs: 2, sm: 4, md: 8, lg: 16, xl: 24, xxl: 32 }
    }

    fn default_typography() -> Typography {
        Typography { small: 8, base: 12, large: 16, extra_large: 20, heading1: 32, heading2: 24 }
    }

    pub fn dark() -> Self {
        Theme {
            name: "dark".to_string(),
            description: "Dark theme - comfortable for extended use".to_string(),
            colors: Colors {
                primary: "#6366F1".to_string(),
                secondary: "#EC4899".to_string(),
                success: "#10B981".to_string(),
                warning: "#F59E0B".to_string(),
                danger: "#EF4444".to_string(),
                info: "#3B82F6".to_string(),
                background: "#0F172A".to_string(),
                foreground: "#F1F5F9".to_string(),
                surface: "#1E293B".to_string(),
                border: "#334155".to_string(),
            },
            spacing: Self::default_spacing(),
            typography: Self::default_typography(),
            is_dark: true,
        }
    }

    pub fn light() -> Self {
        Theme {
            name: "light".to_string(),
            description: "Light theme - high contrast".to_string(),
            colors: Colors {
                primary: "#4F46E5".to_string(),
                secondary: "#DB2777".to_string(),
                success: "#059669".to_string(),
                warning: "#D97706".to_string(),
                danger: "#DC2626".to_string(),
                info: "#2563EB".to_string(),
                background: "#FFFFFF".to_string(),
                foreground: "#1F2937".to_string(),
                surface: "#F3F4F6".to_string(),
                border: "#D1D5DB".to_string(),
            },
            spacing: Self::default_spacing(),
            typography: Self::default_typography(),
            is_dark: false,
        }
    }

    pub fn nord() -> Self {
        Theme {
            name: "nord".to_string(),
            description: "Nord - Arctic, north-bluish palette".to_string(),
            colors: Colors {
                primary: "#88C0D0".to_string(),
                secondary: "#81A1C1".to_string(),
                success: "#A3BE8C".to_string(),
                warning: "#EBCB8B".to_string(),
                danger: "#BF616A".to_string(),
                info: "#5E81AC".to_string(),
                background: "#2E3440".to_string(),
                foreground: "#ECEFF4".to_string(),
                surface: "#3B4252".to_string(),
                border: "#434C5E".to_string(),
            },
            spacing: Self::default_spacing(),
            typography: Self::default_typography(),
            is_dark: true,
        }
    }

    pub fn dracula() -> Self {
        Theme {
            name: "dracula".to_string(),
            description: "Dracula - Dark, sophisticated palette".to_string(),
            colors: Colors {
                primary: "#8BE9FD".to_string(),
                secondary: "#FF79C6".to_string(),
                success: "#50FA7B".to_string(),
                warning: "#F1FA8C".to_string(),
                danger: "#FF5555".to_string(),
                info: "#61AFEF".to_string(),
                background: "#282A36".to_string(),
                foreground: "#F8F8F2".to_string(),
                surface: "#44475A".to_string(),
                border: "#6272A4".to_string(),
            },
            spacing: Self::default_spacing(),
            typography: Self::default_typography(),
            is_dark: true,
        }
    }

    pub fn monokai() -> Self {
        Theme {
            name: "monokai".to_string(),
            description: "Monokai - Dark with vibrant accents".to_string(),
            colors: Colors {
                primary: "#66D9EF".to_string(),
                secondary: "#F92672".to_string(),
                success: "#A6E22E".to_string(),
                warning: "#E6DB74".to_string(),
                danger: "#F92672".to_string(),
                info: "#66D9EF".to_string(),
                background: "#272822".to_string(),
                foreground: "#F8F8F2".to_string(),
                surface: "#3E3D32".to_string(),
                border: "#49483E".to_string(),
            },
            spacing: Self::default_spacing(),
            typography: Self::default_typography(),
            is_dark: true,
        }
    }

    pub fn solarized() -> Self {
        Theme {
            name: "solarized".to_string(),
            description: "Solarized - Precision colors".to_string(),
            colors: Colors {
                primary: "#268BD2".to_string(),
                secondary: "#D33682".to_string(),
                success: "#859900".to_string(),
                warning: "#B58900".to_string(),
                danger: "#DC322F".to_string(),
                info: "#2AA198".to_string(),
                background: "#002B36".to_string(),
                foreground: "#839496".to_string(),
                surface: "#073642".to_string(),
                border: "#586E75".to_string(),
            },
            spacing: Self::default_spacing(),
            typography: Self::default_typography(),
            is_dark: true,
        }
    }

    pub fn gruvbox() -> Self {
        Theme {
            name: "gruvbox".to_string(),
            description: "Gruvbox - Retro groove palette".to_string(),
            colors: Colors {
                primary: "#83A598".to_string(),
                secondary: "#D3869B".to_string(),
                success: "#B8BB26".to_string(),
                warning: "#FABD2F".to_string(),
                danger: "#FB4934".to_string(),
                info: "#8EC07C".to_string(),
                background: "#282828".to_string(),
                foreground: "#EBDBB2".to_string(),
                surface: "#3C3836".to_string(),
                border: "#504945".to_string(),
            },
            spacing: Self::default_spacing(),
            typography: Self::default_typography(),
            is_dark: true,
        }
    }

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
        }
    }

    #[test]
    fn test_theme_serialization() {
        let theme = Theme::dark();
        let json = serde_json::to_string(&theme).unwrap();
        let deserialized: Theme = serde_json::from_str(&json).unwrap();
        assert_eq!(theme.name, deserialized.name);
    }

    #[test]
    fn test_default_spacing() {
        let spacing = Theme::default_spacing();
        assert_eq!(spacing.xs, 2);
        assert_eq!(spacing.md, 8);
    }

    #[test]
    fn test_default_typography() {
        let typography = Theme::default_typography();
        assert_eq!(typography.small, 8);
        assert_eq!(typography.base, 12);
    }

    #[test]
    fn test_nord_theme_colors() {
        let theme = Theme::nord();
        assert_eq!(theme.colors.primary, "#88C0D0");
        assert_eq!(theme.colors.success, "#A3BE8C");
    }

    #[test]
    fn test_dracula_theme_colors() {
        let theme = Theme::dracula();
        assert_eq!(theme.colors.primary, "#8BE9FD");
        assert_eq!(theme.colors.success, "#50FA7B");
    }

    #[test]
    fn test_all_themes_valid() {
        for preset in ThemePreset::all() {
            let theme = Theme::load_preset(preset);
            assert!(!theme.colors.primary.is_empty());
            assert!(theme.spacing.xs > 0);
            assert!(theme.typography.small > 0);
        }
    }

    #[test]
    fn test_theme_from_env_nord() {
        env::set_var("TERMGFX_THEME", "nord");
        let theme = Theme::from_env();
        assert_eq!(theme.name, "nord");
        env::remove_var("TERMGFX_THEME");
    }

    #[test]
    fn test_theme_from_env_fallback() {
        env::set_var("TERMGFX_THEME", "invalid");
        let theme = Theme::from_env();
        assert_eq!(theme.name, "dark");
        env::remove_var("TERMGFX_THEME");
    }
}
