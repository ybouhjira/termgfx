use owo_colors::Style;
use serde::{Deserialize, Serialize};

/// A complete style preset
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StylePreset {
    /// Preset name/identifier
    pub name: String,
    /// Description of the preset's visual style
    pub description: String,
    /// Border style: single, double, rounded, heavy, ascii
    pub border_style: String,
    /// Color palette
    pub colors: ColorPalette,
    /// Typography settings
    pub typography: Typography,
    /// Spacing settings
    pub spacing: Spacing,
    /// Show box shadows
    pub has_shadow: bool,
}

/// Typography settings for presets
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Typography {
    pub bold_headers: bool,
    pub italic_body: bool,
    pub text_transform: TextTransform,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum TextTransform {
    None,
    Uppercase,
    Lowercase,
}

/// Color palette for preset styling
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ColorPalette {
    pub border: String,
    pub accent: String,
    pub secondary: String,
    pub background: String,
    pub text: String,
    pub danger: String,
    pub success: String,
}

/// Spacing configuration
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Spacing {
    pub horizontal_padding: usize,
    pub vertical_padding: usize,
    pub element_gap: usize,
    pub margin: usize,
}

impl StylePreset {
    pub fn corporate() -> Self {
        StylePreset {
            name: "corporate".to_string(),
            description: "Professional business aesthetic".to_string(),
            border_style: "double".to_string(),
            colors: ColorPalette {
                border: "bright-white".to_string(),
                accent: "cyan".to_string(),
                secondary: "blue".to_string(),
                background: "bright-black".to_string(),
                text: "white".to_string(),
                danger: "red".to_string(),
                success: "green".to_string(),
            },
            typography: Typography {
                bold_headers: true,
                italic_body: false,
                text_transform: TextTransform::None,
            },
            spacing: Spacing {
                horizontal_padding: 3,
                vertical_padding: 1,
                element_gap: 2,
                margin: 1,
            },
            has_shadow: false,
        }
    }

    pub fn playful() -> Self {
        StylePreset {
            name: "playful".to_string(),
            description: "Fun and vibrant aesthetic".to_string(),
            border_style: "rounded".to_string(),
            colors: ColorPalette {
                border: "bright-magenta".to_string(),
                accent: "bright_cyan".to_string(),
                secondary: "bright-yellow".to_string(),
                background: "bright-black".to_string(),
                text: "bright-white".to_string(),
                danger: "bright-red".to_string(),
                success: "bright-green".to_string(),
            },
            typography: Typography {
                bold_headers: true,
                italic_body: false,
                text_transform: TextTransform::None,
            },
            spacing: Spacing {
                horizontal_padding: 2,
                vertical_padding: 1,
                element_gap: 1,
                margin: 0,
            },
            has_shadow: false,
        }
    }

    pub fn minimal() -> Self {
        StylePreset {
            name: "minimal".to_string(),
            description: "Simple and clean aesthetic".to_string(),
            border_style: "ascii".to_string(),
            colors: ColorPalette {
                border: "white".to_string(),
                accent: "white".to_string(),
                secondary: "bright-black".to_string(),
                background: "black".to_string(),
                text: "white".to_string(),
                danger: "red".to_string(),
                success: "green".to_string(),
            },
            typography: Typography {
                bold_headers: false,
                italic_body: false,
                text_transform: TextTransform::None,
            },
            spacing: Spacing {
                horizontal_padding: 1,
                vertical_padding: 0,
                element_gap: 0,
                margin: 0,
            },
            has_shadow: false,
        }
    }

    pub fn retro() -> Self {
        StylePreset {
            name: "retro".to_string(),
            description: "80s/90s nostalgic aesthetic".to_string(),
            border_style: "heavy".to_string(),
            colors: ColorPalette {
                border: "bright-yellow".to_string(),
                accent: "bright-magenta".to_string(),
                secondary: "bright-cyan".to_string(),
                background: "bright-black".to_string(),
                text: "bright-yellow".to_string(),
                danger: "bright-red".to_string(),
                success: "bright-green".to_string(),
            },
            typography: Typography {
                bold_headers: true,
                italic_body: false,
                text_transform: TextTransform::Uppercase,
            },
            spacing: Spacing {
                horizontal_padding: 2,
                vertical_padding: 1,
                element_gap: 1,
                margin: 1,
            },
            has_shadow: false,
        }
    }

    pub fn neon() -> Self {
        StylePreset {
            name: "neon".to_string(),
            description: "High-energy neon aesthetic".to_string(),
            border_style: "single".to_string(),
            colors: ColorPalette {
                border: "bright-cyan".to_string(),
                accent: "bright-magenta".to_string(),
                secondary: "bright-cyan".to_string(),
                background: "black".to_string(),
                text: "bright-white".to_string(),
                danger: "bright-red".to_string(),
                success: "bright_green".to_string(),
            },
            typography: Typography {
                bold_headers: true,
                italic_body: false,
                text_transform: TextTransform::None,
            },
            spacing: Spacing {
                horizontal_padding: 2,
                vertical_padding: 1,
                element_gap: 1,
                margin: 0,
            },
            has_shadow: false,
        }
    }

    pub fn elegant() -> Self {
        StylePreset {
            name: "elegant".to_string(),
            description: "Sophisticated and refined aesthetic".to_string(),
            border_style: "rounded".to_string(),
            colors: ColorPalette {
                border: "bright-white".to_string(),
                accent: "bright-blue".to_string(),
                secondary: "bright-cyan".to_string(),
                background: "bright-black".to_string(),
                text: "bright-white".to_string(),
                danger: "bright-red".to_string(),
                success: "bright-green".to_string(),
            },
            typography: Typography {
                bold_headers: true,
                italic_body: true,
                text_transform: TextTransform::None,
            },
            spacing: Spacing {
                horizontal_padding: 3,
                vertical_padding: 1,
                element_gap: 2,
                margin: 1,
            },
            has_shadow: true,
        }
    }

    pub fn from_name(name: &str) -> Option<Self> {
        match name.to_lowercase().as_str() {
            "corporate" => Some(Self::corporate()),
            "playful" => Some(Self::playful()),
            "minimal" => Some(Self::minimal()),
            "retro" => Some(Self::retro()),
            "neon" => Some(Self::neon()),
            "elegant" => Some(Self::elegant()),
            _ => None,
        }
    }

    pub fn list_all() -> Vec<&'static str> {
        vec!["corporate", "playful", "minimal", "retro", "neon", "elegant"]
    }
}

