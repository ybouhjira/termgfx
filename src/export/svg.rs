//! SVG export functionality for terminal graphics

use super::ExportConfig;
use std::io::Write;

/// SVG builder for creating SVG output
pub struct SvgBuilder {
    config: ExportConfig,
    content: Vec<String>,
}

impl SvgBuilder {
    /// Create a new SVG builder
    pub fn new(config: ExportConfig) -> Self {
        Self {
            config,
            content: Vec::new(),
        }
    }

    /// Add text to the SVG
    pub fn add_text(&mut self, x: f32, y: f32, text: &str, color: &str, font_size: u32) {
        let svg_line = format!(
            r#"  <text x="{}" y="{}" font-family="monospace" font-size="{}" fill="{}">{}</text>"#,
            x,
            y,
            font_size,
            color,
            escape_xml(text)
        );
        self.content.push(svg_line);
    }

    /// Add a rectangle to the SVG
    #[allow(clippy::too_many_arguments)]
    pub fn add_rect(
        &mut self,
        x: f32,
        y: f32,
        width: f32,
        height: f32,
        fill: &str,
        stroke: &str,
        stroke_width: f32,
    ) {
        let svg_line = format!(
            r#"  <rect x="{}" y="{}" width="{}" height="{}" fill="{}" stroke="{}" stroke-width="{}"/>"#,
            x, y, width, height, fill, stroke, stroke_width
        );
        self.content.push(svg_line);
    }

    /// Add a line to the SVG
    pub fn add_line(
        &mut self,
        x1: f32,
        y1: f32,
        x2: f32,
        y2: f32,
        stroke: &str,
        stroke_width: f32,
    ) {
        let svg_line = format!(
            r#"  <line x1="{}" y1="{}" x2="{}" y2="{}" stroke="{}" stroke-width="{}"/>"#,
            x1, y1, x2, y2, stroke, stroke_width
        );
        self.content.push(svg_line);
    }

    /// Add a styled box (like termgfx box command)
    pub fn add_box(&mut self, x: f32, y: f32, width: f32, height: f32, text: &str, style: &str) {
        let (color, bg_color) = get_style_colors(style);

        // Background
        self.add_rect(x, y, width, height, bg_color, color, 1.0);

        // Text (centered vertically within box)
        let text_y = y + height / 2.0 + (self.config.font_size as f32 / 2.0);
        self.add_text(x + 10.0, text_y, text, color, self.config.font_size);
    }

    /// Add a bar chart
    pub fn add_bar_chart(&mut self, x: f32, y: f32, values: &[(String, f32)], max_value: f32) {
        let bar_width = 60.0;
        let bar_height = 200.0;
        let spacing = 20.0;
        let scale = bar_height / max_value;

        for (idx, (label, value)) in values.iter().enumerate() {
            let bar_x = x + (idx as f32) * (bar_width + spacing);
            let bar_y = y + bar_height - (value * scale);
            let height = value * scale;

            // Draw bar
            self.add_rect(bar_x, bar_y, bar_width, height, "#3b82f6", "#1e40af", 1.0);

            // Draw label
            self.add_text(
                bar_x + bar_width / 2.0 - 20.0,
                y + bar_height + 25.0,
                label,
                "#333333",
                12,
            );

            // Draw value
            self.add_text(
                bar_x + bar_width / 2.0 - 15.0,
                bar_y - 10.0,
                &value.to_string(),
                "#1e40af",
                11,
            );
        }
    }

    /// Add a progress bar
    pub fn add_progress_bar(
        &mut self,
        x: f32,
        y: f32,
        width: f32,
        height: f32,
        percent: f32,
        style: &str,
    ) {
        let (color, _) = get_style_colors(style);

        // Background bar
        self.add_rect(x, y, width, height, "#e5e7eb", "#9ca3af", 1.0);

        // Progress fill
        let fill_width = width * (percent / 100.0);
        self.add_rect(x, y, fill_width, height, color, color, 0.0);

        // Percentage text
        self.add_text(
            x + width / 2.0 - 15.0,
            y + height / 2.0 + 5.0,
            &format!("{}%", percent as i32),
            "#ffffff",
            self.config.font_size,
        );
    }

