//! UI rendering for the studio TUI

use ratatui::{
    prelude::*,
    widgets::{Block, Borders, Clear, List, ListItem, Paragraph, Wrap},
};
use std::collections::HashMap;

use super::app::{FocusedPanel, StudioApp};
use super::layout::StudioAreas;
use super::registry::{ComponentDef, ParamType};

/// Render the entire studio UI
pub fn render(frame: &mut Frame, app: &StudioApp, areas: StudioAreas) {
    render_sidebar(frame, app, areas.sidebar);
    render_params(frame, app, areas.params);
    render_preview(frame, app, areas.preview);
    render_command(frame, app, areas.command);
}

/// Render the component sidebar
fn render_sidebar(frame: &mut Frame, app: &StudioApp, area: Rect) {
    let focused = matches!(app.focused_panel, FocusedPanel::Sidebar);
    let border_style = if focused {
        Style::default().fg(Color::Cyan).bold()
    } else {
        Style::default().fg(Color::DarkGray)
    };

    let block = Block::default()
        .title(" Components ")
        .borders(Borders::ALL)
        .border_style(border_style);

    let mut items: Vec<ListItem> = Vec::new();
    let mut current_category = "";

    for (idx, component) in app.components.iter().enumerate() {
        // Add category header if changed
        if component.category != current_category {
            current_category = component.category;
            let header = ListItem::new(Line::from(vec![Span::styled(
                format!(" {} ", component.category.to_uppercase()),
                Style::default().fg(Color::Yellow).bold(),
            )]));
            items.push(header);
        }

        // Add component item
        let style = if idx == app.selected_component {
            Style::default().fg(Color::Green).bold()
        } else {
            Style::default()
        };

        let marker = if idx == app.selected_component {
            "▶ "
        } else {
            "  "
        };
        let item = ListItem::new(Line::from(vec![
            Span::styled(marker, style),
            Span::styled(component.name, style),
        ]));
        items.push(item);
    }

    let list = List::new(items).block(block);

    frame.render_widget(list, area);
}

/// Render the parameter editor panel
fn render_params(frame: &mut Frame, app: &StudioApp, area: Rect) {
    let focused = matches!(app.focused_panel, FocusedPanel::Params);
    let border_style = if focused {
        Style::default().fg(Color::Cyan).bold()
    } else {
        Style::default().fg(Color::DarkGray)
    };

    let block = Block::default()
        .title(" Parameters ")
        .borders(Borders::ALL)
        .border_style(border_style);

    let inner = block.inner(area);
    frame.render_widget(block, area);

    if let Some(component) = app.components.get(app.selected_component) {
        let mut lines: Vec<Line> = Vec::new();

        for (idx, param) in component.params.iter().enumerate() {
            let is_selected = idx == app.selected_param;
            let is_editing = is_selected && app.editing;

            let marker = if is_selected { "▶ " } else { "  " };
            let marker_style = if is_selected {
                Style::default().fg(Color::Green)
            } else {
                Style::default().fg(Color::DarkGray)
            };

            let value = app
                .param_values
                .get(param.name)
                .map(|s| s.as_str())
                .unwrap_or(param.default);
            let display_value = if is_editing {
                format!("{}█", app.edit_buffer)
            } else {
                value.to_string()
            };

            let value_style = if is_editing {
                Style::default().fg(Color::Yellow)
            } else if is_selected {
                Style::default().fg(Color::White)
            } else {
                Style::default().fg(Color::Gray)
            };

            // Type hint
            let type_hint = match &param.param_type {
                ParamType::String => "str",
                ParamType::Number { .. } => "num",
                ParamType::Enum(opts) => &opts.join("|"),
                ParamType::Bool => "bool",
                ParamType::Data => "data",
            };

            let line = Line::from(vec![
                Span::styled(marker, marker_style),
                Span::styled(
                    format!("{:12}", param.name),
                    Style::default().fg(Color::Cyan),
                ),
                Span::raw(": "),
                Span::styled(format!("{:20}", display_value), value_style),
                Span::styled(
                    format!(" ({})", type_hint),
                    Style::default().fg(Color::DarkGray),
                ),
            ]);
            lines.push(line);
        }

        let paragraph = Paragraph::new(lines);
        frame.render_widget(paragraph, inner);
    }
}

/// Render the live preview panel
fn render_preview(frame: &mut Frame, app: &StudioApp, area: Rect) {
    let focused = matches!(app.focused_panel, FocusedPanel::Preview);
    let border_style = if focused {
        Style::default().fg(Color::Cyan).bold()
    } else {
        Style::default().fg(Color::DarkGray)
    };

    let block = Block::default()
        .title(" Live Preview ")
        .borders(Borders::ALL)
        .border_style(border_style);

    let inner = block.inner(area);
    frame.render_widget(block, area);

    if let Some(component) = app.components.get(app.selected_component) {
        let preview_text = generate_preview(component, &app.param_values);
        let paragraph = Paragraph::new(preview_text).wrap(Wrap { trim: false });
        frame.render_widget(paragraph, inner);
    }
}