impl Default for StylePreset {
    fn default() -> Self {
        StylePreset::corporate()
    }
}

impl Default for TextTransform {
    fn default() -> Self {
        TextTransform::None
    }
}

pub fn color_to_style(color_name: &str) -> Style {
    match color_name.to_lowercase().as_str() {
        "cyan" | "bright-cyan" | "bright_cyan" => Style::new().cyan(),
        "blue" | "bright-blue" | "bright_blue" => Style::new().blue(),
        "magenta" | "bright-magenta" | "bright_magenta" => Style::new().magenta(),
        "yellow" | "bright-yellow" | "bright_yellow" => Style::new().yellow(),
        "red" | "bright-red" | "bright_red" => Style::new().red(),
        "green" | "bright-green" | "bright_green" => Style::new().green(),
        "white" | "bright-white" | "bright_white" => Style::new().white(),
        "black" | "bright-black" | "bright_black" => Style::new().black(),
        _ => Style::new().white(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_presets() {
        let presets = vec![
            StylePreset::corporate(),
            StylePreset::playful(),
            StylePreset::minimal(),
            StylePreset::retro(),
            StylePreset::neon(),
            StylePreset::elegant(),
        ];
        assert_eq!(presets.len(), 6);
    }

    #[test]
    fn test_from_name() {
        assert!(StylePreset::from_name("corporate").is_some());
        assert!(StylePreset::from_name("playful").is_some());
        assert!(StylePreset::from_name("minimal").is_some());
        assert!(StylePreset::from_name("retro").is_some());
        assert!(StylePreset::from_name("neon").is_some());
        assert!(StylePreset::from_name("elegant").is_some());
        assert!(StylePreset::from_name("unknown").is_none());
    }

    #[test]
    fn test_list_all() {
        let presets = StylePreset::list_all();
        assert_eq!(presets.len(), 6);
        assert!(presets.contains(&"corporate"));
        assert!(presets.contains(&"elegant"));
    }
}
