//! Unified RGB color palette system for termgfx
//! Provides structured color palettes with semantic meanings and utility functions.

use std::fmt;

/// RGB color representation with utility functions
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RgbColor {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl RgbColor {
    pub const fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }

    pub fn from_hex(hex: &str) -> Result<Self, String> {
        let hex = hex.trim_start_matches('#');
        if hex.len() != 6 {
            return Err(format!("Invalid hex color: {}", hex));
        }
        let r = u8::from_str_radix(&hex[0..2], 16).map_err(|_| format!("Invalid hex"))?;
        let g = u8::from_str_radix(&hex[2..4], 16).map_err(|_| format!("Invalid hex"))?;
        let b = u8::from_str_radix(&hex[4..6], 16).map_err(|_| format!("Invalid hex"))?;
        Ok(Self { r, g, b })
    }

    pub fn to_hex(&self) -> String {
        format!("#{:02X}{:02X}{:02X}", self.r, self.g, self.b)
    }

    pub fn lighten(&self, percent: u8) -> RgbColor {
        let p = percent.min(100) as f32 / 100.0;
        RgbColor {
            r: ((self.r as f32) + (((255.0 - self.r as f32)) * p)) as u8,
            g: ((self.g as f32) + (((255.0 - self.g as f32)) * p)) as u8,
            b: ((self.b as f32) + (((255.0 - self.b as f32)) * p)) as u8,
        }
    }

    pub fn darken(&self, percent: u8) -> RgbColor {
        let p = percent.min(100) as f32 / 100.0;
        RgbColor {
            r: ((self.r as f32) * (1.0 - p)) as u8,
            g: ((self.g as f32) * (1.0 - p)) as u8,
            b: ((self.b as f32) * (1.0 - p)) as u8,
        }
    }

    pub fn mix(&self, other: &RgbColor, ratio: f32) -> RgbColor {
        let r = ratio.max(0.0).min(1.0);
        RgbColor {
            r: ((self.r as f32) * (1.0 - r) + (other.r as f32) * r) as u8,
            g: ((self.g as f32) * (1.0 - r) + (other.g as f32) * r) as u8,
            b: ((self.b as f32) * (1.0 - r) + (other.b as f32) * r) as u8,
        }
    }

    pub fn luminance(&self) -> f32 {
        let r = (self.r as f32) / 255.0;
        let g = (self.g as f32) / 255.0;
        let b = (self.b as f32) / 255.0;
        let r = if r <= 0.03928 { r / 12.92 } else { ((r + 0.055) / 1.055).powf(2.4) };
        let g = if g <= 0.03928 { g / 12.92 } else { ((g + 0.055) / 1.055).powf(2.4) };
        let b = if b <= 0.03928 { b / 12.92 } else { ((b + 0.055) / 1.055).powf(2.4) };
        0.2126 * r + 0.7152 * g + 0.0722 * b
    }

    pub fn contrast_ratio(&self, other: &RgbColor) -> f32 {
        let l1 = self.luminance();
        let l2 = other.luminance();
        let lighter = l1.max(l2);
        let darker = l1.min(l2);
        (lighter + 0.05) / (darker + 0.05)
    }

    #[cfg(feature = "cli")]
    pub fn to_crossterm(&self) -> crossterm::style::Color {
        crossterm::style::Color::Rgb { r: self.r, g: self.g, b: self.b }
    }

    #[cfg(feature = "cli")]
    pub fn to_ratatui(&self) -> ratatui::style::Color {
        ratatui::style::Color::Rgb(self.r, self.g, self.b)
    }
}

impl fmt::Display for RgbColor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_hex())
    }
}

impl From<(u8, u8, u8)> for RgbColor {
    fn from((r, g, b): (u8, u8, u8)) -> Self {
        Self { r, g, b }
    }
}

/// Color shade palette with 10 shades (50-900)
#[derive(Debug, Clone)]
pub struct ColorShade {
    pub shade_50: RgbColor,
    pub shade_100: RgbColor,
    pub shade_200: RgbColor,
    pub shade_300: RgbColor,
    pub shade_400: RgbColor,
    pub shade_500: RgbColor,
    pub shade_600: RgbColor,
    pub shade_700: RgbColor,
    pub shade_800: RgbColor,
    pub shade_900: RgbColor,
}

