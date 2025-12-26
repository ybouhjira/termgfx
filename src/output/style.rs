use owo_colors::{OwoColorize, Color, DynColors};
use unicode_width::UnicodeWidthStr;
use std::io::{stdout, Write};

/// Style preset with name, description, and colors
#[derive(Debug, Clone)]
pub struct StylePreset {
    pub name: &'static str,
    pub emoji: &'static str,
    pub description: &'static str,
    pub border_color: Color,
    pub text_color: Color,
    pub accent_color: Color,
    pub bg_color: Option<Color>,
}

impl StylePreset {
    /// Get all available style presets
    pub fn all() -> Vec<StylePreset> {
        vec![
            StylePreset {
                name: "info",
                emoji: "ℹ",
                description: "Information messages",
                border_color: Color::Blue,
                text_color: Color::BrightBlue,
                accent_color: Color::Cyan,
                bg_color: None,
            },
            StylePreset {
                name: "success",
                emoji: "✓",
                description: "Success/completed state",
                border_color: Color::Green,
                text_color: Color::BrightGreen,
                accent_color: Color::Cyan,
                bg_color: None,
            },
            StylePreset {
                name: "warning",
                emoji: "⚠",
                description: "Warning messages",
                border_color: Color::Yellow,
                text_color: Color::BrightYellow,
                accent_color: Color::Yellow,
                bg_color: None,
            },
            StylePreset {
                name: "danger",
                emoji: "🚨",
                description: "Error/critical state",
                border_color: Color::Red,
                text_color: Color::BrightRed,
                accent_color: Color::Red,
                bg_color: None,
            },
            StylePreset {
                name: "gradient",
                emoji: "✨",
                description: "Gradient/highlight style",
                border_color: Color::Magenta,
                text_color: Color::BrightMagenta,
                accent_color: Color::Magenta,
                bg_color: None,
            },
            StylePreset {
                name: "neutral",
                emoji: "◆",
                description: "Neutral/default style",
                border_color: Color::White,
                text_color: Color::White,
                accent_color: Color::BrightWhite,
                bg_color: None,
            },
        ]
    }

    /// Find a preset by name
    pub fn find(name: &str) -> Option<StylePreset> {
        Self::all().into_iter().find(|p| p.name == name.to_lowercase())
    }
}

/// Render a sample box with the given style
fn render_sample_box(preset: &StylePreset) -> String {
    let mut result = String::new();

    let top_border = format!("╭─────────────────╮");
    let bottom_border = format!("╰─────────────────╯");
    let vertical = "│";

    result.push_str(&format!("{}\n", top_border.color(DynColors::Color(preset.border_color))));

    // Header line
    let header = format!(" {} {} Sample Box", preset.emoji, preset.name.to_uppercase());
    let header_colored = header
        .color(DynColors::Color(preset.text_color))
        .bold();
    let padding = 17.saturating_sub(preset.emoji.len() + 1 + preset.name.len() + 11);
    result.push_str(&format!("{} {}{} {}\n",
        vertical.color(DynColors::Color(preset.border_color)),
        header_colored,
        " ".repeat(padding),
        vertical.color(DynColors::Color(preset.border_color))
    ));

    // Content line
    let content = "Beautiful styling".color(DynColors::Color(preset.accent_color));
    result.push_str(&format!("{} {} {}\n",
        vertical.color(DynColors::Color(preset.border_color)),
        content,
        " ".repeat(17)
    ));

    result.push_str(&format!("{}\n", bottom_border.color(DynColors::Color(preset.border_color))));

    result
}

/// Render a sample sparkline with the style
fn render_sample_sparkline(preset: &StylePreset) -> String {
    let data = "▁ ▂ ▃ ▄ ▅ ▆ ▇ █";
    format!("{}", data.color(DynColors::Color(preset.text_color)))
}

/// Render a sample bar chart with the style
fn render_sample_chart(preset: &StylePreset) -> String {
    let bar = "████████████".color(DynColors::Color(preset.text_color));
    format!("{} 75%", bar)
}

/// Render a sample table row with the style
fn render_sample_table(preset: &StylePreset) -> String {
    let header = "Name".color(DynColors::Color(preset.accent_color)).bold();
    let value = "Data".color(DynColors::Color(preset.text_color));
    format!("{} │ {}", header, value)
}

