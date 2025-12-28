use owo_colors::{OwoColorize, Style};
use std::io::{stdout, Write};

/// Border style type for presets
#[derive(Debug, Clone, Copy, PartialEq)]
#[allow(dead_code)]
pub enum BorderStyle {
    Single,  // ┌─┐│└─┘
    Double,  // ╔═╗║╚═╝
    Rounded, // ╭─╮│╰─╯
    Thick,   // ┏━┓┃┗━┛
    Ascii,   // +-+|+-+
    None,    // No border
}

impl BorderStyle {
    /// Get border characters: (top_left, horizontal, top_right, vertical, bottom_left, bottom_right)
    pub fn chars(
        &self,
    ) -> (
        &'static str,
        &'static str,
        &'static str,
        &'static str,
        &'static str,
        &'static str,
    ) {
        match self {
            BorderStyle::Single => ("┌", "─", "┐", "│", "└", "┘"),
            BorderStyle::Double => ("╔", "═", "╗", "║", "╚", "╝"),
            BorderStyle::Rounded => ("╭", "─", "╮", "│", "╰", "╯"),
            BorderStyle::Thick => ("┏", "━", "┓", "┃", "┗", "┛"),
            BorderStyle::Ascii => ("+", "-", "+", "|", "+", "+"),
            BorderStyle::None => ("", "", "", "", "", ""),
        }
    }
}

/// Color scheme for a preset
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct ColorScheme {
    pub primary: (u8, u8, u8),            // RGB for main text/elements
    pub secondary: (u8, u8, u8),          // RGB for accents
    pub border: (u8, u8, u8),             // RGB for borders
    pub background: Option<(u8, u8, u8)>, // Optional background
}

/// Style preset with full styling configuration
#[derive(Debug, Clone)]
pub struct StylePreset {
    pub name: &'static str,
    pub emoji: &'static str,
    pub description: &'static str,
    pub border_style: BorderStyle,
    pub colors: ColorScheme,
    pub category: PresetCategory,
}

/// Category of preset for organization
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PresetCategory {
    Semantic, // info, success, warning, danger
    Design,   // corporate, playful, minimal, retro
    Other,    // gradient, neutral
}

impl StylePreset {
    /// Get all available style presets
    pub fn all() -> Vec<StylePreset> {
        vec![
            // Semantic presets
            StylePreset {
                name: "info",
                emoji: "ℹ",
                description: "Information messages",
                border_style: BorderStyle::Rounded,
                colors: ColorScheme {
                    primary: (0, 191, 255),     // Deep sky blue
                    secondary: (135, 206, 250), // Light sky blue
                    border: (100, 149, 237),    // Cornflower blue
                    background: None,
                },
                category: PresetCategory::Semantic,
            },
            StylePreset {
                name: "success",
                emoji: "✓",
                description: "Success/completed state",
                border_style: BorderStyle::Rounded,
                colors: ColorScheme {
                    primary: (0, 255, 127),     // Spring green
                    secondary: (144, 238, 144), // Light green
                    border: (50, 205, 50),      // Lime green
                    background: None,
                },
                category: PresetCategory::Semantic,
            },
            StylePreset {
                name: "warning",
                emoji: "⚠",
                description: "Warning messages",
                border_style: BorderStyle::Single,
                colors: ColorScheme {
                    primary: (255, 215, 0),   // Gold
                    secondary: (255, 165, 0), // Orange
                    border: (255, 140, 0),    // Dark orange
                    background: None,
                },
                category: PresetCategory::Semantic,
            },
            StylePreset {
                name: "danger",
                emoji: "🚨",
                description: "Error/critical state",
                border_style: BorderStyle::Double,
                colors: ColorScheme {
                    primary: (255, 69, 0),    // Red-orange
                    secondary: (255, 99, 71), // Tomato
                    border: (220, 20, 60),    // Crimson
                    background: None,
                },
                category: PresetCategory::Semantic,
            },
            // Design presets
            StylePreset {
                name: "corporate",
                emoji: "🏢",
                description: "Professional blues & grays with double borders",
                border_style: BorderStyle::Double,
                colors: ColorScheme {
                    primary: (65, 105, 225),    // Royal blue
                    secondary: (119, 136, 153), // Light slate gray
                    border: (70, 130, 180),     // Steel blue
                    background: None,
                },
                category: PresetCategory::Design,
            },
            StylePreset {
                name: "playful",
                emoji: "🎨",
                description: "Rainbow gradients with rounded borders",
                border_style: BorderStyle::Rounded,
                colors: ColorScheme {
                    primary: (255, 105, 180),   // Hot pink
                    secondary: (147, 112, 219), // Medium purple
                    border: (255, 182, 193),    // Light pink
                    background: None,
                },
                category: PresetCategory::Design,
            },
            StylePreset {
                name: "minimal",
                emoji: "◻",
                description: "Monochrome with thin single borders",
                border_style: BorderStyle::Single,
                colors: ColorScheme {
                    primary: (192, 192, 192),   // Silver
                    secondary: (169, 169, 169), // Dark gray
                    border: (128, 128, 128),    // Gray
                    background: None,
                },
                category: PresetCategory::Design,
            },
            StylePreset {
                name: "retro",
                emoji: "💾",
                description: "Matrix-style green on black with ASCII borders",
                border_style: BorderStyle::Ascii,
                colors: ColorScheme {
                    primary: (0, 255, 0),        // Lime (Matrix green)
                    secondary: (50, 205, 50),    // Lime green
                    border: (0, 128, 0),         // Green
                    background: Some((0, 0, 0)), // Black background
                },
                category: PresetCategory::Design,
            },
            // Other presets
            StylePreset {
                name: "gradient",
                emoji: "✨",
                description: "Gradient/highlight style",
                border_style: BorderStyle::Rounded,
                colors: ColorScheme {
                    primary: (138, 43, 226),   // Blue violet
                    secondary: (255, 20, 147), // Deep pink
                    border: (186, 85, 211),    // Medium orchid
                    background: None,
                },
                category: PresetCategory::Other,
            },
            StylePreset {
                name: "neutral",
                emoji: "◆",
                description: "Neutral/default style",
                border_style: BorderStyle::Single,
                colors: ColorScheme {
                    primary: (211, 211, 211),   // Light gray
                    secondary: (169, 169, 169), // Dark gray
                    border: (128, 128, 128),    // Gray
                    background: None,
                },
                category: PresetCategory::Other,
            },
        ]
    }