impl ColorShade {
    pub fn from_base(base: RgbColor) -> Self {
        Self {
            shade_50: base.lighten(95),
            shade_100: base.lighten(90),
            shade_200: base.lighten(80),
            shade_300: base.lighten(70),
            shade_400: base.lighten(50),
            shade_500: base,
            shade_600: base.darken(15),
            shade_700: base.darken(30),
            shade_800: base.darken(50),
            shade_900: base.darken(70),
        }
    }

    pub fn custom(shades: [RgbColor; 10]) -> Self {
        Self {
            shade_50: shades[0], shade_100: shades[1], shade_200: shades[2], shade_300: shades[3],
            shade_400: shades[4], shade_500: shades[5], shade_600: shades[6], shade_700: shades[7],
            shade_800: shades[8], shade_900: shades[9],
        }
    }

    pub fn get(&self, shade: u16) -> Option<RgbColor> {
        match shade {
            50 => Some(self.shade_50), 100 => Some(self.shade_100), 200 => Some(self.shade_200),
            300 => Some(self.shade_300), 400 => Some(self.shade_400), 500 => Some(self.shade_500),
            600 => Some(self.shade_600), 700 => Some(self.shade_700), 800 => Some(self.shade_800),
            900 => Some(self.shade_900), _ => None,
        }
    }

    pub fn all(&self) -> [RgbColor; 10] {
        [self.shade_50, self.shade_100, self.shade_200, self.shade_300, self.shade_400,
         self.shade_500, self.shade_600, self.shade_700, self.shade_800, self.shade_900]
    }
}

/// Semantic color palette variant
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PaletteVariant {
    Default,
    Nord,
    Dracula,
    Monokai,
    Solarized,
}

impl fmt::Display for PaletteVariant {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Default => write!(f, "default"),
            Self::Nord => write!(f, "nord"),
            Self::Dracula => write!(f, "dracula"),
            Self::Monokai => write!(f, "monokai"),
            Self::Solarized => write!(f, "solarized"),
        }
    }
}

/// Complete color palette with 8 semantic colors and 10 shades each
#[derive(Debug, Clone)]
pub struct ColorPalette {
    pub primary: ColorShade,
    pub secondary: ColorShade,
    pub accent: ColorShade,
    pub success: ColorShade,
    pub warning: ColorShade,
    pub danger: ColorShade,
    pub info: ColorShade,
    pub neutral: ColorShade,
    pub variant: PaletteVariant,
}

impl ColorPalette {
    pub fn new(variant: PaletteVariant) -> Self {
        match variant {
            PaletteVariant::Default => Self::default_palette(),
            PaletteVariant::Nord => Self::nord_palette(),
            PaletteVariant::Dracula => Self::dracula_palette(),
            PaletteVariant::Monokai => Self::monokai_palette(),
            PaletteVariant::Solarized => Self::solarized_palette(),
        }
    }

