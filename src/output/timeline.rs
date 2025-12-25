use owo_colors::OwoColorize;
use std::io::{self, IsTerminal};
use std::thread;
use std::time::Duration;

pub struct TimelineArgs {
    pub events: String,
    pub style: String,
    pub color: Option<String>,
    pub animate: bool,
    pub vertical: bool,
}

#[derive(Debug)]
struct Event {
    date: Option<String>,
    label: String,
}

impl Event {
    fn parse(event_str: &str) -> Self {
        if let Some((date, label)) = event_str.split_once(':') {
            Event {
                date: Some(date.to_string()),
                label: label.to_string(),
            }
        } else {
            Event {
                date: None,
                label: event_str.to_string(),
            }
        }
    }
}

pub fn render_timeline(args: &TimelineArgs) -> io::Result<()> {
    let events: Vec<Event> = args
        .events
        .split(',')
        .map(|s| Event::parse(s.trim()))
        .collect();

    if events.is_empty() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "No events provided",
        ));
    }

    let is_tty = io::stdout().is_terminal();
    let can_animate = args.animate && is_tty;

    if args.vertical {
        render_vertical_timeline(&events, &args.style, &args.color, can_animate)
    } else {
        render_horizontal_timeline(&events, &args.style, &args.color, can_animate)
    }
}

fn render_horizontal_timeline(
    events: &[Event],
    style: &str,
    color: &Option<String>,
    animate: bool,
) -> io::Result<()> {
    let (marker, connector) = match style {
        "arrow" => ("●", "─"),
        "line" => ("┬", "─"),
        "dots" => ("●", "·"),
        _ => ("●", "─"),
    };

    // Calculate spacing
    let max_label_len = events
        .iter()
        .map(|e| e.label.len())
        .max()
        .unwrap_or(0);
    let segment_width = max_label_len.max(10);

    // Render dates if present
    if events.iter().any(|e| e.date.is_some()) {
        for (i, event) in events.iter().enumerate() {
            if animate {
                thread::sleep(Duration::from_millis(100));
            }
            if let Some(date) = &event.date {
                let date_str = apply_color(date, color);
                print!("{:width$}", date_str, width = segment_width);
                if i < events.len() - 1 {
                    print!("  ");
                }
            } else {
                print!("{:width$}", "", width = segment_width);
                if i < events.len() - 1 {
                    print!("  ");
                }
            }
        }
        println!();
    }

    // Render timeline line with markers
    for (i, _) in events.iter().enumerate() {
        if animate {
            thread::sleep(Duration::from_millis(100));
        }
        let marker_str = apply_color(marker, color);
        print!("{}", marker_str);

        if i < events.len() - 1 {
            let line = connector.repeat(segment_width + 1);
            let line_str = apply_color(&line, color);
            print!("{}", line_str);
        }
    }
    println!();

    // Render labels
    for (i, event) in events.iter().enumerate() {
        if animate {
            thread::sleep(Duration::from_millis(100));
        }
        let label_str = apply_color(&event.label, color);
        print!("{:width$}", label_str, width = segment_width);
        if i < events.len() - 1 {
            print!("  ");
        }
    }
    println!();

    Ok(())
}

fn render_vertical_timeline(
    events: &[Event],
    style: &str,
    color: &Option<String>,
    animate: bool,
) -> io::Result<()> {
    let (marker, connector) = match style {
        "arrow" => ("●", "│"),
        "line" => ("┼", "│"),
        "dots" => ("●", "┊"),
        _ => ("●", "│"),
    };

    for (i, event) in events.iter().enumerate() {
        if animate {
            thread::sleep(Duration::from_millis(200));
        }

        // Render marker and label
        let marker_str = apply_color(marker, color);
        let label_str = apply_color(&event.label, color);

        if let Some(date) = &event.date {
            let date_str = apply_color(date, color);
            println!("{} {} ({})", marker_str, label_str, date_str);
        } else {
            println!("{} {}", marker_str, label_str);
        }

        // Render connector (except after last event)
        if i < events.len() - 1 {
            let connector_str = apply_color(connector, color);
            println!("{}", connector_str);
        }
    }

    Ok(())
}

fn apply_color(text: &str, color: &Option<String>) -> String {
    if let Some(c) = color {
        match c.to_lowercase().as_str() {
            "red" => text.red().to_string(),
            "green" => text.green().to_string(),
            "blue" => text.blue().to_string(),
            "yellow" => text.yellow().to_string(),
            "magenta" => text.magenta().to_string(),
            "cyan" => text.cyan().to_string(),
            "white" => text.white().to_string(),
            _ => text.to_string(),
        }
    } else {
        text.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_event_parse_with_date() {
        let event = Event::parse("2024-01:Start");
        assert_eq!(event.date, Some("2024-01".to_string()));
        assert_eq!(event.label, "Start");
    }

    #[test]
    fn test_event_parse_without_date() {
        let event = Event::parse("Start");
        assert_eq!(event.date, None);
        assert_eq!(event.label, "Start");
    }
}