    /// Get semantic presets only
    pub fn semantic() -> Vec<StylePreset> {
        Self::all()
            .into_iter()
            .filter(|p| p.category == PresetCategory::Semantic)
            .collect()
    }

    /// Get design presets only
    pub fn design() -> Vec<StylePreset> {
        Self::all()
            .into_iter()
            .filter(|p| p.category == PresetCategory::Design)
            .collect()
    }

    /// Find a preset by name
    pub fn find(name: &str) -> Option<StylePreset> {
        Self::all()
            .into_iter()
            .find(|p| p.name == name.to_lowercase())
    }

    /// Get owo-colors Style for primary color
    #[allow(dead_code)]
    pub fn primary_style(&self) -> Style {
        let (r, g, b) = self.colors.primary;
        Style::new().truecolor(r, g, b)
    }

    /// Get owo-colors Style for secondary color
    #[allow(dead_code)]
    pub fn secondary_style(&self) -> Style {
        let (r, g, b) = self.colors.secondary;
        Style::new().truecolor(r, g, b)
    }

    /// Get owo-colors Style for border color
    #[allow(dead_code)]
    pub fn border_style_color(&self) -> Style {
        let (r, g, b) = self.colors.border;
        Style::new().truecolor(r, g, b)
    }
}

/// Render a sample box with the given style (uses preset's border style)
fn render_sample_box(preset: &StylePreset) -> String {
    let (tl, h, tr, v, bl, br) = preset.border_style.chars();
    let (r, g, b) = preset.colors.border;

    let mut result = String::new();
    let width = 19;
    let horizontal = h.repeat(width);

    // Top border
    result.push_str(
        &format!("{}{}{}\n", tl, horizontal, tr)
            .truecolor(r, g, b)
            .to_string(),
    );

    // Content with primary color
    let (pr, pg, pb) = preset.colors.primary;
    let title = format!("{} {} Sample Box", preset.emoji, preset.name.to_uppercase());
    let padding = width.saturating_sub(title.chars().count());
    result.push_str(&format!(
        "{} {}{} {}\n",
        v.truecolor(r, g, b),
        title.truecolor(pr, pg, pb),
        " ".repeat(padding),
        v.truecolor(r, g, b)
    ));

    let content = "Beautiful styling";
    let padding2 = width.saturating_sub(content.len());
    result.push_str(&format!(
        "{} {}{} {}\n",
        v.truecolor(r, g, b),
        content.truecolor(pr, pg, pb),
        " ".repeat(padding2),
        v.truecolor(r, g, b)
    ));

    // Bottom border
    result.push_str(
        &format!("{}{}{}\n", bl, horizontal, br)
            .truecolor(r, g, b)
            .to_string(),
    );

    result
}

