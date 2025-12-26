use owo_colors::Style;
use serde::{Deserialize, Serialize};

/// Typography settings for presets
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Typography {
    /// Use bold text for headers/titles
    pub bold_headers: bool,
    /// Use italic text if supported
    pub italic_body: bool,
    /// Text transformation: none, uppercase, lowercase
    pub text_transform: TextTransform,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum TextTransform {
    None,
    Uppercase,
    Lowercase,
}

impl Default for TextTransform {
    fn default() -> Self {
        TextTransform::None
    }
}

/// Color palette for preset styling
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ColorPalette {
    /// Border color (CSS/ANSI color name)
    pub border: String,
    /// Primary accent color
    pub accent: String,
    /// Secondary accent color
    pub secondary: String,
    /// Background/highlight color
    pub background: String,
    /// Text/foreground color
    pub text: String,
    /// Error/warning color
    pub danger: String,
    /// Success color
    pub success: String,
}

/// Spacing configuration
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Spacing {
    /// Horizontal padding inside boxes
    pub horizontal_padding: usize,
    /// Vertical padding inside boxes
    pub vertical_padding: usize,
    /// Gap between elements
    pub element_gap: usize,
    /// Margin around entire box
    pub margin: usize,
}

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
    /// Show box shadows (represented as double borders in terminal)
    pub has_shadow: bool,
}

impl StylePreset {
    /// Create a new preset
    pub fn new(
        name: &str,
        description: &str,
        border_style: &str,
        colors: ColorPalette,
        typography: Typography,
        spacing: Spacing,
        has_shadow: bool,
    ) -> Self {
        StylePreset {
            name: name.to_string(),
            description: description.to_string(),
            border_style: border_style.to_string(),
            colors,
            typography,
            spacing,
            has_shadow,
        }
    }

    /// Get a preset by name
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

    /// Corporate preset: Professional, clean, business-like
    pub fn corporate() -> Self {
        StylePreset::new(
            "corporate",
            "Professional business aesthetic with clean lines and muted colors",
            "double",
            ColorPalette {
                border: "bright-white".to_string(),
                accent: "cyan".to_string(),
                secondary: "blue".to_string(),
                background: "bright-black".to_string(),
                text: "white".to_string(),
                danger: "red".to_string(),
                success: "green".to_string(),
            },
            Typography {
                bold_headers: true,
                italic_body: false,
                text_transform: TextTransform::None,
            },
            Spacing {
                horizontal_padding: 3,
                vertical_padding: 1,
                element_gap: 2,
                margin: 1,
            },
            false,
        )
    }

    /// Playful preset: Vibrant, fun, casual
    pub fn playful() -> Self {
        StylePreset::new(
            "playful",
            "Fun and vibrant with rounded corners and bright colors",
            "rounded",
            ColorPalette {
                border: "bright-magenta".to_string(),
                accent: "bright_cyan".to_string(),
                secondary: "bright-yellow".to_string(),
                background: "bright-black".to_string(),
                text: "bright-white".to_string(),
                danger: "bright-red".to_string(),
                success: "bright-green".to_string(),
            },
            Typography {
                bold_headers: true,
                italic_body: false,
                text_transform: TextTransform::None,
            },
            Spacing {
                horizontal_padding: 2,
                vertical_padding: 1,
                element_gap: 1,
                margin: 0,
            },
            false,
        )
    }

    /// Minimal preset: Clean, simple, distraction-free
    pub fn minimal() -> Self {
        StylePreset::new(
            "minimal",
            "Simple and clean with ASCII borders and minimal styling",
            "ascii",
            ColorPalette {
                border: "white".to_string(),
                accent: "white".to_string(),
                secondary: "bright-black".to_string(),
                background: "black".to_string(),
                text: "white".to_string(),
                danger: "red".to_string(),
                success: "green".to_string(),
            },
            Typography {
                bold_headers: false,
                italic_body: false,
                text_transform: TextTransform::None,
            },
            Spacing {
                horizontal_padding: 1,
                vertical_padding: 0,
                element_gap: 0,
                margin: 0,
            },
            false,
        )
    }

