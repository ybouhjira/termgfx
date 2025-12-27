use std::io::{stdout, Write};

/// Style preset with color names
#[derive(Debug, Clone)]
pub struct StylePreset {
    pub name: &'static str,
    pub emoji: &'static str,
    pub description: &'static str,
}

impl StylePreset {
    /// Get all available style presets
    pub fn all() -> Vec<StylePreset> {
        vec![
            StylePreset {
                name: "info",
                emoji: "ℹ",
                description: "Information messages",
            },
            StylePreset {
                name: "success",
                emoji: "✓",
                description: "Success/completed state",
            },
            StylePreset {
                name: "warning",
                emoji: "⚠",
                description: "Warning messages",
            },
            StylePreset {
                name: "danger",
                emoji: "🚨",
                description: "Error/critical state",
            },
            StylePreset {
                name: "gradient",
                emoji: "✨",
                description: "Gradient/highlight style",
            },
            StylePreset {
                name: "neutral",
                emoji: "◆",
                description: "Neutral/default style",
            },
        ]
    }

    /// Find a preset by name
    pub fn find(name: &str) -> Option<StylePreset> {
        Self::all()
            .into_iter()
            .find(|p| p.name == name.to_lowercase())
    }
}

/// Render a sample box with the given style
fn render_sample_box(preset: &StylePreset) -> String {
    let mut result = String::new();
    result.push_str("╭─────────────────╮\n");
    result.push_str(&format!(
        "│ {} {} Sample Box │\n",
        preset.emoji,
        preset.name.to_uppercase()
    ));
    result.push_str("│ Beautiful styling │\n");
    result.push_str("╰─────────────────╯\n");
    result
}

/// Render a sample sparkline with the style
fn render_sample_sparkline(_preset: &StylePreset) -> String {
    "▁ ▂ ▃ ▄ ▅ ▆ ▇ █".to_string()
}

/// Render a sample bar chart with the style
fn render_sample_chart(_preset: &StylePreset) -> String {
    "████████████ 75%".to_string()
}

/// Render a sample table row with the style
fn render_sample_table(_preset: &StylePreset) -> String {
    "Name │ Data".to_string()
}

/// Preview all style presets side by side
pub fn render_all_preview() {
    let presets = StylePreset::all();
    let mut stdout = stdout();

    let _ = writeln!(
        stdout,
        "\n╔════════════════════════════════════════════════════════════════════════╗"
    );
    let _ = writeln!(
        stdout,
        "║     TERMGFX STYLE PRESETS - Complete Style Showcase                    ║"
    );
    let _ = writeln!(
        stdout,
        "╚════════════════════════════════════════════════════════════════════════╝\n"
    );

    // Display each preset
    for (idx, preset) in presets.iter().enumerate() {
        if idx > 0 {
            let _ = writeln!(stdout, "");
        }

        let _ = writeln!(
            stdout,
            "├─ {} {} ({})",
            preset.emoji,
            preset.name.to_uppercase(),
            preset.description
        );
        let _ = writeln!(stdout, "");

        // Sample box
        let _ = write!(stdout, "  {}", render_sample_box(preset));

        // Sample components
        let _ = writeln!(stdout, "  ┌─ Components");
        let _ = writeln!(stdout, "  │  • Chart:    {}", render_sample_chart(preset));
        let _ = writeln!(stdout, "  │  • Data:     {}", render_sample_table(preset));
        let _ = writeln!(
            stdout,
            "  │  • Sparkle:  {}",
            render_sample_sparkline(preset)
        );
        let _ = writeln!(stdout, "  └─────────────────────────────────────────────\n");
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
        let _ = writeln!(stdout, "");

        // Single border
        let _ = writeln!(stdout, "╭─────────────────────────────────────╮");
        let _ = writeln!(
            stdout,
            "│ Single Border with {} Style              │",
            preset.name
        );
        let _ = writeln!(stdout, "╰─────────────────────────────────────╯");
        let _ = writeln!(stdout, "");

        // Double border
        let _ = writeln!(stdout, "╔═════════════════════════════════════╗");
        let _ = writeln!(
            stdout,
            "║ Double Border with {} Style             ║",
            preset.name
        );
        let _ = writeln!(stdout, "╚═════════════════════════════════════╝");
        let _ = writeln!(stdout, "");

        // Heavy border
        let _ = writeln!(stdout, "┏━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┓");
        let _ = writeln!(
            stdout,
            "┃ Heavy Border with {} Style              ┃",
            preset.name
        );
        let _ = writeln!(stdout, "┗━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┛");
        let _ = writeln!(stdout, "");

        // Color showcase
        let _ = writeln!(stdout, "Color Showcase:");
        let _ = writeln!(stdout, "  Text Color:   Cyan colored text");
        let _ = writeln!(stdout, "  Accent Color: Green colored text");
        let _ = writeln!(stdout, "  Border Color: Yellow colored text");
        let _ = writeln!(stdout, "");

        // Component preview
        let _ = writeln!(stdout, "Component Examples:");
        let _ = writeln!(stdout, "  Box:       {}", render_sample_box(&preset));
        let _ = writeln!(stdout, "  Chart:     {}", render_sample_chart(&preset));
        let _ = writeln!(stdout, "  Sparkline: {}", render_sample_sparkline(&preset));
        let _ = writeln!(stdout, "  Table:     {}", render_sample_table(&preset));
        let _ = writeln!(stdout, "");
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
        assert_eq!(presets.len(), 6);
    }

    #[test]
    fn test_find_style_preset() {
        assert!(StylePreset::find("info").is_some());
        assert!(StylePreset::find("success").is_some());
        assert!(StylePreset::find("warning").is_some());
        assert!(StylePreset::find("danger").is_some());
        assert!(StylePreset::find("gradient").is_some());
        assert!(StylePreset::find("neutral").is_some());
    }

    #[test]
    fn test_find_style_case_insensitive() {
        assert!(StylePreset::find("INFO").is_some());
        assert!(StylePreset::find("Success").is_some());
        assert!(StylePreset::find("DANGER").is_some());
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
}