    fn default_palette() -> Self { Self { primary: ColorShade::custom([RgbColor::new(240,248,255),RgbColor::new(219,234,254),RgbColor::new(191,219,254),RgbColor::new(147,197,253),RgbColor::new(96,165,250),RgbColor::new(59,130,246),RgbColor::new(37,99,235),RgbColor::new(29,78,216),RgbColor::new(30,58,138),RgbColor::new(15,23,42)]), secondary: ColorShade::custom([RgbColor::new(244,237,255),RgbColor::new(233,213,255),RgbColor::new(216,180,254),RgbColor::new(196,181,253),RgbColor::new(168,85,247),RgbColor::new(147,51,234),RgbColor::new(126,34,206),RgbColor::new(108,15,168),RgbColor::new(59,7,100),RgbColor::new(44,1,58)]), accent: ColorShade::custom([RgbColor::new(254,243,199),RgbColor::new(254,229,117),RgbColor::new(253,230,138),RgbColor::new(252,211,77),RgbColor::new(251,146,60),RgbColor::new(249,115,22),RgbColor::new(234,88,12),RgbColor::new(194,65,12),RgbColor::new(124,45,18),RgbColor::new(67,20,7)]), success: ColorShade::custom([RgbColor::new(240,253,244),RgbColor::new(220,252,231),RgbColor::new(187,247,208),RgbColor::new(134,239,172),RgbColor::new(74,222,128),RgbColor::new(34,197,94),RgbColor::new(22,163,74),RgbColor::new(16,185,129),RgbColor::new(5,150,105),RgbColor::new(4,120,87)]), warning: ColorShade::custom([RgbColor::new(254,252,232),RgbColor::new(254,248,199),RgbColor::new(253,224,71),RgbColor::new(250,204,21),RgbColor::new(234,179,8),RgbColor::new(202,138,4),RgbColor::new(180,83,9),RgbColor::new(161,98,7),RgbColor::new(120,53,15),RgbColor::new(78,22,6)]), danger: ColorShade::custom([RgbColor::new(254,242,242),RgbColor::new(254,226,226),RgbColor::new(254,202,202),RgbColor::new(252,165,165),RgbColor::new(248,113,113),RgbColor::new(239,68,68),RgbColor::new(220,38,38),RgbColor::new(185,28,28),RgbColor::new(127,29,29),RgbColor::new(127,29,29)]), info: ColorShade::custom([RgbColor::new(240,249,255),RgbColor::new(224,242,254),RgbColor::new(206,235,253),RgbColor::new(164,224,251),RgbColor::new(125,211,252),RgbColor::new(6,182,212),RgbColor::new(8,145,178),RgbColor::new(13,110,253),RgbColor::new(3,102,214),RgbColor::new(8,71,137)]), neutral: ColorShade::custom([RgbColor::new(250,250,250),RgbColor::new(245,245,245),RgbColor::new(229,229,229),RgbColor::new(212,212,212),RgbColor::new(163,230,53),RgbColor::new(120,113,108),RgbColor::new(87,83,82),RgbColor::new(64,63,63),RgbColor::new(38,38,38),RgbColor::new(23,23,23)]), variant: PaletteVariant::Default } }

    fn nord_palette() -> Self { Self { primary: ColorShade::custom([RgbColor::new(236,239,244),RgbColor::new(229,233,240),RgbColor::new(216,222,233),RgbColor::new(216,222,233),RgbColor::new(216,222,233),RgbColor::new(216,222,233),RgbColor::new(46,52,64),RgbColor::new(59,66,82),RgbColor::new(76,86,106),RgbColor::new(46,52,64)]), secondary: ColorShade::custom([RgbColor::new(191,197,206),RgbColor::new(216,222,233),RgbColor::new(236,239,244),RgbColor::new(143,188,187),RgbColor::new(136,192,208),RgbColor::new(129,161,193),RgbColor::new(191,144,0),RgbColor::new(135,106,146),RgbColor::new(76,86,106),RgbColor::new(46,52,64)]), accent: ColorShade::custom([RgbColor::new(191,144,0),RgbColor::new(208,135,112),RgbColor::new(191,144,0),RgbColor::new(208,135,112),RgbColor::new(191,144,0),RgbColor::new(191,144,0),RgbColor::new(176,98,55),RgbColor::new(129,74,52),RgbColor::new(76,86,106),RgbColor::new(46,52,64)]), success: ColorShade::custom([RgbColor::new(163,190,140),RgbColor::new(163,190,140),RgbColor::new(163,190,140),RgbColor::new(163,190,140),RgbColor::new(163,190,140),RgbColor::new(163,190,140),RgbColor::new(120,140,100),RgbColor::new(90,105,75),RgbColor::new(76,86,106),RgbColor::new(46,52,64)]), warning: ColorShade::custom([RgbColor::new(235,203,139),RgbColor::new(235,203,139),RgbColor::new(235,203,139),RgbColor::new(235,203,139),RgbColor::new(235,203,139),RgbColor::new(235,203,139),RgbColor::new(206,165,100),RgbColor::new(176,135,70),RgbColor::new(76,86,106),RgbColor::new(46,52,64)]), danger: ColorShade::custom([RgbColor::new(191,97,106),RgbColor::new(191,97,106),RgbColor::new(191,97,106),RgbColor::new(191,97,106),RgbColor::new(191,97,106),RgbColor::new(191,97,106),RgbColor::new(168,70,80),RgbColor::new(140,50,60),RgbColor::new(76,86,106),RgbColor::new(46,52,64)]), info: ColorShade::custom([RgbColor::new(136,192,208),RgbColor::new(136,192,208),RgbColor::new(136,192,208),RgbColor::new(136,192,208),RgbColor::new(129,161,193),RgbColor::new(129,161,193),RgbColor::new(100,135,155),RgbColor::new(75,105,130),RgbColor::new(76,86,106),RgbColor::new(46,52,64)]), neutral: ColorShade::custom([RgbColor::new(236,239,244),RgbColor::new(229,233,240),RgbColor::new(216,222,233),RgbColor::new(191,197,206),RgbColor::new(143,188,187),RgbColor::new(76,86,106),RgbColor::new(59,66,82),RgbColor::new(46,52,64),RgbColor::new(46,52,64),RgbColor::new(46,52,64)]), variant: PaletteVariant::Nord } }