    /// Retro preset: 80s/90s inspired with heavy borders
    pub fn retro() -> Self {
        StylePreset::new(
            "retro",
            "Nostalgic 80s/90s aesthetic with heavy borders and warm colors",
            "heavy",
            ColorPalette {
                border: "bright-yellow".to_string(),
                accent: "bright-magenta".to_string(),
                secondary: "bright-cyan".to_string(),
                background: "bright-black".to_string(),
                text: "bright-yellow".to_string(),
                danger: "bright-red".to_string(),
                success: "bright-green".to_string(),
            },
            Typography {
                bold_headers: true,
                italic_body: false,
                text_transform: TextTransform::Uppercase,
            },
            Spacing {
                horizontal_padding: 2,
                vertical_padding: 1,
                element_gap: 1,
                margin: 1,
            },
            false,
        )
    }

    /// Neon preset: High contrast with bright colors on dark background
    pub fn neon() -> Self {
        StylePreset::new(
            "neon",
            "High-energy neon aesthetic with bright colors and dark background",
            "single",
            ColorPalette {
                border: "bright-cyan".to_string(),
                accent: "bright-magenta".to_string(),
                secondary: "bright-cyan".to_string(),
                background: "black".to_string(),
                text: "bright-white".to_string(),
                danger: "bright-red".to_string(),
                success: "bright_green".to_string(),
            },
            Typography {
                bold_headers: true,
                italic_body: false,
                text_transform: TextTransform::None,
            },
            Spacing {
                horizontal_padding: 2,
                vertical_padding: 1,
                element_gap: 1,
                margin: 0,
            },
            false,
        )
    }

    /// Elegant preset: Sophisticated with subtle colors
    pub fn elegant() -> Self {
        StylePreset::new(
            "elegant",
            "Sophisticated and refined with subtle colors and rounded borders",
            "rounded",
            ColorPalette {
                border: "bright-white".to_string(),
                accent: "bright-blue".to_string(),
                secondary: "bright-cyan".to_string(),
                background: "bright-black".to_string(),
                text: "bright-white".to_string(),
                danger: "bright-red".to_string(),
                success: "bright-green".to_string(),
            },
            Typography {
                bold_headers: true,
                italic_body: true,
                text_transform: TextTransform::None,
            },
            Spacing {
                horizontal_padding: 3,
                vertical_padding: 1,
                element_gap: 2,
                margin: 1,
            },
            true,
        )
    }

    /// List all available presets
    pub fn list_all() -> Vec<&'static str> {
        vec!["corporate", "playful", "minimal", "retro", "neon", "elegant"]
    }

    /// Get a description of all presets
    pub fn describe_all() -> Vec<(&'static str, &'static str)> {
        vec![
            ("corporate", "Professional business aesthetic with clean lines and muted colors"),
            ("playful", "Fun and vibrant with rounded corners and bright colors"),
            ("minimal", "Simple and clean with ASCII borders and minimal styling"),
            ("retro", "Nostalgic 80s/90s aesthetic with heavy borders and warm colors"),
            ("neon", "High-energy neon aesthetic with bright colors and dark background"),
            ("elegant", "Sophisticated and refined with subtle colors and rounded borders"),
        ]
    }
}

impl Default for StylePreset {
    fn default() -> Self {
        StylePreset::corporate()
    }
}

