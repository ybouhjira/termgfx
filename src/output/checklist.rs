use owo_colors::OwoColorize;
use serde::{Deserialize, Serialize};
use std::io::{self, Write};
use unicode_width::UnicodeWidthStr;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChecklistItem {
    pub label: String,
    pub checked: bool,
    pub columns: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChecklistOutput {
    pub items: Vec<ChecklistItem>,
    pub completed: usize,
    pub total: usize,
    pub percentage: f64,
}

/// Parse items from format: "Task A:done:2h,Task B:pending:1h"
/// or "Task A:✓:2h,Task B:x:1h"
fn parse_items(items_str: &str) -> Vec<ChecklistItem> {
    items_str
        .split(',')
        .filter(|s| !s.trim().is_empty())
        .map(|item| {
            let parts: Vec<&str> = item.split(':').collect();
            let label = parts.first().unwrap_or(&"").trim().to_string();
            let status = parts.get(1).unwrap_or(&"pending").trim().to_lowercase();
            let checked = matches!(
                status.as_str(),
                "done" | "complete" | "completed" | "yes" | "true" | "1" | "✓" | "✔" | "☑"
            );
            let columns: Vec<String> = parts
                .iter()
                .skip(2)
                .map(|s| s.trim().to_string())
                .collect();
            ChecklistItem {
                label,
                checked,
                columns,
            }
        })
        .collect()
}

/// Parse column headers from format: "Status,Duration"
fn parse_columns(columns_str: &str) -> Vec<String> {
    columns_str
        .split(',')
        .map(|s| s.trim().to_string())
        .collect()
}

/// Calculate column widths for proper alignment
fn calculate_widths(items: &[ChecklistItem], headers: &[String]) -> (usize, Vec<usize>) {
    // Label width (checkbox + space + label)
    let label_width = items
        .iter()
        .map(|i| UnicodeWidthStr::width(i.label.as_str()))
        .max()
        .unwrap_or(10);

    // Column widths
    let mut col_widths: Vec<usize> = headers
        .iter()
        .map(|h| UnicodeWidthStr::width(h.as_str()))
        .collect();

    for item in items {
        for (i, col) in item.columns.iter().enumerate() {
            if i < col_widths.len() {
                col_widths[i] = col_widths[i].max(UnicodeWidthStr::width(col.as_str()));
            } else {
                col_widths.push(UnicodeWidthStr::width(col.as_str()));
            }
        }
    }

    (label_width, col_widths)
}

/// Render a single checklist item
fn render_item(item: &ChecklistItem, label_width: usize, col_widths: &[usize]) {
    let checkbox = if item.checked {
        "☑".green().to_string()
    } else {
        "☐".dimmed().to_string()
    };

    let label = if item.checked {
        item.label.green().to_string()
    } else {
        item.label.white().to_string()
    };

    print!("{} {:width$}", checkbox, label, width = label_width);

    for (i, col) in item.columns.iter().enumerate() {
        let width = col_widths.get(i).copied().unwrap_or(10);
        let styled = if item.checked {
            col.dimmed().to_string()
        } else {
            col.white().to_string()
        };
        print!("  {:width$}", styled, width = width);
    }

    println!();
}

/// Render stats summary
fn render_stats(completed: usize, total: usize) {
    let percentage = if total > 0 {
        (completed as f64 / total as f64) * 100.0
    } else {
        0.0
    };

    println!();

    let stats_text = format!("Stats: {}/{} completed ({:.0}%)", completed, total, percentage);

    if percentage == 100.0 {
        println!("{}", stats_text.green().bold());
    } else if percentage >= 50.0 {
        println!("{}", stats_text.yellow());
    } else {
        println!("{}", stats_text.red());
    }
}

/// Render the checklist as JSON
fn render_json(items: &[ChecklistItem]) {
    let completed = items.iter().filter(|i| i.checked).count();
    let total = items.len();
    let percentage = if total > 0 {
        (completed as f64 / total as f64) * 100.0
    } else {
        0.0
    };

    let output = ChecklistOutput {
        items: items.to_vec(),
        completed,
        total,
        percentage,
    };

    match serde_json::to_string_pretty(&output) {
        Ok(json) => println!("{}", json),
        Err(e) => eprintln!("Error serializing to JSON: {}", e),
    }
}

/// Main render function
pub fn render(items_str: &str, columns_str: Option<&str>, output_json: bool, show_stats: bool) {
    let items = parse_items(items_str);

    if items.is_empty() {
        eprintln!("No items to display");
        return;
    }

    if output_json {
        render_json(&items);
        return;
    }

    let headers = columns_str.map(parse_columns).unwrap_or_default();
    let (label_width, col_widths) = calculate_widths(&items, &headers);

    // Render header if columns provided
    if !headers.is_empty() {
        print!("  {:width$}", "", width = label_width);
        for (i, header) in headers.iter().enumerate() {
            let width = col_widths.get(i).copied().unwrap_or(10);
            print!("  {:width$}", header.bold(), width = width);
        }
        println!();
        print!("  {:width$}", "", width = label_width);
        for (i, _) in headers.iter().enumerate() {
            let width = col_widths.get(i).copied().unwrap_or(10);
            print!("  {}", "─".repeat(width).dimmed());
        }
        println!();
    }

    // Render items
    for item in &items {
        render_item(item, label_width, &col_widths);
    }

    // Render stats
    if show_stats {
        let completed = items.iter().filter(|i| i.checked).count();
        render_stats(completed, items.len());
    }

    io::stdout().flush().ok();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_items_basic() {
        let items = parse_items("Task A:done:2h,Task B:pending:1h");
        assert_eq!(items.len(), 2);
        assert!(items[0].checked);
        assert_eq!(items[0].label, "Task A");
        assert_eq!(items[0].columns, vec!["2h"]);
        assert!(!items[1].checked);
    }

    #[test]
    fn test_parse_items_various_statuses() {
        let items = parse_items("A:done,B:complete,C:yes,D:true,E:1,F:✓");
        assert!(items.iter().all(|i| i.checked));

        let items = parse_items("A:pending,B:no,C:false,D:0,E:x");
        assert!(items.iter().all(|i| !i.checked));
    }

    #[test]
    fn test_parse_columns() {
        let cols = parse_columns("Status,Duration,Priority");
        assert_eq!(cols, vec!["Status", "Duration", "Priority"]);
    }

    #[test]
    fn test_calculate_widths() {
        let items = vec![
            ChecklistItem {
                label: "Short".to_string(),
                checked: true,
                columns: vec!["1h".to_string()],
            },
            ChecklistItem {
                label: "Longer task name".to_string(),
                checked: false,
                columns: vec!["30min".to_string()],
            },
        ];
        let headers = vec!["Duration".to_string()];
        let (label_width, col_widths) = calculate_widths(&items, &headers);
        assert_eq!(label_width, 16); // "Longer task name"
        assert_eq!(col_widths[0], 8); // "Duration"
    }
}