/// Render a sample sparkline with the style
fn render_sample_sparkline(preset: &StylePreset) -> String {
    let (r, g, b) = preset.colors.primary;
    "▁ ▂ ▃ ▄ ▅ ▆ ▇ █".truecolor(r, g, b).to_string()
}

/// Render a sample bar chart with the style
fn render_sample_chart(preset: &StylePreset) -> String {
    let (r, g, b) = preset.colors.primary;
    let (sr, sg, sb) = preset.colors.secondary;
    format!(
        "{} {}",
        "████████████".truecolor(r, g, b),
        "75%".truecolor(sr, sg, sb)
    )
}

/// Render a sample table row with the style
fn render_sample_table(preset: &StylePreset) -> String {
    let (r, g, b) = preset.colors.border;
    let (pr, pg, pb) = preset.colors.primary;
    format!(
        "{} {} {}",
        "Name".truecolor(pr, pg, pb),
        "│".truecolor(r, g, b),
        "Data".truecolor(pr, pg, pb)
    )
}

/// Render a single preset section for the preview
fn render_preset_section(stdout: &mut std::io::Stdout, preset: &StylePreset) {
    let (pr, pg, pb) = preset.colors.primary;

    let _ = writeln!(
        stdout,
        "├─ {} {} ({})",
        preset.emoji.truecolor(pr, pg, pb),
        preset.name.to_uppercase().truecolor(pr, pg, pb).bold(),
        preset.description.dimmed()
    );
    let _ = writeln!(stdout);

    // Sample box
    let _ = write!(stdout, "  {}", render_sample_box(preset));

    // Sample components
    let (br, bg, bb) = preset.colors.border;
    let _ = writeln!(stdout, "  {} Components", "┌─".truecolor(br, bg, bb));
    let _ = writeln!(
        stdout,
        "  {}  • Chart:    {}",
        "│".truecolor(br, bg, bb),
        render_sample_chart(preset)
    );
    let _ = writeln!(
        stdout,
        "  {}  • Data:     {}",
        "│".truecolor(br, bg, bb),
        render_sample_table(preset)
    );
    let _ = writeln!(
        stdout,
        "  {}  • Sparkle:  {}",
        "│".truecolor(br, bg, bb),
        render_sample_sparkline(preset)
    );

    // Show border style info
    let border_name = match preset.border_style {
        BorderStyle::Single => "Single",
        BorderStyle::Double => "Double",
        BorderStyle::Rounded => "Rounded",
        BorderStyle::Thick => "Thick",
        BorderStyle::Ascii => "ASCII",
        BorderStyle::None => "None",
    };
    let _ = writeln!(
        stdout,
        "  {}  • Border:   {}",
        "│".truecolor(br, bg, bb),
        border_name.truecolor(pr, pg, pb)
    );
    let _ = writeln!(
        stdout,
        "  {}",
        "└─────────────────────────────────────────────".truecolor(br, bg, bb)
    );
    let _ = writeln!(stdout);
}