    fn dracula_palette() -> Self { Self { primary: ColorShade::custom([RgbColor::new(98,114,164),RgbColor::new(98,114,164),RgbColor::new(98,114,164),RgbColor::new(98,114,164),RgbColor::new(98,114,164),RgbColor::new(98,114,164),RgbColor::new(68,84,134),RgbColor::new(48,64,104),RgbColor::new(28,44,74),RgbColor::new(8,24,44)]), secondary: ColorShade::custom([RgbColor::new(189,147,249),RgbColor::new(189,147,249),RgbColor::new(189,147,249),RgbColor::new(189,147,249),RgbColor::new(189,147,249),RgbColor::new(189,147,249),RgbColor::new(159,117,219),RgbColor::new(129,87,189),RgbColor::new(99,57,159),RgbColor::new(69,27,129)]), accent: ColorShade::custom([RgbColor::new(255,121,198),RgbColor::new(255,121,198),RgbColor::new(255,121,198),RgbColor::new(255,121,198),RgbColor::new(255,121,198),RgbColor::new(255,121,198),RgbColor::new(225,91,168),RgbColor::new(195,61,138),RgbColor::new(165,31,108),RgbColor::new(135,1,78)]), success: ColorShade::custom([RgbColor::new(80,250,123),RgbColor::new(80,250,123),RgbColor::new(80,250,123),RgbColor::new(80,250,123),RgbColor::new(80,250,123),RgbColor::new(80,250,123),RgbColor::new(50,220,93),RgbColor::new(20,190,63),RgbColor::new(10,160,33),RgbColor::new(0,130,3)]), warning: ColorShade::custom([RgbColor::new(241,250,140),RgbColor::new(241,250,140),RgbColor::new(241,250,140),RgbColor::new(241,250,140),RgbColor::new(241,250,140),RgbColor::new(241,250,140),RgbColor::new(211,220,110),RgbColor::new(181,190,80),RgbColor::new(151,160,50),RgbColor::new(121,130,20)]), danger: ColorShade::custom([RgbColor::new(255,85,85),RgbColor::new(255,85,85),RgbColor::new(255,85,85),RgbColor::new(255,85,85),RgbColor::new(255,85,85),RgbColor::new(255,85,85),RgbColor::new(225,55,55),RgbColor::new(195,25,25),RgbColor::new(165,0,0),RgbColor::new(135,0,0)]), info: ColorShade::custom([RgbColor::new(139,233,253),RgbColor::new(139,233,253),RgbColor::new(139,233,253),RgbColor::new(139,233,253),RgbColor::new(139,233,253),RgbColor::new(139,233,253),RgbColor::new(109,203,223),RgbColor::new(79,173,193),RgbColor::new(49,143,163),RgbColor::new(19,113,133)]), neutral: ColorShade::custom([RgbColor::new(248,248,242),RgbColor::new(98,114,164),RgbColor::new(98,114,164),RgbColor::new(98,114,164),RgbColor::new(68,71,90),RgbColor::new(68,71,90),RgbColor::new(44,47,66),RgbColor::new(33,36,55),RgbColor::new(22,25,44),RgbColor::new(40,42,54)]), variant: PaletteVariant::Dracula } }