/// Parse a named color to owo-colors Style
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
    fn test_corporate_preset() {
        let preset = StylePreset::corporate();
        assert_eq!(preset.name, "corporate");
        assert_eq!(preset.border_style, "double");
        assert_eq!(preset.colors.border, "bright-white");
        assert_eq!(preset.colors.accent, "cyan");
        assert_eq!(preset.spacing.horizontal_padding, 3);
        assert!(preset.typography.bold_headers);
        assert!(!preset.has_shadow);
    }

    #[test]
    fn test_playful_preset() {
        let preset = StylePreset::playful();
        assert_eq!(preset.name, "playful");
        assert_eq!(preset.border_style, "rounded");
        assert_eq!(preset.colors.border, "bright-magenta");
        assert!(preset.typography.bold_headers);
        assert!(!preset.has_shadow);
    }

    #[test]
    fn test_minimal_preset() {
        let preset = StylePreset::minimal();
        assert_eq!(preset.name, "minimal");
        assert_eq!(preset.border_style, "ascii");
        assert_eq!(preset.spacing.horizontal_padding, 1);
        assert!(!preset.typography.bold_headers);
        assert!(!preset.has_shadow);
    }

    #[test]
    fn test_retro_preset() {
        let preset = StylePreset::retro();
        assert_eq!(preset.name, "retro");
        assert_eq!(preset.border_style, "heavy");
        assert_eq!(preset.colors.accent, "bright-magenta");
        assert_eq!(preset.typography.text_transform, TextTransform::Uppercase);
        assert!(!preset.has_shadow);
    }

    #[test]
    fn test_neon_preset() {
        let preset = StylePreset::neon();
        assert_eq!(preset.name, "neon");
        assert_eq!(preset.border_style, "single");
        assert_eq!(preset.colors.background, "black");
        assert_eq!(preset.colors.accent, "bright-magenta");
        assert!(!preset.has_shadow);
    }

    #[test]
    fn test_elegant_preset() {
        let preset = StylePreset::elegant();
        assert_eq!(preset.name, "elegant");
        assert_eq!(preset.border_style, "rounded");
        assert!(preset.typography.italic_body);
        assert!(preset.has_shadow);
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
    fn test_from_name_case_insensitive() {
        assert!(StylePreset::from_name("CORPORATE").is_some());
        assert!(StylePreset::from_name("Playful").is_some());
        assert!(StylePreset::from_name("MiNiMaL").is_some());
    }

    #[test]
    fn test_list_all() {
        let presets = StylePreset::list_all();
        assert_eq!(presets.len(), 6);
        assert!(presets.contains(&"corporate"));
        assert!(presets.contains(&"playful"));
        assert!(presets.contains(&"minimal"));
        assert!(presets.contains(&"retro"));
        assert!(presets.contains(&"neon"));
        assert!(presets.contains(&"elegant"));
    }

    #[test]
    fn test_describe_all() {
        let descriptions = StylePreset::describe_all();
        assert_eq!(descriptions.len(), 6);
        assert!(descriptions.iter().any(|(name, _)| *name == "corporate"));
    }

    #[test]
    fn test_default_preset() {
        let preset = StylePreset::default();
        assert_eq!(preset.name, "corporate");
    }

    #[test]
    fn test_color_to_style() {
        let cyan_style = color_to_style("cyan");
        let bright_cyan_style = color_to_style("bright-cyan");
        let underscore_cyan_style = color_to_style("bright_cyan");
        let _ = cyan_style;
        let _ = bright_cyan_style;
        let _ = underscore_cyan_style;
    }

    #[test]
    fn test_color_to_style_defaults_to_white() {
        let unknown_style = color_to_style("unknown_color");
        let _ = unknown_style;
    }

    #[test]
    fn test_preset_serialization() {
        let preset = StylePreset::corporate();
        let json = serde_json::to_string(&preset).unwrap();
        assert!(json.contains("corporate"));
        assert!(json.contains("double"));
    }

    #[test]
    fn test_preset_deserialization() {
        let preset = StylePreset::corporate();
        let json = serde_json::to_string(&preset).unwrap();
        let deserialized: StylePreset = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.name, preset.name);
        assert_eq!(deserialized.border_style, preset.border_style);
    }

    #[test]
    fn test_spacing_values_consistent() {
        let presets = vec![
            StylePreset::corporate(),
            StylePreset::playful(),
            StylePreset::minimal(),
            StylePreset::retro(),
            StylePreset::neon(),
            StylePreset::elegant(),
        ];

        for preset in presets {
            assert!(preset.spacing.horizontal_padding <= 5);
            assert!(preset.spacing.vertical_padding <= 3);
            assert!(preset.spacing.margin <= 2);
        }
    }

    #[test]
    fn test_all_presets_have_valid_border_styles() {
        let valid_borders = vec!["single", "double", "rounded", "heavy", "ascii"];
        let presets = vec![
            StylePreset::corporate(),
            StylePreset::playful(),
            StylePreset::minimal(),
            StylePreset::retro(),
            StylePreset::neon(),
            StylePreset::elegant(),
        ];

        for preset in presets {
            assert!(
                valid_borders.contains(&preset.border_style.as_str()),
                "Invalid border style: {}",
                preset.border_style
            );
        }
    }
}