    /// Build and return SVG string
    pub fn build(&self) -> String {
        let mut svg = format!(
            r#"<?xml version="1.0" encoding="UTF-8"?>
<svg xmlns="http://www.w3.org/2000/svg" width="{}" height="{}" viewBox="0 0 {} {}">
  <defs>
    <style>
      text {{ font-family: 'Courier New', monospace; }}
      .info {{ fill: #3b82f6; }}
      .success {{ fill: #10b981; }}
      .warning {{ fill: #f59e0b; }}
      .danger {{ fill: #ef4444; }}
    </style>
  </defs>
  <!-- Background -->
  <rect width="{}" height="{}" fill="{}"/>
"#,
            self.config.width,
            self.config.height,
            self.config.width,
            self.config.height,
            self.config.width,
            self.config.height,
            self.config.background
        );

        // Add all content
        for line in &self.content {
            svg.push_str(line);
            svg.push('\n');
        }

        svg.push_str("</svg>\n");
        svg
    }

    /// Write SVG to writer
    pub fn write_to<W: Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(self.build().as_bytes())?;
        Ok(())
    }
}

/// Get color pair for a style
fn get_style_colors(style: &str) -> (&str, &str) {
    match style.to_lowercase().as_str() {
        "success" => ("#10b981", "#d1fae5"),
        "warning" => ("#f59e0b", "#fef3c7"),
        "danger" | "error" => ("#ef4444", "#fee2e2"),
        "info" => ("#3b82f6", "#dbeafe"),
        "gradient" => ("#a855f7", "#fce7f3"),
        _ => ("#6b7280", "#f3f4f6"),
    }
}

/// Escape XML special characters
fn escape_xml(s: &str) -> String {
    s.replace("&", "&amp;")
        .replace("<", "&lt;")
        .replace(">", "&gt;")
        .replace("\"", "&quot;")
        .replace("'", "&apos;")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_svg_builder_creation() {
        let config = ExportConfig::default();
        let builder = SvgBuilder::new(config);
        assert_eq!(builder.content.len(), 0);
    }

    #[test]
    fn test_svg_add_text() {
        let config = ExportConfig::default();
        let mut builder = SvgBuilder::new(config);
        builder.add_text(10.0, 20.0, "Hello", "#000000", 14);
        assert_eq!(builder.content.len(), 1);
        assert!(builder.content[0].contains("Hello"));
    }

    #[test]
    fn test_svg_add_rect() {
        let config = ExportConfig::default();
        let mut builder = SvgBuilder::new(config);
        builder.add_rect(0.0, 0.0, 100.0, 50.0, "#ff0000", "#000000", 1.0);
        assert_eq!(builder.content.len(), 1);
        assert!(builder.content[0].contains("rect"));
    }

    #[test]
    fn test_svg_add_box() {
        let config = ExportConfig::default();
        let mut builder = SvgBuilder::new(config);
        builder.add_box(50.0, 50.0, 200.0, 100.0, "Test Box", "success");
        assert!(builder.content.len() >= 2); // rect + text
        assert!(builder.content.iter().any(|s| s.contains("Test Box")));
    }

    #[test]
    fn test_svg_build() {
        let config = ExportConfig::default();
        let mut builder = SvgBuilder::new(config);
        builder.add_text(10.0, 20.0, "Test", "#000000", 14);
        let svg = builder.build();

        assert!(svg.starts_with("<?xml"));
        assert!(svg.contains("<svg"));
        assert!(svg.contains("</svg>"));
        assert!(svg.contains("Test"));
    }

    #[test]
    fn test_escape_xml() {
        assert_eq!(escape_xml("hello & world"), "hello &amp; world");
        assert_eq!(escape_xml("<tag>"), "&lt;tag&gt;");
        assert_eq!(escape_xml(r#""quoted""#), "&quot;quoted&quot;");
    }

    #[test]
    fn test_style_colors() {
        assert_eq!(get_style_colors("success"), ("#10b981", "#d1fae5"));
        assert_eq!(get_style_colors("danger"), ("#ef4444", "#fee2e2"));
        assert_eq!(get_style_colors("info"), ("#3b82f6", "#dbeafe"));
    }

    #[test]
    fn test_svg_add_progress_bar() {
        let config = ExportConfig::default();
        let mut builder = SvgBuilder::new(config);
        builder.add_progress_bar(50.0, 50.0, 300.0, 30.0, 75.0, "success");
        assert!(builder.content.len() >= 2); // background + fill
    }

    #[test]
    fn test_svg_add_bar_chart() {
        let config = ExportConfig::default();
        let mut builder = SvgBuilder::new(config);
        let data = vec![
            ("A".to_string(), 10.0),
            ("B".to_string(), 20.0),
            ("C".to_string(), 15.0),
        ];
        builder.add_bar_chart(50.0, 50.0, &data, 25.0);
        assert!(builder.content.len() >= 9); // 3 bars * 3 elements (rect + label + value)
    }
}