/// Generate preview text for a component
fn generate_preview(
    component: &ComponentDef,
    values: &HashMap<String, String>,
) -> Vec<Line<'static>> {
    let mut lines = Vec::new();

    match component.name {
        "box" => {
            let message = values
                .get("message")
                .map(|s| s.as_str())
                .unwrap_or("Hello World!");
            let style = values.get("style").map(|s| s.as_str()).unwrap_or("info");
            let border = values
                .get("border")
                .map(|s| s.as_str())
                .unwrap_or("rounded");
            let emoji = values.get("emoji").map(|s| s.as_str()).unwrap_or("");

            let (tl, tr, bl, br, h, v) = match border {
                "rounded" => ("╭", "╮", "╰", "╯", "─", "│"),
                "double" => ("╔", "╗", "╚", "╝", "═", "║"),
                "thick" => ("┏", "┓", "┗", "┛", "━", "┃"),
                _ => ("┌", "┐", "└", "┘", "─", "│"),
            };

            let color = match style {
                "success" => Color::Green,
                "warning" => Color::Yellow,
                "danger" => Color::Red,
                _ => Color::Cyan,
            };

            let content = if emoji.is_empty() {
                message.to_string()
            } else {
                format!("{} {}", emoji, message)
            };
            let width = content.len() + 2;

            lines.push(Line::from(Span::styled(
                format!("{}{}{}", tl, h.repeat(width), tr),
                Style::default().fg(color),
            )));
            lines.push(Line::from(Span::styled(
                format!("{} {} {}", v, content, v),
                Style::default().fg(color),
            )));
            lines.push(Line::from(Span::styled(
                format!("{}{}{}", bl, h.repeat(width), br),
                Style::default().fg(color),
            )));
        }
        "progress" => {
            let percent: u8 = values
                .get("percent")
                .and_then(|s| s.parse().ok())
                .unwrap_or(50);
            let style = values
                .get("style")
                .map(|s| s.as_str())
                .unwrap_or("gradient");

            let width = 30;
            let filled = (width * percent as usize) / 100;

            let (filled_char, empty_char) = match style {
                "blocks" => ("█", "░"),
                "classic" => ("=", "-"),
                _ => ("█", "░"),
            };

            let bar = format!(
                "{}{} {}%",
                filled_char.repeat(filled),
                empty_char.repeat(width - filled),
                percent
            );

            lines.push(Line::from(Span::styled(
                bar,
                Style::default().fg(Color::Green),
            )));
        }
        "gauge" => {
            let value: f64 = values
                .get("value")
                .and_then(|s| s.parse().ok())
                .unwrap_or(75.0);
            let label = values.get("label").map(|s| s.as_str()).unwrap_or("CPU");

            let segments = 20;
            let filled = ((value / 100.0) * segments as f64) as usize;

            let gauge: String = (0..segments)
                .map(|i| if i < filled { "●" } else { "○" })
                .collect();

            lines.push(Line::from(Span::styled(
                gauge,
                Style::default().fg(Color::Cyan),
            )));
            lines.push(Line::from(format!("{}: {:.0}%", label, value)));
        }
        "sparkline" => {
            let data = values
                .get("data")
                .map(|s| s.as_str())
                .unwrap_or("1,4,2,8,5,7");
            let values: Vec<f64> = data
                .split(',')
                .filter_map(|s| s.trim().parse().ok())
                .collect();

            if !values.is_empty() {
                let max = values.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
                let min = values.iter().cloned().fold(f64::INFINITY, f64::min);
                let range = if max == min { 1.0 } else { max - min };

                let chars = ['▁', '▂', '▃', '▄', '▅', '▆', '▇', '█'];
                let spark: String = values
                    .iter()
                    .map(|v| {
                        let idx = (((v - min) / range) * 7.0) as usize;
                        chars[idx.min(7)]
                    })
                    .collect();

                lines.push(Line::from(Span::styled(
                    spark,
                    Style::default().fg(Color::Green),
                )));
            }
        }
        "spinner" => {
            use crate::output::spinner::SpinnerStyle;

            let message = values
                .get("message")
                .map(|s| s.as_str())
                .unwrap_or("Loading...");
            let style_str = values.get("style").map(|s| s.as_str()).unwrap_or("dots");
            let style: SpinnerStyle = style_str.parse().unwrap_or_default();

            // Show the first frame of the spinner
            let frame = style.frames()[0];

            lines.push(Line::from(vec![
                Span::styled(frame, Style::default().fg(Color::Cyan)),
                Span::raw(" "),
                Span::raw(message.to_string()),
            ]));
            lines.push(Line::from(""));
            lines.push(Line::from(Span::styled(
                format!("Style: {} ({} frames)", style_str, style.frames().len()),
                Style::default().fg(Color::DarkGray).italic(),
            )));
        }
        _ => {
            lines.push(Line::from(Span::styled(
                format!("Preview for '{}' component", component.name),
                Style::default().fg(Color::DarkGray),
            )));
            lines.push(Line::from(""));
            lines.push(Line::from(Span::styled(
                "(Full preview requires running the command)",
                Style::default().fg(Color::DarkGray).italic(),
            )));
        }
    }

    lines
}

