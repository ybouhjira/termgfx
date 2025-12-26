use owo_colors::OwoColorize;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ColorInfo {
    pub name: String,
    pub hex: String,
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Palette {
    pub name: String,
    pub description: String,
    pub colors: Vec<ColorInfo>,
}

impl Palette {
    fn new(name: &str, description: &str) -> Self {
        Palette {
            name: name.to_string(),
            description: description.to_string(),
            colors: Vec::new(),
        }
    }

    fn with_color(mut self, name: &str, hex: &str) -> Self {
        let hex_clean = hex.trim_start_matches('#');
        let (r, g, b) = parse_hex_color(hex_clean);
        self.colors.push(ColorInfo {
            name: name.to_string(),
            hex: hex.to_string(),
            r,
            g,
            b,
        });
        self
    }
}

fn parse_hex_color(hex: &str) -> (u8, u8, u8) {
    if hex.len() != 6 {
        return (0, 0, 0);
    }
    let r = u8::from_str_radix(&hex[0..2], 16).unwrap_or(0);
    let g = u8::from_str_radix(&hex[2..4], 16).unwrap_or(0);
    let b = u8::from_str_radix(&hex[4..6], 16).unwrap_or(0);
    (r, g, b)
}

pub fn get_all_palettes() -> Vec<Palette> {
    vec![
        Palette::new("default", "Default termgfx colors")
            .with_color("Success", "#3fb950")
            .with_color("Warning", "#d29922")
            .with_color("Danger", "#f85149")
            .with_color("Info", "#58a6ff")
            .with_color("Muted", "#6e7681"),
        Palette::new("nord", "Arctic, north-bluish color palette")
            .with_color("Frost 1", "#8fbcbb")
            .with_color("Frost 2", "#88c0d0")
            .with_color("Frost 3", "#81a1c1")
            .with_color("Frost 4", "#5e81ac")
            .with_color("Snow Storm", "#eceff4"),
        Palette::new("dracula", "Dark theme inspired by Dracula")
            .with_color("Background", "#282a36")
            .with_color("Current Line", "#44475a")
            .with_color("Foreground", "#f8f8f2")
            .with_color("Comment", "#6272a4")
            .with_color("Cyan", "#8be9fd"),
        Palette::new("solarized", "Precision colors for machines and people")
            .with_color("Base 03", "#002b36")
            .with_color("Base 02", "#073642")
            .with_color("Base 01", "#586e75")
            .with_color("Base 00", "#657b83")
            .with_color("Base 0", "#839496"),
        Palette::new("gruvbox", "Retro groove color scheme")
            .with_color("Dark Red", "#cc241d")
            .with_color("Green", "#98971a")
            .with_color("Dark Yellow", "#d79921")
            .with_color("Dark Blue", "#458588")
            .with_color("Dark Purple", "#b16286"),
        Palette::new("tokyo-night", "Modern Tokyo-inspired color palette")
            .with_color("Blue", "#7aa2f7")
            .with_color("Green", "#9ece6a")
            .with_color("Yellow", "#e0af68")
            .with_color("Red", "#f7768e")
            .with_color("Magenta", "#bb9af7"),
        Palette::new("catppuccin", "Soothing pastel color palette")
            .with_color("Rosewater", "#f5e0dc")
            .with_color("Flamingo", "#f2cdcd")
            .with_color("Pink", "#f5c2e7")
            .with_color("Mauve", "#cba6f7")
            .with_color("Lavender", "#b4a7e8"),
    ]
}

pub fn get_palette(name: &str) -> Option<Palette> {
    get_all_palettes()
        .into_iter()
        .find(|p| p.name.to_lowercase() == name.to_lowercase())
}

pub fn list_palettes() {
    let palettes = get_all_palettes();
    println!();
    println!("  {} Available Color Palettes", "🎨".bright_cyan());
    println!();
    for (idx, palette) in palettes.iter().enumerate() {
        let number_text = format!("{}.", idx + 1);
        println!("  {}  {}  {}",
            number_text.bright_black(),
            palette.name.bright_yellow().bold(),
            palette.description.bright_black());
        for color in &palette.colors {
            println!("     ● {}  {}", color.name.bright_white(), color.hex.bright_black());
        }
        println!();
    }
}

pub fn show_palette(palette: &Palette) {
    println!();
    println!("  {} {}", "🎨".bright_cyan(), palette.name.bright_yellow().bold());
    println!("  {}", palette.description.bright_black());
    println!();
    for (idx, color) in palette.colors.iter().enumerate() {
        let num_text = format!("{}.", idx + 1);
        let swatch = match idx % 3 {
            0 => "███".bright_red().to_string(),
            1 => "███".bright_green().to_string(),
            _ => "███".bright_blue().to_string(),
        };
        println!(
            "  {}  {}  {} {}",
            num_text.bright_black(),
            swatch,
            color.name.bright_white(),
            color.hex.bright_black()
        );
    }
    println!();
}

pub fn export_palette(palette: &Palette) -> String {
    serde_json::to_string_pretty(palette).unwrap_or_default()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_hex_color() {
        let (r, g, b) = parse_hex_color("3fb950");
        assert_eq!(r, 0x3f);
        assert_eq!(g, 0xb9);
        assert_eq!(b, 0x50);
    }

    #[test]
    fn test_get_palette() {
        let palette = get_palette("default").unwrap();
        assert_eq!(palette.name, "default");
        assert!(!palette.colors.is_empty());
    }

    #[test]
    fn test_palette_not_found() {
        assert!(get_palette("nonexistent").is_none());
    }

    #[test]
    fn test_all_palettes_have_colors() {
        for palette in get_all_palettes() {
            assert!(!palette.colors.is_empty(), "Palette {} has no colors", palette.name);
        }
    }

    #[test]
    fn test_case_insensitive_lookup() {
        assert!(get_palette("DEFAULT").is_some());
        assert!(get_palette("default").is_some());
    }

    #[test]
    fn test_export_palette() {
        let palette = get_palette("default").unwrap();
        let json = export_palette(&palette);
        assert!(json.contains("default"));
        assert!(json.contains("colors"));
    }
}
