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
        "banner" => {
            let text = values.get("text").map(|s| s.as_str()).unwrap_or("Hello");
            let style = values
                .get("style")
                .map(|s| s.as_str())
                .unwrap_or("gradient");
            let _font = values.get("font").map(|s| s.as_str()).unwrap_or("standard");

            // Simple ASCII art banner preview
            let banner_line = format!("═══ {} ═══", text.to_uppercase());
            let color = match style {
                "rainbow" => Color::Magenta,
                "solid" => Color::White,
                _ => Color::Cyan,
            };
            lines.push(Line::from(Span::styled(banner_line, Style::default().fg(color))));
            lines.push(Line::from(""));
            lines.push(Line::from(Span::styled(
                "(Full ASCII art requires running the command)",
                Style::default().fg(Color::DarkGray).italic(),
            )));
        }
        "table" => {
            let headers = values
                .get("headers")
                .map(|s| s.as_str())
                .unwrap_or("Name,Value,Status");
            let rows = values
                .get("rows")
                .map(|s| s.as_str())
                .unwrap_or("Item1,100,OK|Item2,200,OK");
            let border = values
                .get("border")
                .map(|s| s.as_str())
                .unwrap_or("rounded");

            let (tl, tr, bl, br, h, v, cross) = match border {
                "double" => ("╔", "╗", "╚", "╝", "═", "║", "╬"),
                "single" => ("┌", "┐", "└", "┘", "─", "│", "┼"),
                _ => ("╭", "╮", "╰", "╯", "─", "│", "┼"),
            };

            let header_parts: Vec<&str> = headers.split(',').collect();
            let col_width = 10;

            // Top border
            let top = format!(
                "{}{}{}",
                tl,
                header_parts
                    .iter()
                    .map(|_| h.repeat(col_width))
                    .collect::<Vec<_>>()
                    .join(cross),
                tr
            );
            lines.push(Line::from(Span::styled(top, Style::default().fg(Color::Cyan))));

            // Header row
            let header_row = format!(
                "{} {} {}",
                v,
                header_parts
                    .iter()
                    .map(|h| format!("{:^width$}", h, width = col_width - 2))
                    .collect::<Vec<_>>()
                    .join(&format!(" {} ", v)),
                v
            );
            lines.push(Line::from(Span::styled(
                header_row,
                Style::default().fg(Color::Yellow).bold(),
            )));

            // Separator
            let sep = format!(
                "{}{}{}",
                v,
                header_parts
                    .iter()
                    .map(|_| h.repeat(col_width))
                    .collect::<Vec<_>>()
                    .join(cross),
                v
            );
            lines.push(Line::from(Span::styled(sep, Style::default().fg(Color::Cyan))));

            // Data rows
            for row in rows.split('|').take(3) {
                let cells: Vec<&str> = row.split(',').collect();
                let data_row = format!(
                    "{} {} {}",
                    v,
                    cells
                        .iter()
                        .map(|c| format!("{:^width$}", c, width = col_width - 2))
                        .collect::<Vec<_>>()
                        .join(&format!(" {} ", v)),
                    v
                );
                lines.push(Line::from(Span::styled(data_row, Style::default().fg(Color::White))));
            }

            // Bottom border
            let bottom = format!(
                "{}{}{}",
                bl,
                header_parts
                    .iter()
                    .map(|_| h.repeat(col_width))
                    .collect::<Vec<_>>()
                    .join(cross),
                br
            );
            lines.push(Line::from(Span::styled(bottom, Style::default().fg(Color::Cyan))));
        }
        "tree" => {
            let structure = values
                .get("structure")
                .map(|s| s.as_str())
                .unwrap_or("root>child1,child2>leaf1,leaf2");

            // Simple tree preview
            lines.push(Line::from(Span::styled("📁 root", Style::default().fg(Color::Yellow))));
            lines.push(Line::from(Span::styled("├── 📁 child1", Style::default().fg(Color::Cyan))));
            lines.push(Line::from(Span::styled("│   ├── 📄 leaf1", Style::default().fg(Color::Green))));
            lines.push(Line::from(Span::styled("│   └── 📄 leaf2", Style::default().fg(Color::Green))));
            lines.push(Line::from(Span::styled("└── 📁 child2", Style::default().fg(Color::Cyan))));
            lines.push(Line::from(""));
            lines.push(Line::from(Span::styled(
                format!("Structure: {}", structure),
                Style::default().fg(Color::DarkGray).italic(),
            )));
        }
        "timeline" => {
            let events = values
                .get("events")
                .map(|s| s.as_str())
                .unwrap_or("Start,Middle,End");
            let style = values.get("style").map(|s| s.as_str()).unwrap_or("arrow");
            let color_str = values.get("color").map(|s| s.as_str()).unwrap_or("cyan");

            let color = match color_str {
                "red" => Color::Red,
                "green" => Color::Green,
                "blue" => Color::Blue,
                "yellow" => Color::Yellow,
                "magenta" => Color::Magenta,
                "white" => Color::White,
                _ => Color::Cyan,
            };

            let event_list: Vec<&str> = events.split(',').collect();
            let connector = match style {
                "dots" => "····",
                "line" => "────",
                _ => "───→",
            };

            let timeline_str: String = event_list
                .iter()
                .enumerate()
                .map(|(i, e)| {
                    if i < event_list.len() - 1 {
                        format!("●{}{}", e, connector)
                    } else {
                        format!("●{}", e)
                    }
                })
                .collect();

            lines.push(Line::from(Span::styled(timeline_str, Style::default().fg(color))));
        }
        "heatmap" => {
            let data = values
                .get("data")
                .map(|s| s.as_str())
                .unwrap_or("1,2,3;4,5,6;7,8,9");
            let colors = values
                .get("colors")
                .map(|s| s.as_str())
                .unwrap_or("blue-red");

            let heat_chars = ['░', '▒', '▓', '█'];

            for row in data.split(';').take(5) {
                let cells: Vec<Span> = row
                    .split(',')
                    .filter_map(|s| s.trim().parse::<f64>().ok())
                    .map(|v| {
                        let idx = ((v / 10.0).min(3.0).max(0.0)) as usize;
                        let color = match colors {
                            "viridis" => match idx {
                                0 => Color::Blue,
                                1 => Color::Cyan,
                                2 => Color::Green,
                                _ => Color::Yellow,
                            },
                            "magma" => match idx {
                                0 => Color::Black,
                                1 => Color::Magenta,
                                2 => Color::Red,
                                _ => Color::Yellow,
                            },
                            "green-red" => match idx {
                                0 => Color::Green,
                                1 => Color::Yellow,
                                _ => Color::Red,
                            },
                            _ => match idx {
                                0 => Color::Blue,
                                1 => Color::Cyan,
                                2 => Color::Yellow,
                                _ => Color::Red,
                            },
                        };
                        Span::styled(
                            format!("{} ", heat_chars[idx]),
                            Style::default().fg(color),
                        )
                    })
                    .collect();
                lines.push(Line::from(cells));
            }
        }
        "checklist" => {
            let items = values
                .get("items")
                .map(|s| s.as_str())
                .unwrap_or("Task A:done:2h,Task B:pending:1h,Task C:done:30m");

            for item in items.split(',').take(5) {
                let parts: Vec<&str> = item.split(':').collect();
                let label = parts.first().unwrap_or(&"Task");
                let status = parts.get(1).unwrap_or(&"pending");
                let data = parts.get(2).unwrap_or(&"");

                let (icon, color) = if *status == "done" || *status == "complete" || *status == "yes" {
                    ("✓", Color::Green)
                } else {
                    ("○", Color::DarkGray)
                };

                lines.push(Line::from(vec![
                    Span::styled(format!("[{}] ", icon), Style::default().fg(color)),
                    Span::styled(label.to_string(), Style::default().fg(Color::White)),
                    Span::styled(format!("  {}", data), Style::default().fg(Color::DarkGray)),
                ]));
            }
        }
        "stats" => {
            let data = values
                .get("data")
                .map(|s| s.as_str())
                .unwrap_or("entries:500,size:2.3 MB,modified:2h ago");
            let separator = values
                .get("separator")
                .map(|s| s.as_str())
                .unwrap_or("pipe");
            let emoji = values.get("emoji").map(|s| s.as_str()).unwrap_or("");

            let sep_char = match separator {
                "dot" => "•",
                "slash" => "/",
                "bar" => "|",
                "diamond" => "◆",
                "arrow" => "→",
                _ => "│",
            };

            let stats_parts: Vec<String> = data
                .split(',')
                .map(|pair| {
                    let parts: Vec<&str> = pair.split(':').collect();
                    let label = parts.first().unwrap_or(&"");
                    let value = parts.get(1).unwrap_or(&"");
                    format!("{}: {}", label, value)
                })
                .collect();

            let stats_line = if emoji.is_empty() {
                stats_parts.join(&format!(" {} ", sep_char))
            } else {
                format!("{} {}", emoji, stats_parts.join(&format!(" {} ", sep_char)))
            };

            lines.push(Line::from(Span::styled(stats_line, Style::default().fg(Color::Cyan))));
        }
        "notification" => {
            let message = values
                .get("message")
                .map(|s| s.as_str())
                .unwrap_or("Task completed!");
            let title = values
                .get("title")
                .map(|s| s.as_str())
                .unwrap_or("TermGFX");
            let style = values.get("style").map(|s| s.as_str()).unwrap_or("info");

            let (icon, color) = match style {
                "success" => ("✓", Color::Green),
                "warning" => ("⚠", Color::Yellow),
                "error" => ("✕", Color::Red),
                _ => ("ℹ", Color::Cyan),
            };

            lines.push(Line::from(Span::styled(
                format!("╭─ {} {} ─╮", icon, title),
                Style::default().fg(color),
            )));
            lines.push(Line::from(Span::styled(
                format!("│ {}  │", message),
                Style::default().fg(color),
            )));
            lines.push(Line::from(Span::styled(
                format!("╰{}╯", "─".repeat(message.len() + 4)),
                Style::default().fg(color),
            )));
        }
        "danger-zone" => {
            let message = values
                .get("message")
                .map(|s| s.as_str())
                .unwrap_or("This action cannot be undone!");
            let title = values.get("title").map(|s| s.as_str()).unwrap_or("DANGER");

            let width = message.len().max(title.len()) + 4;
            lines.push(Line::from(Span::styled(
                format!("╔{}╗", "═".repeat(width)),
                Style::default().fg(Color::Red),
            )));
            lines.push(Line::from(Span::styled(
                format!("║ ⚠ {:^width$} ║", title, width = width - 4),
                Style::default().fg(Color::Red).bold(),
            )));
            lines.push(Line::from(Span::styled(
                format!("╠{}╣", "═".repeat(width)),
                Style::default().fg(Color::Red),
            )));
            lines.push(Line::from(Span::styled(
                format!("║  {:^width$}  ║", message, width = width - 4),
                Style::default().fg(Color::Yellow),
            )));
            lines.push(Line::from(Span::styled(
                format!("╚{}╝", "═".repeat(width)),
                Style::default().fg(Color::Red),
            )));
        }
        "chart bar" => {
            let data = values
                .get("data")
                .map(|s| s.as_str())
                .unwrap_or("Sales:100,Costs:60,Profit:40");

            let pairs: Vec<(&str, f64)> = data
                .split(',')
                .filter_map(|pair| {
                    let parts: Vec<&str> = pair.split(':').collect();
                    if parts.len() == 2 {
                        parts[1].parse::<f64>().ok().map(|v| (parts[0], v))
                    } else {
                        None
                    }
                })
                .collect();

            let max_val = pairs.iter().map(|(_, v)| *v).fold(0.0, f64::max);

            for (label, value) in pairs.iter().take(5) {
                let bar_width = if max_val > 0.0 {
                    ((value / max_val) * 20.0) as usize
                } else {
                    0
                };
                lines.push(Line::from(vec![
                    Span::styled(format!("{:8} ", label), Style::default().fg(Color::Cyan)),
                    Span::styled("█".repeat(bar_width), Style::default().fg(Color::Green)),
                    Span::styled(format!(" {}", value), Style::default().fg(Color::White)),
                ]));
            }
        }
        "chart pie" => {
            let data = values
                .get("data")
                .map(|s| s.as_str())
                .unwrap_or("A:40,B:30,C:20,D:10");

            let pairs: Vec<(&str, f64)> = data
                .split(',')
                .filter_map(|pair| {
                    let parts: Vec<&str> = pair.split(':').collect();
                    if parts.len() == 2 {
                        parts[1].parse::<f64>().ok().map(|v| (parts[0], v))
                    } else {
                        None
                    }
                })
                .collect();

            let total: f64 = pairs.iter().map(|(_, v)| *v).sum();
            let colors = [Color::Red, Color::Green, Color::Blue, Color::Yellow, Color::Magenta];

            // Simple ASCII pie representation
            lines.push(Line::from(Span::styled("    ╭───╮", Style::default().fg(Color::White))));
            lines.push(Line::from(Span::styled("   ╱     ╲", Style::default().fg(Color::White))));
            lines.push(Line::from(Span::styled("  │   ●   │", Style::default().fg(Color::White))));
            lines.push(Line::from(Span::styled("   ╲     ╱", Style::default().fg(Color::White))));
            lines.push(Line::from(Span::styled("    ╰───╯", Style::default().fg(Color::White))));
            lines.push(Line::from(""));

            for (i, (label, value)) in pairs.iter().enumerate().take(5) {
                let pct = if total > 0.0 { (value / total) * 100.0 } else { 0.0 };
                lines.push(Line::from(vec![
                    Span::styled("■ ", Style::default().fg(colors[i % colors.len()])),
                    Span::styled(format!("{}: {:.1}%", label, pct), Style::default().fg(Color::White)),
                ]));
            }
        }
        "chart line" => {
            let data = values
                .get("data")
                .map(|s| s.as_str())
                .unwrap_or("10,25,15,40,30,55,45");

            let values_list: Vec<f64> = data
                .split(',')
                .filter_map(|s| s.trim().parse().ok())
                .collect();

            if !values_list.is_empty() {
                let max = values_list.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
                let min = values_list.iter().cloned().fold(f64::INFINITY, f64::min);
                let range = if max == min { 1.0 } else { max - min };

                // Create a simple 5-row line chart
                for row in (0..5).rev() {
                    let threshold = min + (range * (row as f64 + 0.5) / 5.0);
                    let line_chars: String = values_list
                        .iter()
                        .map(|v| if *v >= threshold { "●" } else { " " })
                        .collect::<Vec<_>>()
                        .join(" ");
                    lines.push(Line::from(Span::styled(
                        format!("{:5.0} │{}", threshold, line_chars),
                        Style::default().fg(Color::Green),
                    )));
                }
                lines.push(Line::from(Span::styled(
                    format!("      └{}", "──".repeat(values_list.len())),
                    Style::default().fg(Color::DarkGray),
                )));
            }
        }
        "input" | "select" | "confirm" | "filter" | "form" => {
            let prompt = values
                .get("prompt")
                .map(|s| s.as_str())
                .unwrap_or("Enter value:");

            lines.push(Line::from(Span::styled(
                format!("? {}", prompt),
                Style::default().fg(Color::Cyan).bold(),
            )));
            lines.push(Line::from(Span::styled(
                "  ▸ [cursor here]",
                Style::default().fg(Color::DarkGray),
            )));
            lines.push(Line::from(""));
            lines.push(Line::from(Span::styled(
                "(Interactive component - run command to use)",
                Style::default().fg(Color::DarkGray).italic(),
            )));
        }
        "dashboard" => {
            let layout = values.get("layout").map(|s| s.as_str()).unwrap_or("2x2");
            let title = values.get("title").map(|s| s.as_str()).unwrap_or("Dashboard");

            lines.push(Line::from(Span::styled(
                format!("╔══════════════════════════════╗"),
                Style::default().fg(Color::Cyan),
            )));
            lines.push(Line::from(Span::styled(
                format!("║  {}  ║", format!("{:^26}", title)),
                Style::default().fg(Color::Yellow).bold(),
            )));
            lines.push(Line::from(Span::styled(
                format!("╠══════════════╦══════════════╣"),
                Style::default().fg(Color::Cyan),
            )));
            lines.push(Line::from(Span::styled(
                format!("║   Panel 1    ║   Panel 2    ║"),
                Style::default().fg(Color::White),
            )));
            lines.push(Line::from(Span::styled(
                format!("╠══════════════╬══════════════╣"),
                Style::default().fg(Color::Cyan),
            )));
            lines.push(Line::from(Span::styled(
                format!("║   Panel 3    ║   Panel 4    ║"),
                Style::default().fg(Color::White),
            )));
            lines.push(Line::from(Span::styled(
                format!("╚══════════════╩══════════════╝"),
                Style::default().fg(Color::Cyan),
            )));
            lines.push(Line::from(""));
            lines.push(Line::from(Span::styled(
                format!("Layout: {}", layout),
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
