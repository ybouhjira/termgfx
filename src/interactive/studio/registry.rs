//! Component registry with metadata for all termgfx components

use std::collections::HashMap;

/// Parameter type for component configuration
#[derive(Debug, Clone)]
pub enum ParamType {
    String,
    Number { min: f64, max: f64 },
    Enum(Vec<&'static str>),
    Bool,
    Data, // For comma-separated data like sparklines
}

/// A single parameter definition
#[derive(Debug, Clone)]
pub struct ParamDef {
    pub name: &'static str,
    pub param_type: ParamType,
    pub default: &'static str,
    pub description: &'static str,
}

/// Component definition with all metadata
#[derive(Debug, Clone)]
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
                    param_type: ParamType::Enum(vec!["rounded", "single", "double", "thick", "ascii"]),
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
                    param_type: ParamType::Number { min: 0.0, max: 100.0 },
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
                    param_type: ParamType::Number { min: 0.0, max: 100.0 },
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
                    param_type: ParamType::Enum(vec!["dots", "line", "arc", "bouncing", "clock", "circle", "bounce", "moon"]),
                    default: "dots",
                    description: "Spinner animation style",
                },
                ParamDef {
                    name: "duration",
                    param_type: ParamType::Number { min: 1.0, max: 60.0 },
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
            params: vec![
                ParamDef {
                    name: "data",
                    param_type: ParamType::Data,
                    default: "1,4,2,8,5,7,3,9,6",
                    description: "Comma-separated numeric values",
                },
            ],
        },
        ComponentDef {
            name: "chart bar",
            description: "Horizontal bar charts",
            category: "Charts",
            params: vec![
                ParamDef {
                    name: "data",
                    param_type: ParamType::Data,
                    default: "Sales:100,Costs:60,Profit:40",
                    description: "Label:value pairs",
                },
            ],
        },
        ComponentDef {
            name: "chart pie",
            description: "ASCII pie charts",
            category: "Charts",
            params: vec![
                ParamDef {
                    name: "data",
                    param_type: ParamType::Data,
                    default: "A:40,B:30,C:20,D:10",
                    description: "Label:value pairs",
                },
            ],
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
            params: vec![
                ParamDef {
                    name: "structure",
                    param_type: ParamType::String,
                    default: "root>child1,child2>leaf1,leaf2",
                    description: "Tree structure (> for children, , for siblings)",
                },
            ],
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
    ]
}

/// Get components grouped by category
pub fn get_components_by_category() -> Vec<(&'static str, Vec<ComponentDef>)> {
    let components = get_all_components();
    let mut categories: Vec<(&'static str, Vec<ComponentDef>)> = vec![
        ("Output", vec![]),
        ("Charts", vec![]),
        ("Data", vec![]),
        ("Interactive", vec![]),
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
        assert_eq!(categories.len(), 4);
        assert!(categories.iter().any(|(name, _)| *name == "Output"));
    }
}