/// Preview all style presets side by side
pub fn render_all_preview() {
    let mut stdout = stdout();

    let _ = writeln!(
        stdout,
        "\n{}",
        "╔════════════════════════════════════════════════════════════════════════╗".bright_cyan()
    );
    let _ = writeln!(
        stdout,
        "{}",
        "║     TERMGFX STYLE PRESETS - Complete Style Showcase                    ║".bright_cyan()
    );
    let _ = writeln!(
        stdout,
        "{}\n",
        "╚════════════════════════════════════════════════════════════════════════╝".bright_cyan()
    );

    // Display semantic presets
    let _ = writeln!(
        stdout,
        "{}",
        "━━━ SEMANTIC PRESETS ━━━".bright_white().bold()
    );
    let _ = writeln!(
        stdout,
        "{}\n",
        "For status and notification messages".dimmed()
    );

    for preset in StylePreset::semantic() {
        render_preset_section(&mut stdout, &preset);
    }

    // Display design presets
    let _ = writeln!(
        stdout,
        "\n{}",
        "━━━ DESIGN PRESETS ━━━".bright_white().bold()
    );
    let _ = writeln!(stdout, "{}\n", "For themed visual styles".dimmed());

    for preset in StylePreset::design() {
        render_preset_section(&mut stdout, &preset);
    }

    // Display other presets
    let _ = writeln!(
        stdout,
        "\n{}",
        "━━━ OTHER PRESETS ━━━".bright_white().bold()
    );
    let _ = writeln!(stdout, "{}\n", "Additional styles".dimmed());

    for preset in StylePreset::all()
        .into_iter()
        .filter(|p| p.category == PresetCategory::Other)
    {
        render_preset_section(&mut stdout, &preset);
    }

    // Usage guide
    let _ = writeln!(
        stdout,
        "╔════════════════════════════════════════════════════════════════════════╗"
    );
    let _ = writeln!(
        stdout,
        "║  USAGE EXAMPLES                                                        ║"
    );
    let _ = writeln!(
        stdout,
        "╠════════════════════════════════════════════════════════════════════════╣"
    );

    let examples = vec![
        ("Box with style", "termgfx box \"Success!\" --style success"),
        (
            "Banner with style",
            "termgfx banner \"Welcome\" --style info",
        ),
        ("Progress bar", "termgfx progress 75 --style gradient"),
        (
            "Notification",
            "termgfx notification \"Done\" --style success",
        ),
        (
            "Table with style",
            "termgfx table --headers \"Name,Age\" --rows \"Alice,30\"",
        ),
        (
            "Gauge indicator",
            "termgfx gauge 75 --label \"CPU\" --style semicircle",
        ),
    ];

    for (desc, cmd) in examples {
        let padding = 35usize.saturating_sub(desc.len());
        let _ = writeln!(stdout, "║  {}{}", desc, " ".repeat(padding));
        let _ = writeln!(stdout, "║    $ {}", cmd);
        let _ = writeln!(stdout, "║");
    }

    let _ = writeln!(
        stdout,
        "╚════════════════════════════════════════════════════════════════════════╝\n"
    );
}

/// Preview a single style preset in detail
pub fn render_preset_preview(preset_name: &str) {
    if let Some(preset) = StylePreset::find(preset_name) {
        let mut stdout = stdout();

        let _ = writeln!(
            stdout,
            "\n═══ {} {} - {} ═══\n",
            preset.emoji,
            preset.name.to_uppercase(),
            preset.description
        );

        // Extended preview
        let _ = writeln!(stdout, "Box Styles:");
        let _ = writeln!(stdout);

        // Single border
        let _ = writeln!(stdout, "╭─────────────────────────────────────╮");
        let _ = writeln!(
            stdout,
            "│ Single Border with {} Style              │",
            preset.name
        );
        let _ = writeln!(stdout, "╰─────────────────────────────────────╯");
        let _ = writeln!(stdout);

        // Double border
        let _ = writeln!(stdout, "╔═════════════════════════════════════╗");
        let _ = writeln!(
            stdout,
            "║ Double Border with {} Style             ║",
            preset.name
        );
        let _ = writeln!(stdout, "╚═════════════════════════════════════╝");
        let _ = writeln!(stdout);

        // Heavy border
        let _ = writeln!(stdout, "┏━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┓");
        let _ = writeln!(
            stdout,
            "┃ Heavy Border with {} Style              ┃",
            preset.name
        );
        let _ = writeln!(stdout, "┗━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┛");
        let _ = writeln!(stdout);

        // Color showcase
        let _ = writeln!(stdout, "Color Showcase:");
        let _ = writeln!(stdout, "  Text Color:   Cyan colored text");
        let _ = writeln!(stdout, "  Accent Color: Green colored text");
        let _ = writeln!(stdout, "  Border Color: Yellow colored text");
        let _ = writeln!(stdout);

        // Component preview
        let _ = writeln!(stdout, "Component Examples:");
        let _ = writeln!(stdout, "  Box:       {}", render_sample_box(&preset));
        let _ = writeln!(stdout, "  Chart:     {}", render_sample_chart(&preset));
        let _ = writeln!(stdout, "  Sparkline: {}", render_sample_sparkline(&preset));
        let _ = writeln!(stdout, "  Table:     {}", render_sample_table(&preset));
        let _ = writeln!(stdout);
    } else {
        eprintln!("Error: Unknown style preset '{}'", preset_name);
        eprintln!(
            "Available presets: {}",
            StylePreset::all()
                .iter()
                .map(|p| p.name)
                .collect::<Vec<_>>()
                .join(", ")
        );
        std::process::exit(1);
    }
}