    fn monokai_palette() -> Self { Self { primary: ColorShade::custom([RgbColor::new(117,113,94),RgbColor::new(117,113,94),RgbColor::new(117,113,94),RgbColor::new(117,113,94),RgbColor::new(117,113,94),RgbColor::new(117,113,94),RgbColor::new(87,83,64),RgbColor::new(57,53,34),RgbColor::new(39,40,34),RgbColor::new(39,40,34)]), secondary: ColorShade::custom([RgbColor::new(117,113,94),RgbColor::new(117,113,94),RgbColor::new(117,113,94),RgbColor::new(117,113,94),RgbColor::new(117,113,94),RgbColor::new(117,113,94),RgbColor::new(87,83,64),RgbColor::new(57,53,34),RgbColor::new(39,40,34),RgbColor::new(39,40,34)]), accent: ColorShade::custom([RgbColor::new(249,38,114),RgbColor::new(249,38,114),RgbColor::new(249,38,114),RgbColor::new(249,38,114),RgbColor::new(249,38,114),RgbColor::new(249,38,114),RgbColor::new(219,8,84),RgbColor::new(189,0,54),RgbColor::new(159,0,24),RgbColor::new(39,40,34)]), success: ColorShade::custom([RgbColor::new(166,226,46),RgbColor::new(166,226,46),RgbColor::new(166,226,46),RgbColor::new(166,226,46),RgbColor::new(166,226,46),RgbColor::new(166,226,46),RgbColor::new(136,196,16),RgbColor::new(106,166,0),RgbColor::new(76,136,0),RgbColor::new(39,40,34)]), warning: ColorShade::custom([RgbColor::new(253,151,31),RgbColor::new(253,151,31),RgbColor::new(253,151,31),RgbColor::new(253,151,31),RgbColor::new(253,151,31),RgbColor::new(253,151,31),RgbColor::new(223,121,1),RgbColor::new(193,91,0),RgbColor::new(163,61,0),RgbColor::new(39,40,34)]), danger: ColorShade::custom([RgbColor::new(248,48,35),RgbColor::new(248,48,35),RgbColor::new(248,48,35),RgbColor::new(248,48,35),RgbColor::new(248,48,35),RgbColor::new(248,48,35),RgbColor::new(218,18,5),RgbColor::new(188,0,0),RgbColor::new(158,0,0),RgbColor::new(39,40,34)]), info: ColorShade::custom([RgbColor::new(102,217,239),RgbColor::new(102,217,239),RgbColor::new(102,217,239),RgbColor::new(102,217,239),RgbColor::new(102,217,239),RgbColor::new(102,217,239),RgbColor::new(72,187,209),RgbColor::new(42,157,179),RgbColor::new(12,127,149),RgbColor::new(39,40,34)]), neutral: ColorShade::custom([RgbColor::new(248,248,242),RgbColor::new(117,113,94),RgbColor::new(117,113,94),RgbColor::new(117,113,94),RgbColor::new(75,75,75),RgbColor::new(75,75,75),RgbColor::new(57,57,57),RgbColor::new(39,40,34),RgbColor::new(39,40,34),RgbColor::new(39,40,34)]), variant: PaletteVariant::Monokai } }

