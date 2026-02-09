//! Component registry with metadata for all termgfx components

use std::collections::HashMap;

/// Parameter type for component configuration
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum ParamType {
    String,
    Number { min: f64, max: f64 },
    Enum(Vec<&'static str>),
    Bool,
    Data, // For comma-separated data like sparklines
}

/// A single parameter definition
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct ParamDef {
    pub name: &'static str,
    pub param_type: ParamType,
    pub default: &'static str,
    pub description: &'static str,
}

/// Component definition with all metadata
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct ComponentDef {
    pub name: &'static str,
    pub description: &'static str,
    pub category: &'static str,
    pub params: Vec<ParamDef>,
}

impl ComponentDef {
    /// Generate CLI command from current parameter values
    pub fn generate_command(&self, values: &HashMap<String, String>) -> String {
        let mut cmd = format!("termgfx {}", self.name);

        for param in &self.params {
            if let Some(value) = values.get(param.name) {
                if value != param.default && !value.is_empty() {
                    match &param.param_type {
                        ParamType::Bool => {
                            if value == "true" {
                                cmd.push_str(&format!(" --{}", param.name));
                            }
                        }
                        ParamType::String | ParamType::Data => {
                            cmd.push_str(&format!(" --{} \"{}\"", param.name, value));
                        }
                        _ => {
                            cmd.push_str(&format!(" --{} {}", param.name, value));
                        }
                    }
                }
            }
        }

        cmd
    }
}

/// Get all registered components
pub fn get_all_components() -> Vec<ComponentDef> {
    vec![
        // OUTPUT category
        ComponentDef {
            name: "box",
            description: "Styled message boxes with borders",
            category: "Output",
            params: vec![
                ParamDef {
                    name: "message",
                    param_type: ParamType::String,
                    default: "Hello World!",
                    description: "The message to display",
                },
                ParamDef {
                    name: "style",
                    param_type: ParamType::Enum(vec!["info", "success", "warning", "danger"]),
                    default: "info",
                    description: "Box style/color scheme",
                },
                ParamDef {
                    name: "border",
                    param_type: ParamType::Enum(vec![
                        "rounded", "single", "double", "thick", "ascii",
                    ]),
                    default: "rounded",
                    description: "Border style",
                },
                ParamDef {
                    name: "emoji",
                    param_type: ParamType::String,
                    default: "",
                    description: "Optional emoji prefix",
                },
            ],
        },
        ComponentDef {
            name: "progress",
            description: "Progress bars with various styles",
            category: "Output",
            params: vec![
                ParamDef {
                    name: "percent",
                    param_type: ParamType::Number {
                        min: 0.0,
                        max: 100.0,
                    },
                    default: "50",
                    description: "Progress percentage (0-100)",
                },
                ParamDef {
                    name: "style",
                    param_type: ParamType::Enum(vec!["gradient", "blocks", "classic"]),
                    default: "gradient",
                    description: "Progress bar style",
                },
            ],
        },
        ComponentDef {
            name: "gauge",
            description: "Radial gauge indicators",
            category: "Output",
            params: vec![
                ParamDef {
                    name: "value",
                    param_type: ParamType::Number {
                        min: 0.0,
                        max: 100.0,
                    },
                    default: "75",
                    description: "Gauge value (0-100)",
                },
                ParamDef {
                    name: "label",
                    param_type: ParamType::String,
                    default: "CPU",
                    description: "Gauge label",
                },
                ParamDef {
                    name: "style",
                    param_type: ParamType::Enum(vec!["semicircle", "full", "minimal"]),
                    default: "semicircle",
                    description: "Gauge display style",
                },
            ],
        },
        ComponentDef {
            name: "banner",
            description: "ASCII art text banners",
            category: "Output",
            params: vec![
                ParamDef {
                    name: "text",
                    param_type: ParamType::String,
                    default: "Hello",
                    description: "Banner text",
                },
                ParamDef {
                    name: "style",
                    param_type: ParamType::Enum(vec!["gradient", "solid", "rainbow"]),
                    default: "gradient",
                    description: "Color style",
                },
                ParamDef {
                    name: "font",
                    param_type: ParamType::Enum(vec!["standard", "slant", "small", "big"]),
                    default: "standard",
                    description: "ASCII font",
                },
            ],
        },
        ComponentDef {
            name: "spinner",
            description: "Loading spinners with messages",
            category: "Output",
            params: vec![
                ParamDef {
                    name: "message",
                    param_type: ParamType::String,
                    default: "Loading...",
                    description: "Spinner message",
                },
                ParamDef {
                    name: "style",
                    param_type: ParamType::Enum(vec![
                        "dots", "line", "arc", "bouncing", "clock", "circle", "bounce", "moon",
                    ]),
                    default: "dots",
                    description: "Spinner animation style",
                },
                ParamDef {
                    name: "duration",
                    param_type: ParamType::Number {
                        min: 1.0,
                        max: 60.0,
                    },
                    default: "3",
                    description: "Duration in seconds",
                },
            ],
        },
        // CHARTS category
        ComponentDef {
            name: "sparkline",
            description: "Inline mini charts",
            category: "Charts",
            params: vec![ParamDef {
                name: "data",
                param_type: ParamType::Data,
                default: "1,4,2,8,5,7,3,9,6",
                description: "Comma-separated numeric values",
            }],
        },
        ComponentDef {
            name: "chart bar",
            description: "Horizontal bar charts",
            category: "Charts",
            params: vec![ParamDef {
                name: "data",
                param_type: ParamType::Data,
                default: "Sales:100,Costs:60,Profit:40",
                description: "Label:value pairs",
            }],
        },
        ComponentDef {
            name: "chart pie",
            description: "ASCII pie charts",
            category: "Charts",
            params: vec![ParamDef {
                name: "data",
                param_type: ParamType::Data,
                default: "A:40,B:30,C:20,D:10",
                description: "Label:value pairs",
            }],
        },
        // DATA category
        ComponentDef {
            name: "table",
            description: "Formatted data tables",
            category: "Data",
            params: vec![
                ParamDef {
                    name: "headers",
                    param_type: ParamType::String,
                    default: "Name,Value,Status",
                    description: "Comma-separated headers",
                },
                ParamDef {
                    name: "rows",
                    param_type: ParamType::Data,
                    default: "Item1,100,OK|Item2,200,OK",
                    description: "Pipe-separated rows",
                },
                ParamDef {
                    name: "border",
                    param_type: ParamType::Enum(vec!["rounded", "single", "double", "ascii"]),
                    default: "rounded",
                    description: "Border style",
                },
            ],
        },
        ComponentDef {
            name: "tree",
            description: "Tree structure display",
            category: "Data",
            params: vec![ParamDef {
                name: "structure",
                param_type: ParamType::String,
                default: "root>child1,child2>leaf1,leaf2",
                description: "Tree structure (> for children, , for siblings)",
            }],
        },
        // INTERACTIVE category
        ComponentDef {
            name: "preview",
            description: "Preview pane for data inspection",
            category: "Interactive",
            params: vec![
                ParamDef {
                    name: "title",
                    param_type: ParamType::String,
                    default: "Preview",
                    description: "Pane title",
                },
                ParamDef {
                    name: "items",
                    param_type: ParamType::Data,
                    default: "item1,item2,item3",
                    description: "Items to display",
                },
                ParamDef {
                    name: "style",
                    param_type: ParamType::Enum(vec!["info", "success", "warning", "danger"]),
                    default: "info",
                    description: "Pane style",
                },
                ParamDef {
                    name: "action",
                    param_type: ParamType::String,
                    default: "Select",
                    description: "Action button label",
                },
            ],
        },
        ComponentDef {
            name: "regex-filter",
            description: "Filter entries using regex patterns",
            category: "Interactive",
            params: vec![
                ParamDef {
                    name: "pattern",
                    param_type: ParamType::String,
                    default: "\\.log$",
                    description: "Regex pattern to match",
                },
                ParamDef {
                    name: "items",
                    param_type: ParamType::Data,
                    default: "app.log,config.json,error.log",
                    description: "Items to filter",
                },
                ParamDef {
                    name: "action",
                    param_type: ParamType::String,
                    default: "Apply",
                    description: "Action button label",
                },
            ],
        },
        ComponentDef {
            name: "danger-zone",
            description: "Warning box for destructive operations",
            category: "Interactive",
            params: vec![
                ParamDef {
                    name: "message",
                    param_type: ParamType::String,
                    default: "This action cannot be undone!",
                    description: "Warning message",
                },
                ParamDef {
                    name: "title",
                    param_type: ParamType::String,
                    default: "DANGER",
                    description: "Box title",
                },
            ],
        },
        // Charts - Line
        ComponentDef {
            name: "chart line",
            description: "Line chart with animation support",
            category: "Charts",
            params: vec![
                ParamDef {
                    name: "data",
                    param_type: ParamType::Data,
                    default: "10,25,15,40,30,55,45",
                    description: "Comma-separated numeric values",
                },
                ParamDef {
                    name: "title",
                    param_type: ParamType::String,
                    default: "Line Chart",
                    description: "Chart title",
                },
                ParamDef {
                    name: "animate",
                    param_type: ParamType::Bool,
                    default: "false",
                    description: "Animate line drawing",
                },
            ],
        },
        // Timeline
        ComponentDef {
            name: "timeline",
            description: "Horizontal/vertical timeline display",
            category: "Data",
            params: vec![
                ParamDef {
                    name: "events",
                    param_type: ParamType::Data,
                    default: "Start,Middle,End",
                    description: "Events (comma-separated or date:label format)",
                },
                ParamDef {
                    name: "style",
                    param_type: ParamType::Enum(vec!["arrow", "line", "dots"]),
                    default: "arrow",
                    description: "Timeline style",
                },
                ParamDef {
                    name: "color",
                    param_type: ParamType::Enum(vec!["red", "green", "blue", "yellow", "magenta", "cyan", "white"]),
                    default: "cyan",
                    description: "Timeline color",
                },
                ParamDef {
                    name: "vertical",
                    param_type: ParamType::Bool,
                    default: "false",
                    description: "Render vertically",
                },
                ParamDef {
                    name: "animate",
                    param_type: ParamType::Bool,
                    default: "false",
                    description: "Animate the timeline",
                },
            ],
        },
        // Heatmap
        ComponentDef {
            name: "heatmap",
            description: "2D heatmap visualization",
            category: "Charts",
            params: vec![
                ParamDef {
                    name: "data",
                    param_type: ParamType::String,
                    default: "1,2,3;4,5,6;7,8,9",
                    description: "2D data (semicolon separates rows)",
                },
                ParamDef {
                    name: "title",
                    param_type: ParamType::String,
                    default: "Heatmap",
                    description: "Chart title",
                },
                ParamDef {
                    name: "colors",
                    param_type: ParamType::Enum(vec!["blue-red", "green-red", "viridis", "magma"]),
                    default: "blue-red",
                    description: "Color scheme",
                },
                ParamDef {
                    name: "x-labels",
                    param_type: ParamType::String,
                    default: "A,B,C",
                    description: "X-axis labels",
                },
                ParamDef {
                    name: "y-labels",
                    param_type: ParamType::String,
                    default: "1,2,3",
                    description: "Y-axis labels",
                },
                ParamDef {
                    name: "animate",
                    param_type: ParamType::Bool,
                    default: "false",
                    description: "Animate the heatmap",
                },
            ],
        },
        // Diff
        ComponentDef {
            name: "diff",
            description: "File diff display with highlighting",
            category: "Data",
            params: vec![
                ParamDef {
                    name: "file1",
                    param_type: ParamType::String,
                    default: "file1.txt",
                    description: "First file path",
                },
                ParamDef {
                    name: "file2",
                    param_type: ParamType::String,
                    default: "file2.txt",
                    description: "Second file path",
                },
                ParamDef {
                    name: "unified",
                    param_type: ParamType::Bool,
                    default: "false",
                    description: "Use unified diff format",
                },
            ],
        },
        // Notification
        ComponentDef {
            name: "notification",
            description: "Desktop + terminal alerts",
            category: "Output",
            params: vec![
                ParamDef {
                    name: "message",
                    param_type: ParamType::String,
                    default: "Task completed!",
                    description: "Notification message",
                },
                ParamDef {
                    name: "title",
                    param_type: ParamType::String,
                    default: "TermGFX",
                    description: "Notification title",
                },
                ParamDef {
                    name: "style",
                    param_type: ParamType::Enum(vec!["info", "success", "warning", "error"]),
                    default: "info",
                    description: "Notification style",
                },
                ParamDef {
                    name: "sound",
                    param_type: ParamType::Bool,
                    default: "false",
                    description: "Play sound",
                },
            ],
        },
        // Checklist
        ComponentDef {
            name: "checklist",
            description: "Checklist with checkboxes and data columns",
            category: "Data",
            params: vec![
                ParamDef {
                    name: "items",
                    param_type: ParamType::Data,
                    default: "Task A:done:2h,Task B:pending:1h,Task C:done:30m",
                    description: "Items (Label:status:data format)",
                },
                ParamDef {
                    name: "columns",
                    param_type: ParamType::String,
                    default: "Duration",
                    description: "Column headers",
                },
            ],
        },
        // Stats
        ComponentDef {
            name: "stats",
            description: "Compact stats bar with key-value pairs",
            category: "Output",
            params: vec![
                ParamDef {
                    name: "data",
                    param_type: ParamType::Data,
                    default: "entries:500,size:2.3 MB,modified:2h ago",
                    description: "Stats data (label:value format)",
                },
                ParamDef {
                    name: "separator",
                    param_type: ParamType::Enum(vec!["pipe", "dot", "slash", "bar", "diamond", "arrow"]),
                    default: "pipe",
                    description: "Separator style",
                },
                ParamDef {
                    name: "emoji",
                    param_type: ParamType::String,
                    default: "",
                    description: "Emoji prefix",
                },
            ],
        },
        // Image
        ComponentDef {
            name: "image",
            description: "Display images in terminal",
            category: "Output",
            params: vec![
                ParamDef {
                    name: "path",
                    param_type: ParamType::String,
                    default: "image.png",
                    description: "Path or URL to image",
                },
                ParamDef {
                    name: "protocol",
                    param_type: ParamType::Enum(vec!["auto", "kitty", "sixel", "halfblock"]),
                    default: "auto",
                    description: "Image protocol",
                },
            ],
        },
        // Dashboard
        ComponentDef {
            name: "dashboard",
            description: "Multi-panel dashboard layout",
            category: "Output",
            params: vec![
                ParamDef {
                    name: "layout",
                    param_type: ParamType::Enum(vec!["2x2", "3x1", "1x3", "2x1"]),
                    default: "2x2",
                    description: "Layout grid",
                },
                ParamDef {
                    name: "title",
                    param_type: ParamType::String,
                    default: "Dashboard",
                    description: "Dashboard title",
                },
                ParamDef {
                    name: "panels",
                    param_type: ParamType::String,
                    default: "box:Hello,progress:75,sparkline:1;2;3,gauge:50",
                    description: "Panel definitions",
                },
                ParamDef {
                    name: "border",
                    param_type: ParamType::Enum(vec!["single", "double", "rounded"]),
                    default: "single",
                    description: "Border style",
                },
            ],
        },
        // Animate
        ComponentDef {
            name: "animate",
            description: "Run animation effects",
            category: "Animation",
            params: vec![
                ParamDef {
                    name: "effect-type",
                    param_type: ParamType::Enum(vec!["progress", "typewriter", "counter", "chart-build", "bars"]),
                    default: "typewriter",
                    description: "Animation type",
                },
                ParamDef {
                    name: "text",
                    param_type: ParamType::String,
                    default: "Hello, World!",
                    description: "Text content (for typewriter)",
                },
                ParamDef {
                    name: "duration",
                    param_type: ParamType::Number { min: 0.5, max: 30.0 },
                    default: "2.0",
                    description: "Duration in seconds",
                },
                ParamDef {
                    name: "speed",
                    param_type: ParamType::Number { min: 5.0, max: 200.0 },
                    default: "30.0",
                    description: "Speed (chars/sec for typewriter)",
                },
                ParamDef {
                    name: "from",
                    param_type: ParamType::Number { min: 0.0, max: 10000.0 },
                    default: "0",
                    description: "From value (for counter)",
                },
                ParamDef {
                    name: "to",
                    param_type: ParamType::Number { min: 0.0, max: 10000.0 },
                    default: "100",
                    description: "To value (for counter)",
                },
            ],
        },
        // Input
        ComponentDef {
            name: "input",
            description: "Text input prompt",
            category: "Interactive",
            params: vec![
                ParamDef {
                    name: "prompt",
                    param_type: ParamType::String,
                    default: "Enter your name:",
                    description: "Prompt question",
                },
                ParamDef {
                    name: "placeholder",
                    param_type: ParamType::String,
                    default: "",
                    description: "Placeholder text",
                },
                ParamDef {
                    name: "password",
                    param_type: ParamType::Bool,
                    default: "false",
                    description: "Password mode (hide input)",
                },
            ],
        },
        // Select
        ComponentDef {
            name: "select",
            description: "Select from a list of options",
            category: "Interactive",
            params: vec![
                ParamDef {
                    name: "prompt",
                    param_type: ParamType::String,
                    default: "Choose an option:",
                    description: "Prompt question",
                },
                ParamDef {
                    name: "options",
                    param_type: ParamType::Data,
                    default: "Option A,Option B,Option C",
                    description: "Options to choose from",
                },
                ParamDef {
                    name: "multi",
                    param_type: ParamType::Bool,
                    default: "false",
                    description: "Enable multi-select",
                },
            ],
        },
        // Confirm
        ComponentDef {
            name: "confirm",
            description: "Yes/No confirmation prompt",
            category: "Interactive",
            params: vec![
                ParamDef {
                    name: "prompt",
                    param_type: ParamType::String,
                    default: "Are you sure?",
                    description: "Confirmation question",
                },
                ParamDef {
                    name: "default",
                    param_type: ParamType::Enum(vec!["yes", "no"]),
                    default: "yes",
                    description: "Default answer",
                },
                ParamDef {
                    name: "style",
                    param_type: ParamType::Enum(vec!["normal", "danger"]),
                    default: "normal",
                    description: "Prompt style",
                },
            ],
        },
        // Filter
        ComponentDef {
            name: "filter",
            description: "Fuzzy filter items (like fzf)",
            category: "Interactive",
            params: vec![
                ParamDef {
                    name: "prompt",
                    param_type: ParamType::String,
                    default: "Filter:",
                    description: "Custom prompt text",
                },
                ParamDef {
                    name: "multi",
                    param_type: ParamType::Bool,
                    default: "false",
                    description: "Enable multi-select mode",
                },
                ParamDef {
                    name: "height",
                    param_type: ParamType::Number { min: 5.0, max: 50.0 },
                    default: "10",
                    description: "Maximum list height",
                },
            ],
        },
        // Form
        ComponentDef {
            name: "form",
            description: "Multi-field interactive form",
            category: "Interactive",
            params: vec![
                ParamDef {
                    name: "fields",
                    param_type: ParamType::String,
                    default: "name:text:Name,email:text:Email,role:select:Role:Admin;User",
                    description: "Form fields (name:type:label[:options])",
                },
                ParamDef {
                    name: "output",
                    param_type: ParamType::Enum(vec!["json", "env", "csv"]),
                    default: "json",
                    description: "Output format",
                },
            ],
        },
        // Style Preview
        ComponentDef {
            name: "style",
            description: "Preview style presets",
            category: "Themes",
            params: vec![
                ParamDef {
                    name: "preset",
                    param_type: ParamType::Enum(vec!["default", "minimal", "corporate", "playful", "dark"]),
                    default: "default",
                    description: "Style preset to preview",
                },
            ],
        },
        // Palette
        ComponentDef {
            name: "palette",
            description: "View and manage color palettes",
            category: "Themes",
            params: vec![
                ParamDef {
                    name: "name",
                    param_type: ParamType::Enum(vec!["default", "ocean", "forest", "sunset", "neon"]),
                    default: "default",
                    description: "Palette name",
                },
            ],
        },
        // Theme
        ComponentDef {
            name: "theme",
            description: "Browse and preview theme presets",
            category: "Themes",
            params: vec![
                ParamDef {
                    name: "name",
                    param_type: ParamType::Enum(vec!["default", "dark", "light", "nord", "dracula"]),
                    default: "default",
                    description: "Theme name",
                },
            ],
        },
        // Export
        ComponentDef {
            name: "export",
            description: "Export terminal graphics to SVG",
            category: "Utils",
            params: vec![
                ParamDef {
                    name: "component",
                    param_type: ParamType::Enum(vec!["box", "progress", "bar-chart", "pie-chart"]),
                    default: "box",
                    description: "Component to export",
                },
                ParamDef {
                    name: "output",
                    param_type: ParamType::String,
                    default: "output.svg",
                    description: "Output file path",
                },
            ],
        },
        // Join
        ComponentDef {
            name: "join",
            description: "Join content horizontally or vertically",
            category: "Layout",
            params: vec![
                ParamDef {
                    name: "direction",
                    param_type: ParamType::Enum(vec!["horizontal", "vertical"]),
                    default: "horizontal",
                    description: "Join direction",
                },
                ParamDef {
                    name: "separator",
                    param_type: ParamType::String,
                    default: " ",
                    description: "Separator between items",
                },
            ],
        },
        // Columns
        ComponentDef {
            name: "columns",
            description: "Split content into columns",
            category: "Layout",
            params: vec![
                ParamDef {
                    name: "count",
                    param_type: ParamType::Number { min: 2.0, max: 10.0 },
                    default: "2",
                    description: "Number of columns",
                },
                ParamDef {
                    name: "separator",
                    param_type: ParamType::String,
                    default: "│",
                    description: "Column separator",
                },
            ],
        },
        // Stack
        ComponentDef {
            name: "stack",
            description: "Stack content vertically with alignment",
            category: "Layout",
            params: vec![
                ParamDef {
                    name: "align",
                    param_type: ParamType::Enum(vec!["left", "center", "right"]),
                    default: "left",
                    description: "Text alignment",
                },
                ParamDef {
                    name: "gap",
                    param_type: ParamType::Number { min: 0.0, max: 10.0 },
                    default: "0",
                    description: "Gap between items",
                },
            ],
        },
        // Watch
        ComponentDef {
            name: "watch",
            description: "Watch mode - repeatedly execute a command",
            category: "Utils",
            params: vec![
                ParamDef {
                    name: "command",
                    param_type: ParamType::String,
                    default: "date",
                    description: "Command to watch",
                },
                ParamDef {
                    name: "interval",
                    param_type: ParamType::Number { min: 0.5, max: 60.0 },
                    default: "2.0",
                    description: "Refresh interval (seconds)",
                },
            ],
        },
    ]
}

/// Get components grouped by category
#[allow(dead_code)]
pub fn get_components_by_category() -> Vec<(&'static str, Vec<ComponentDef>)> {
    let components = get_all_components();
    let mut categories: Vec<(&'static str, Vec<ComponentDef>)> = vec![
        ("Output", vec![]),
        ("Charts", vec![]),
        ("Data", vec![]),
        ("Interactive", vec![]),
        ("Animation", vec![]),
        ("Themes", vec![]),
        ("Layout", vec![]),
        ("Utils", vec![]),
    ];

    for component in components {
        for (cat_name, cat_components) in &mut categories {
            if *cat_name == component.category {
                cat_components.push(component.clone());
                break;
            }
        }
    }

    categories
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_all_components() {
        let components = get_all_components();
        assert!(!components.is_empty());
        assert!(components.iter().any(|c| c.name == "box"));
        assert!(components.iter().any(|c| c.name == "progress"));
    }

    #[test]
    fn test_generate_command() {
        let components = get_all_components();
        let box_component = components.iter().find(|c| c.name == "box").unwrap();

        let mut values = HashMap::new();
        values.insert("message".to_string(), "Test".to_string());
        values.insert("style".to_string(), "success".to_string());

        let cmd = box_component.generate_command(&values);
        assert!(cmd.contains("termgfx box"));
        assert!(cmd.contains("--style success"));
    }

    #[test]
    fn test_components_by_category() {
        let categories = get_components_by_category();
        assert_eq!(categories.len(), 8); // Output, Charts, Data, Interactive, Animation, Themes, Layout, Utils
        assert!(categories.iter().any(|(name, _)| *name == "Output"));
        assert!(categories.iter().any(|(name, _)| *name == "Animation"));
        assert!(categories.iter().any(|(name, _)| *name == "Themes"));
    }
}
