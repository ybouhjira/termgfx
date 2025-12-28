//! Export functionality for termgfx output to SVG and PNG formats

#![allow(dead_code)]
#![allow(clippy::upper_case_acronyms)]

pub mod svg;

/// Supported export formats
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExportFormat {
    SVG,
    PNG,
}

impl ExportFormat {
    /// Parse export format from string
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "svg" => Some(ExportFormat::SVG),
            "png" => Some(ExportFormat::PNG),
            _ => None,
        }
    }

    /// Get file extension for format
    pub fn extension(&self) -> &str {
        match self {
            ExportFormat::SVG => "svg",
            ExportFormat::PNG => "png",
        }
    }
}

/// Export configuration
#[derive(Debug, Clone)]
pub struct ExportConfig {
    pub format: ExportFormat,
    pub width: u32,
    pub height: u32,
    pub scale: f32,
    pub background: String,
    pub font_size: u32,
}

impl Default for ExportConfig {
    fn default() -> Self {
        Self {
            format: ExportFormat::SVG,
            width: 800,
            height: 600,
            scale: 1.0,
            background: "#ffffff".to_string(),
            font_size: 14,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_export_format_from_str() {
        assert_eq!(ExportFormat::from_str("svg"), Some(ExportFormat::SVG));
        assert_eq!(ExportFormat::from_str("SVG"), Some(ExportFormat::SVG));
        assert_eq!(ExportFormat::from_str("png"), Some(ExportFormat::PNG));
        assert_eq!(ExportFormat::from_str("PNG"), Some(ExportFormat::PNG));
        assert_eq!(ExportFormat::from_str("gif"), None);
    }

    #[test]
    fn test_export_format_extension() {
        assert_eq!(ExportFormat::SVG.extension(), "svg");
        assert_eq!(ExportFormat::PNG.extension(), "png");
    }

    #[test]
    fn test_export_config_default() {
        let config = ExportConfig::default();
        assert_eq!(config.format, ExportFormat::SVG);
        assert_eq!(config.width, 800);
        assert_eq!(config.height, 600);
        assert_eq!(config.background, "#ffffff");
    }
}
