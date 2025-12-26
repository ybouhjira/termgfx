use crossterm::{
    cursor::{Hide, MoveTo, Show},
    event::{self, Event, KeyCode, KeyEvent, KeyModifiers},
    execute,
    style::{Color, Print, ResetColor, SetForegroundColor, Stylize},
    terminal::{self, Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen},
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::{
    fs,
    io::{self, Write},
};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum FieldType {
    Text,
    Password,
    Select,
    Multiselect,
    Confirm,
    Number,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Field {
    pub name: String,
    #[serde(rename = "type")]
    pub field_type: FieldType,
    pub label: String,
    #[serde(default)]
    pub options: Vec<String>,
    #[serde(default)]
    pub value: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FormConfig {
    pub fields: Vec<Field>,
}

pub struct Form {
    fields: Vec<Field>,
    current_field: usize,
    values: HashMap<String, String>,
}

impl Form {
    pub fn new(fields: Vec<Field>) -> Self {
        Self {
            fields,
            current_field: 0,
            values: HashMap::new(),
        }
    }

    pub fn from_config_file(path: &str) -> io::Result<Self> {
        let content = fs::read_to_string(path)?;
        let config: FormConfig = serde_json::from_str(&content)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
        Ok(Self::new(config.fields))
    }

    pub fn parse_field(field_str: &str) -> io::Result<Field> {
        let parts: Vec<&str> = field_str.split(':').collect();
        if parts.len() < 3 {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                format!(
                    "Invalid field format: {}. Expected format: name:type:label[:options]",
                    field_str
                ),
            ));
        }

        let name = parts[0].to_string();
        let field_type = match parts[1].to_lowercase().as_str() {
            "text" => FieldType::Text,
            "password" => FieldType::Password,
            "select" => FieldType::Select,
            "multiselect" => FieldType::Multiselect,
            "confirm" => FieldType::Confirm,
            "number" => FieldType::Number,
            _ => {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidInput,
                    format!("Unknown field type: {}", parts[1]),
                ))
            }
        };
        let label = parts[2].to_string();
        let options = if parts.len() > 3 {
            parts[3].split(',').map(|s| s.trim().to_string()).collect()
        } else {
            vec![]
        };

        Ok(Field {
            name,
            field_type,
            label,
            options,
            value: String::new(),
        })
    }

    pub fn run(&mut self, output_format: &str) -> io::Result<String> {
        let mut stdout = io::stdout();
        terminal::enable_raw_mode()?;
        execute!(stdout, EnterAlternateScreen, Hide)?;

        let result = self.run_form(&mut stdout);

        execute!(stdout, Show, LeaveAlternateScreen)?;
        terminal::disable_raw_mode()?;

        match result {
            Ok(_) => self.format_output(output_format),
            Err(e) => Err(e),
        }
    }

    fn run_form(&mut self, stdout: &mut io::Stdout) -> io::Result<()> {
        loop {
            self.render(stdout)?;

            if let Event::Key(KeyEvent {
                code, modifiers, ..
            }) = event::read()?
            {
                match code {
                    KeyCode::Tab if modifiers.contains(KeyModifiers::SHIFT) => {
                        // Shift+Tab - previous field
                        if self.current_field > 0 {
                            self.current_field -= 1;
                        }
                    }
                    KeyCode::Tab => {
                        // Tab - next field
                        if self.current_field < self.fields.len() - 1 {
                            self.current_field += 1;
                        }
                    }
                    KeyCode::Enter => {
                        // Enter - handle current field
                        let current_field = &self.fields[self.current_field].clone();
                        let value = self.handle_field_input(stdout, current_field)?;
                        self.values.insert(current_field.name.clone(), value);

                        // Move to next field or finish
                        if self.current_field < self.fields.len() - 1 {
                            self.current_field += 1;
                        } else {
                            break;
                        }
                    }
                    KeyCode::Esc => {
                        return Err(io::Error::new(io::ErrorKind::Interrupted, "Cancelled"));
                    }
                    _ => {}
                }
            }
        }

        Ok(())
    }

    fn render(&self, stdout: &mut io::Stdout) -> io::Result<()> {
        execute!(stdout, Clear(ClearType::All), MoveTo(0, 0))?;

        // Title
        execute!(
            stdout,
            SetForegroundColor(Color::Cyan),
            Print("📋 Form\n\n".bold()),
            ResetColor
        )?;

        // Render fields
        for (idx, field) in self.fields.iter().enumerate() {
            let is_current = idx == self.current_field;
            let has_value = self.values.contains_key(&field.name);

            let indicator = if is_current { "❯" } else { " " };
            let status = if has_value { "✓" } else { " " };

            let field_display = if is_current {
                field.label.clone().bold().to_string()
            } else {
                field.label.clone()
            };

            let value_display = if let Some(value) = self.values.get(&field.name) {
                if matches!(field.field_type, FieldType::Password) {
                    "********".to_string()
                } else {
                    value.clone()
                }
            } else {
                String::new()
            };

            execute!(
                stdout,
                SetForegroundColor(if is_current {
                    Color::Green
                } else {
                    Color::Reset
                }),
                Print(format!("{} {} {}", indicator, status, field_display)),
                ResetColor
            )?;

            if !value_display.is_empty() {
                execute!(
                    stdout,
                    SetForegroundColor(Color::DarkGrey),
                    Print(format!(": {}", value_display)),
                    ResetColor
                )?;
            }

            execute!(stdout, Print("\n"))?;
        }

        // Help text
        execute!(
            stdout,
            Print("\n"),
            SetForegroundColor(Color::DarkGrey),
            Print("Tab/Shift+Tab: Navigate • Enter: Fill field • Esc: Cancel"),
            ResetColor
        )?;

        stdout.flush()?;
        Ok(())
    }

    fn handle_field_input(&self, stdout: &mut io::Stdout, field: &Field) -> io::Result<String> {
        execute!(stdout, Clear(ClearType::All), MoveTo(0, 0))?;

        match &field.field_type {
            FieldType::Text | FieldType::Password => {
                let is_password = matches!(field.field_type, FieldType::Password);
                self.input_field(stdout, &field.label, is_password)
            }
            FieldType::Number => {
                let value = self.input_field(stdout, &field.label, false)?;
                // Validate number
                value
                    .parse::<f64>()
                    .map(|_| value)
                    .map_err(|_| io::Error::new(io::ErrorKind::InvalidInput, "Invalid number"))
            }
            FieldType::Select => {
                if field.options.is_empty() {
                    return Err(io::Error::new(
                        io::ErrorKind::InvalidInput,
                        "Select field requires options",
                    ));
                }
                self.select_field(stdout, &field.label, &field.options, false)
            }
            FieldType::Multiselect => {
                if field.options.is_empty() {
                    return Err(io::Error::new(
                        io::ErrorKind::InvalidInput,
                        "Multiselect field requires options",
                    ));
                }
                self.select_field(stdout, &field.label, &field.options, true)
            }
            FieldType::Confirm => {
                let result = self.confirm_field(stdout, &field.label)?;
                Ok(if result { "true" } else { "false" }.to_string())
            }
        }
    }

    fn input_field(
        &self,
        stdout: &mut io::Stdout,
        label: &str,
        password: bool,
    ) -> io::Result<String> {
        execute!(
            stdout,
            SetForegroundColor(Color::Cyan),
            Print(label),
            Print(": "),
            ResetColor,
            Show
        )?;
        stdout.flush()?;

        let mut input = String::new();

        loop {
            if let Event::Key(KeyEvent {
                code, modifiers, ..
            }) = event::read()?
            {
                match code {
                    KeyCode::Enter => break,
                    KeyCode::Char('c') if modifiers.contains(KeyModifiers::CONTROL) => {
                        return Err(io::Error::new(io::ErrorKind::Interrupted, "Cancelled"));
                    }
                    KeyCode::Backspace => {
                        if !input.is_empty() {
                            input.pop();
                            execute!(
                                stdout,
                                crossterm::cursor::MoveLeft(1),
                                Clear(ClearType::UntilNewLine)
                            )?;
                        }
                    }
                    KeyCode::Char(c)
                        if modifiers == KeyModifiers::NONE || modifiers == KeyModifiers::SHIFT =>
                    {
                        input.push(c);
                        if password {
                            execute!(stdout, Print('*'))?;
                        } else {
                            execute!(stdout, Print(c))?;
                        }
                    }
                    _ => {}
                }
                stdout.flush()?;
            }
        }

        execute!(stdout, Print("\n"), Hide)?;
        Ok(input)
    }

    fn select_field(
        &self,
        stdout: &mut io::Stdout,
        label: &str,
        options: &[String],
        multi: bool,
    ) -> io::Result<String> {
        execute!(
            stdout,
            SetForegroundColor(Color::Cyan),
            Print(label),
            Print("\n\n"),
            ResetColor
        )?;

        let mut selected_idx = 0;
        let mut selected_items = std::collections::HashSet::new();

        loop {
            execute!(stdout, MoveTo(0, 2))?;

            for (idx, option) in options.iter().enumerate() {
                let is_current = idx == selected_idx;
                let is_selected = selected_items.contains(&idx);

                let checkbox = if multi {
                    if is_selected {
                        "[x]"
                    } else {
                        "[ ]"
                    }
                } else {
                    " "
                };

                let indicator = if is_current { "❯" } else { " " };

                execute!(
                    stdout,
                    SetForegroundColor(if is_current {
                        Color::Green
                    } else {
                        Color::Reset
                    }),
                    Print(format!("{} {} {}\n", indicator, checkbox, option)),
                    ResetColor
                )?;
            }

            execute!(
                stdout,
                Print("\n"),
                SetForegroundColor(Color::DarkGrey),
                Print(if multi {
                    "↑↓: Navigate • Space: Toggle • Enter: Confirm"
                } else {
                    "↑↓: Navigate • Enter: Select"
                }),
                ResetColor
            )?;
            stdout.flush()?;

            if let Event::Key(KeyEvent { code, .. }) = event::read()? {
                match code {
                    KeyCode::Up => {
                        if selected_idx > 0 {
                            selected_idx -= 1;
                        }
                    }
                    KeyCode::Down => {
                        if selected_idx < options.len() - 1 {
                            selected_idx += 1;
                        }
                    }
                    KeyCode::Char(' ') if multi => {
                        if selected_items.contains(&selected_idx) {
                            selected_items.remove(&selected_idx);
                        } else {
                            selected_items.insert(selected_idx);
                        }
                    }
                    KeyCode::Enter => {
                        if multi {
                            let mut result: Vec<_> =
                                selected_items.iter().map(|&i| options[i].clone()).collect();
                            result.sort_by_key(|item| {
                                options.iter().position(|x| x == item).unwrap()
                            });
                            return Ok(result.join(","));
                        } else {
                            return Ok(options[selected_idx].clone());
                        }
                    }
                    KeyCode::Esc => {
                        return Err(io::Error::new(io::ErrorKind::Interrupted, "Cancelled"));
                    }
                    _ => {}
                }
            }
        }
    }

    fn confirm_field(&self, stdout: &mut io::Stdout, label: &str) -> io::Result<bool> {
        execute!(
            stdout,
            SetForegroundColor(Color::Cyan),
            Print(label),
            Print(" [Y/n]: "),
            ResetColor,
            Show
        )?;
        stdout.flush()?;

        loop {
            if let Event::Key(KeyEvent { code, .. }) = event::read()? {
                let result = match code {
                    KeyCode::Char('y') | KeyCode::Char('Y') => Some(true),
                    KeyCode::Char('n') | KeyCode::Char('N') => Some(false),
                    KeyCode::Enter => Some(true),
                    KeyCode::Esc => {
                        return Err(io::Error::new(io::ErrorKind::Interrupted, "Cancelled"))
                    }
                    _ => None,
                };

                if let Some(answer) = result {
                    execute!(
                        stdout,
                        SetForegroundColor(if answer { Color::Green } else { Color::Red }),
                        Print(if answer { "y" } else { "n" }),
                        Print("\n"),
                        ResetColor,
                        Hide
                    )?;
                    return Ok(answer);
                }
            }
        }
    }

    fn format_output(&self, format: &str) -> io::Result<String> {
        match format.to_lowercase().as_str() {
            "json" => serde_json::to_string_pretty(&self.values)
                .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e)),
            "env" => {
                let mut output = String::new();
                for (key, value) in &self.values {
                    output.push_str(&format!("{}={}\n", key.to_uppercase(), value));
                }
                Ok(output.trim_end().to_string())
            }
            "csv" => {
                let mut output = String::new();
                for (key, value) in &self.values {
                    output.push_str(&format!("{},{}\n", key, value));
                }
                Ok(output.trim_end().to_string())
            }
            _ => Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                format!("Unknown output format: {}", format),
            )),
        }
    }
}

pub fn render(
    field_args: Vec<String>,
    config: Option<String>,
    output_format: String,
) -> io::Result<()> {
    let mut form = if let Some(config_path) = config {
        Form::from_config_file(&config_path)?
    } else {
        let fields: Result<Vec<_>, _> = field_args.iter().map(|s| Form::parse_field(s)).collect();
        Form::new(fields?)
    };

    match form.run(&output_format) {
        Ok(output) => {
            println!("{}", output);
            Ok(())
        }
        Err(e) => Err(e),
    }
}