    fn solarized_palette() -> Self { Self { primary: ColorShade::custom([RgbColor::new(238,232,213),RgbColor::new(253,246,227),RgbColor::new(239,231,212),RgbColor::new(181,137,0),RgbColor::new(42,161,152),RgbColor::new(38,139,210),RgbColor::new(7,54,66),RgbColor::new(0,43,54),RgbColor::new(0,43,54),RgbColor::new(0,43,54)]), secondary: ColorShade::custom([RgbColor::new(238,232,213),RgbColor::new(253,246,227),RgbColor::new(147,161,161),RgbColor::new(101,123,142),RgbColor::new(131,148,150),RgbColor::new(108,113,196),RgbColor::new(7,54,66),RgbColor::new(0,43,54),RgbColor::new(0,43,54),RgbColor::new(0,43,54)]), accent: ColorShade::custom([RgbColor::new(238,232,213),RgbColor::new(253,246,227),RgbColor::new(220,50,47),RgbColor::new(220,50,47),RgbColor::new(220,50,47),RgbColor::new(220,50,47),RgbColor::new(190,20,17),RgbColor::new(160,0,0),RgbColor::new(130,0,0),RgbColor::new(0,43,54)]), success: ColorShade::custom([RgbColor::new(238,232,213),RgbColor::new(253,246,227),RgbColor::new(133,153,0),RgbColor::new(133,153,0),RgbColor::new(133,153,0),RgbColor::new(133,153,0),RgbColor::new(103,123,0),RgbColor::new(73,93,0),RgbColor::new(43,63,0),RgbColor::new(0,43,54)]), warning: ColorShade::custom([RgbColor::new(238,232,213),RgbColor::new(253,246,227),RgbColor::new(181,137,0),RgbColor::new(181,137,0),RgbColor::new(181,137,0),RgbColor::new(181,137,0),RgbColor::new(151,107,0),RgbColor::new(121,77,0),RgbColor::new(91,47,0),RgbColor::new(0,43,54)]), danger: ColorShade::custom([RgbColor::new(238,232,213),RgbColor::new(253,246,227),RgbColor::new(203,75,22),RgbColor::new(203,75,22),RgbColor::new(203,75,22),RgbColor::new(203,75,22),RgbColor::new(173,45,0),RgbColor::new(143,15,0),RgbColor::new(113,0,0),RgbColor::new(0,43,54)]), info: ColorShade::custom([RgbColor::new(238,232,213),RgbColor::new(253,246,227),RgbColor::new(42,161,152),RgbColor::new(42,161,152),RgbColor::new(42,161,152),RgbColor::new(42,161,152),RgbColor::new(12,131,122),RgbColor::new(0,101,92),RgbColor::new(0,71,62),RgbColor::new(0,43,54)]), neutral: ColorShade::custom([RgbColor::new(253,246,227),RgbColor::new(238,232,213),RgbColor::new(147,161,161),RgbColor::new(131,148,150),RgbColor::new(101,123,142),RgbColor::new(88,110,117),RgbColor::new(7,54,66),RgbColor::new(0,43,54),RgbColor::new(0,43,54),RgbColor::new(0,43,54)]), variant: PaletteVariant::Solarized } }

    pub fn all_variants() -> &'static [PaletteVariant] {
        &[PaletteVariant::Default, PaletteVariant::Nord, PaletteVariant::Dracula, PaletteVariant::Monokai, PaletteVariant::Solarized]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rgb_creation() {
        let color = RgbColor::new(255, 128, 0);
        assert_eq!(color.r, 255);
        assert_eq!(color.g, 128);
        assert_eq!(color.b, 0);
    }

    #[test]
    fn test_hex_parsing() {
        assert!(RgbColor::from_hex("#FF8000").is_ok());
        assert!(RgbColor::from_hex("00FF00").is_ok());
        assert!(RgbColor::from_hex("INVALID").is_err());
    }

    #[test]
    fn test_lighten_darken() {
        let c = RgbColor::new(100, 100, 100);
        let l = c.lighten(50);
        assert!(l.r > c.r && l.g > c.g && l.b > c.b);

        let d = RgbColor::new(200, 200, 200).darken(50);
        assert!(d.r < 200 && d.g < 200 && d.b < 200);
    }

    #[test]
    fn test_mix() {
        let red = RgbColor::new(255, 0, 0);
        let blue = RgbColor::new(0, 0, 255);
        let mixed = red.mix(&blue, 0.5);
        assert_eq!(mixed.r, 127);
        assert_eq!(mixed.b, 127);
    }

    #[test]
    fn test_all_palettes() {
        let variants = ColorPalette::all_variants();
        assert_eq!(variants.len(), 5);
        for v in variants {
            let p = ColorPalette::new(*v);
            assert_eq!(p.variant, *v);
        }
    }
}