/// Render the command panel
fn render_command(frame: &mut Frame, app: &StudioApp, area: Rect) {
    let border_style = Style::default().fg(Color::DarkGray);

    let block = Block::default()
        .title(" Command ")
        .borders(Borders::ALL)
        .border_style(border_style);

    let inner = block.inner(area);
    frame.render_widget(block, area);

    if let Some(component) = app.components.get(app.selected_component) {
        let cmd = component.generate_command(&app.param_values);

        let lines = vec![
            Line::from(vec![
                Span::styled("$ ", Style::default().fg(Color::Green)),
                Span::styled(cmd, Style::default().fg(Color::White)),
            ]),
            Line::from(""),
            Line::from(Span::styled(
                "[c] Copy   [Enter] Run   [?] Help   [q] Quit",
                Style::default().fg(Color::DarkGray),
            )),
        ];

        let paragraph = Paragraph::new(lines);
        frame.render_widget(paragraph, inner);
    }
}

/// Render the help overlay with keyboard shortcuts
pub fn render_help_overlay(frame: &mut Frame) {
    let area = frame.area();

    // Center the help panel
    let help_width = 50;
    let help_height = 22;
    let x = (area.width.saturating_sub(help_width)) / 2;
    let y = (area.height.saturating_sub(help_height)) / 2;
    let help_area = Rect::new(x, y, help_width, help_height);

    // Clear the area behind the popup
    frame.render_widget(Clear, help_area);

    let block = Block::default()
        .title(" ⌨ Keyboard Shortcuts ")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Cyan).bold())
        .style(Style::default().bg(Color::Black));

    let shortcuts = vec![
        ("", ""),
        (" Navigation", ""),
        ("  Tab / Shift+Tab", "Cycle panels"),
        ("  1 / 2 / 3", "Jump to panel"),
        ("  j/↓  k/↑", "Navigate items"),
        ("  h/←  l/→", "Move between panels"),
        ("", ""),
        (" Editing", ""),
        ("  Enter", "Edit parameter"),
        ("  Space", "Toggle bool / cycle enum"),
        ("  r", "Reset to defaults"),
        ("  Esc", "Cancel edit"),
        ("", ""),
        (" Actions", ""),
        ("  c", "Copy command"),
        ("  ?", "Toggle this help"),
        ("  q / Esc", "Quit"),
        ("", ""),
    ];

    let lines: Vec<Line> = shortcuts
        .iter()
        .map(|(key, desc)| {
            if desc.is_empty() {
                Line::from(Span::styled(
                    *key,
                    Style::default().fg(Color::Yellow).bold(),
                ))
            } else {
                Line::from(vec![
                    Span::styled(format!("{:18}", key), Style::default().fg(Color::Green)),
                    Span::styled(*desc, Style::default().fg(Color::White)),
                ])
            }
        })
        .collect();

    let paragraph = Paragraph::new(lines).block(block);
    frame.render_widget(paragraph, help_area);
}

/// Render a status message at the bottom of the screen
pub fn render_status_message(frame: &mut Frame, message: &str) {
    let area = frame.area();

    // Position at bottom center
    let msg_width = (message.len() + 4).min(area.width as usize) as u16;
    let x = (area.width.saturating_sub(msg_width)) / 2;
    let y = area.height.saturating_sub(2);
    let status_area = Rect::new(x, y, msg_width, 1);

    let paragraph = Paragraph::new(Line::from(Span::styled(
        format!(" {} ", message),
        Style::default().fg(Color::Black).bg(Color::Green).bold(),
    )));

    frame.render_widget(paragraph, status_area);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_box_preview() {
        let component = ComponentDef {
            name: "box",
            description: "Test",
            category: "Output",
            params: vec![],
        };

        let mut values = HashMap::new();
        values.insert("message".to_string(), "Hello".to_string());
        values.insert("style".to_string(), "success".to_string());

        let lines = generate_preview(&component, &values);
        assert!(!lines.is_empty());
    }

    #[test]
    fn test_generate_progress_preview() {
        let component = ComponentDef {
            name: "progress",
            description: "Test",
            category: "Output",
            params: vec![],
        };

        let mut values = HashMap::new();
        values.insert("percent".to_string(), "75".to_string());

        let lines = generate_preview(&component, &values);
        assert!(!lines.is_empty());
    }
}