/// List all available style presets
pub fn render_style_list() {
    let mut stdout = stdout();
    let presets = StylePreset::all();

    let _ = writeln!(stdout, "\nAvailable Style Presets:\n");

    let max_name_len = presets.iter().map(|p| p.name.len()).max().unwrap_or(0);

    for preset in presets {
        let padding = max_name_len.saturating_sub(preset.name.len()) + 3;
        let _ = writeln!(
            stdout,
            "  {} {}{}  - {}",
            preset.emoji,
            preset.name,
            " ".repeat(padding),
            preset.description
        );
    }

    let _ = writeln!(
        stdout,
        "\nUse 'termgfx style preview <preset>' for detailed preview\n"
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_style_presets_exist() {
        let presets = StylePreset::all();
        assert!(!presets.is_empty());
        assert_eq!(presets.len(), 10); // 4 semantic + 4 design + 2 other
    }

    #[test]
    fn test_find_semantic_presets() {
        assert!(StylePreset::find("info").is_some());
        assert!(StylePreset::find("success").is_some());
        assert!(StylePreset::find("warning").is_some());
        assert!(StylePreset::find("danger").is_some());
    }

    #[test]
    fn test_find_design_presets() {
        assert!(StylePreset::find("corporate").is_some());
        assert!(StylePreset::find("playful").is_some());
        assert!(StylePreset::find("minimal").is_some());
        assert!(StylePreset::find("retro").is_some());
    }

    #[test]
    fn test_find_other_presets() {
        assert!(StylePreset::find("gradient").is_some());
        assert!(StylePreset::find("neutral").is_some());
    }

    #[test]
    fn test_find_style_case_insensitive() {
        assert!(StylePreset::find("INFO").is_some());
        assert!(StylePreset::find("Success").is_some());
        assert!(StylePreset::find("DANGER").is_some());
        assert!(StylePreset::find("Corporate").is_some());
        assert!(StylePreset::find("PLAYFUL").is_some());
        assert!(StylePreset::find("Retro").is_some());
    }

    #[test]
    fn test_find_unknown_style() {
        assert!(StylePreset::find("unknown").is_none());
    }

    #[test]
    fn test_all_presets_have_names() {
        let presets = StylePreset::all();
        for preset in presets {
            assert!(!preset.name.is_empty());
            assert!(!preset.emoji.is_empty());
            assert!(!preset.description.is_empty());
        }
    }

    #[test]
    fn test_semantic_presets_count() {
        assert_eq!(StylePreset::semantic().len(), 4);
    }

    #[test]
    fn test_design_presets_count() {
        assert_eq!(StylePreset::design().len(), 4);
    }

    #[test]
    fn test_border_styles() {
        // Corporate uses double borders
        let corporate = StylePreset::find("corporate").unwrap();
        assert_eq!(corporate.border_style, BorderStyle::Double);

        // Playful uses rounded borders
        let playful = StylePreset::find("playful").unwrap();
        assert_eq!(playful.border_style, BorderStyle::Rounded);

        // Minimal uses single borders
        let minimal = StylePreset::find("minimal").unwrap();
        assert_eq!(minimal.border_style, BorderStyle::Single);

        // Retro uses ASCII borders
        let retro = StylePreset::find("retro").unwrap();
        assert_eq!(retro.border_style, BorderStyle::Ascii);
    }

    #[test]
    fn test_retro_has_background() {
        let retro = StylePreset::find("retro").unwrap();
        assert!(retro.colors.background.is_some());
        assert_eq!(retro.colors.background, Some((0, 0, 0))); // Black background
    }

    #[test]
    fn test_border_chars() {
        let (tl, h, tr, v, bl, br) = BorderStyle::Double.chars();
        assert_eq!(tl, "╔");
        assert_eq!(h, "═");
        assert_eq!(tr, "╗");
        assert_eq!(v, "║");
        assert_eq!(bl, "╚");
        assert_eq!(br, "╝");

        let (tl, h, tr, v, bl, br) = BorderStyle::Ascii.chars();
        assert_eq!(tl, "+");
        assert_eq!(h, "-");
        assert_eq!(tr, "+");
        assert_eq!(v, "|");
        assert_eq!(bl, "+");
        assert_eq!(br, "+");
    }

    #[test]
    fn test_preset_categories() {
        let corporate = StylePreset::find("corporate").unwrap();
        assert_eq!(corporate.category, PresetCategory::Design);

        let info = StylePreset::find("info").unwrap();
        assert_eq!(info.category, PresetCategory::Semantic);

        let gradient = StylePreset::find("gradient").unwrap();
        assert_eq!(gradient.category, PresetCategory::Other);
    }
}