/// Preview all style presets side by side
pub fn render_all_preview() {
    let presets = StylePreset::all();
    let mut stdout = stdout();

    let _ = writeln!(stdout, "\n{}\n",
        "╔════════════════════════════════════════════════════════════════════════╗"
            .bright_white()
            .bold()
    );

    let _ = writeln!(stdout, "{}",
        "║     TERMGFX STYLE PRESETS - Complete Style Showcase                    ║"
            .bright_white()
            .bold()
    );

    let _ = writeln!(stdout, "{}\n",
        "╚════════════════════════════════════════════════════════════════════════╝"
            .bright_white()
            .bold()
    );

    // Display each preset
    for (idx, preset) in presets.iter().enumerate() {
        if idx > 0 {
            let _ = writeln!(stdout, "");
        }

        let _ = writeln!(stdout, "{}",
            format!("├─ {} {} ({})", preset.emoji, preset.name.to_uppercase(), preset.description)
                .color(DynColors::Color(preset.text_color))
                .bold()
        );

        let _ = writeln!(stdout, "");

        // Sample box
        let _ = write!(stdout, "  {}", render_sample_box(preset));

        // Sample components
        let _ = writeln!(stdout, "  ┌─ Components");
        let _ = writeln!(stdout, "  │  • Chart:    {}", render_sample_chart(preset));
        let _ = writeln!(stdout, "  │  • Data:     {}", render_sample_table(preset));
        let _ = writeln!(stdout, "  │  • Sparkle:  {}", render_sample_sparkline(preset));
        let _ = writeln!(stdout, "  └─────────────────────────────────────────────\n");
    }

    // Usage guide
    let _ = writeln!(stdout, "{}",
        "╔════════════════════════════════════════════════════════════════════════╗"
            .bright_white()
            .bold()
    );

    let _ = writeln!(stdout, "{}",
        "║  USAGE EXAMPLES                                                        ║"
            .bright_white()
            .bold()
    );

    let _ = writeln!(stdout, "{}",
        "╠════════════════════════════════════════════════════════════════════════╣"
            .bright_white()
            .bold()
    );

    let examples = vec![
        ("Box with style", "termgfx box \"Success!\" --style success"),
        ("Banner with style", "termgfx banner \"Welcome\" --style info"),
        ("Progress bar", "termgfx progress 75 --style gradient"),
        ("Notification", "termgfx notification \"Done\" --style success"),
        ("Table with style", "termgfx table --headers \"Name,Age\" --rows \"Alice,30\""),
        ("Gauge indicator", "termgfx gauge 75 --label \"CPU\" --style semicircle"),
    ];

    for (desc, cmd) in examples {
        let _ = writeln!(stdout, "║  {} {}",
            desc.bright_cyan(),
            " ".repeat(35.saturating_sub(desc.len()))
        );
        let _ = writeln!(stdout, "║    $ {}", cmd.bright_black());
        let _ = writeln!(stdout, "║");
    }

    let _ = writeln!(stdout, "{}",
        "╚════════════════════════════════════════════════════════════════════════╝\n"
            .bright_white()
            .bold()
    );
}

/// Preview a single style preset in detail
pub fn render_preset_preview(preset_name: &str) {
    if let Some(preset) = StylePreset::find(preset_name) {
        let mut stdout = stdout();

        let _ = writeln!(stdout, "\n{}\n",
            format!("═══ {} {} - {} ═══",
                preset.emoji,
                preset.name.to_uppercase(),
                preset.description
            )
                .color(DynColors::Color(preset.text_color))
                .bold()
        );

        // Extended preview
        let _ = writeln!(stdout, "{}", "Box Styles:");
        let _ = writeln!(stdout, "");

        // Single border
        let border = format!("╭─────────────────────────────────────╮");
        let _ = writeln!(stdout, "{}", border.color(DynColors::Color(preset.border_color)));
        let _ = writeln!(stdout, "{}", format!("│ Single Border with {} Style              │", preset.name)
            .color(DynColors::Color(preset.text_color)));
        let _ = writeln!(stdout, "{}", border.replace("╭", "╰").replace("╮", "╯").color(DynColors::Color(preset.border_color)));
        let _ = writeln!(stdout, "");

        // Double border
        let border_double = format!("╔═════════════════════════════════════╗");
        let _ = writeln!(stdout, "{}", border_double.color(DynColors::Color(preset.border_color)));
        let _ = writeln!(stdout, "{}", format!("║ Double Border with {} Style             ║", preset.name)
            .color(DynColors::Color(preset.text_color)));
        let _ = writeln!(stdout, "{}", border_double.replace("╔", "╚").replace("╗", "╝").color(DynColors::Color(preset.border_color)));
        let _ = writeln!(stdout, "");

        // Heavy border
        let border_heavy = format!("┏━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┓");
        let _ = writeln!(stdout, "{}", border_heavy.color(DynColors::Color(preset.border_color)));
        let _ = writeln!(stdout, "{}", format!("┃ Heavy Border with {} Style              ┃", preset.name)
            .color(DynColors::Color(preset.text_color)));
        let _ = writeln!(stdout, "{}", border_heavy.replace("┏", "┗").replace("┓", "┛").color(DynColors::Color(preset.border_color)));
        let _ = writeln!(stdout, "");

        // Color showcase
        let _ = writeln!(stdout, "{}", "Color Showcase:".bold());
        let _ = writeln!(stdout, "  Text Color:   {}", "This is the main text color".color(DynColors::Color(preset.text_color)));
        let _ = writeln!(stdout, "  Accent Color: {}", "This is the accent color".color(DynColors::Color(preset.accent_color)));
        let _ = writeln!(stdout, "  Border Color: {}", "This is the border color".color(DynColors::Color(preset.border_color)));
        let _ = writeln!(stdout, "");

        // Component preview
        let _ = writeln!(stdout, "{}", "Component Examples:".bold());
        let _ = writeln!(stdout, "  Box:       {}", render_sample_box(&preset));
        let _ = writeln!(stdout, "  Chart:     {}", render_sample_chart(&preset));
        let _ = writeln!(stdout, "  Sparkline: {}", render_sample_sparkline(&preset));
        let _ = writeln!(stdout, "  Table:     {}", render_sample_table(&preset));
        let _ = writeln!(stdout, "");
    } else {
        eprintln!("Error: Unknown style preset '{}'", preset_name);
        eprintln!("Available presets: {}",
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

    let _ = writeln!(stdout, "\n{}\n",
        "Available Style Presets:".bright_white().bold()
    );

    let max_name_len = presets.iter().map(|p| p.name.len()).max().unwrap_or(0);

    for preset in presets {
        let _ = writeln!(stdout, "  {} {} {}  - {}",
            preset.emoji,
            preset.name.bright_cyan(),
            " ".repeat(max_name_len.saturating_sub(preset.name.len()) + 3),
            preset.description
        );
    }

    let _ = writeln!(stdout, "\n{}",
        "Use 'termgfx style preview <preset>' for detailed preview\n"
            .bright_black()
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
