use crossterm::{
    cursor::{Hide, MoveTo, Show},
    event::{self, Event, KeyCode, KeyEvent, KeyModifiers},
    execute,
    style::{Color, Print, ResetColor, SetForegroundColor, Stylize},
    terminal::{self, Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen},
};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::{
    fs,
    io::{self, IsTerminal, Write},
};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum StepType {
    Input,
    Select,
    #[serde(rename = "multiselect")]
    MultiSelect,
    Confirm,
    Summary,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WizardStep {
    pub id: String,
    #[serde(rename = "type")]
    pub step_type: StepType,
    pub prompt: String,
    #[serde(default)]
    pub options: Vec<String>,
    #[serde(default)]
    pub placeholder: Option<String>,
    #[serde(default)]
    pub password: bool,
    #[serde(default)]
    pub validate: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WizardConfig {
    pub title: Option<String>,
    pub steps: Vec<WizardStep>,
}

pub struct Wizard {
    title: Option<String>,
    steps: Vec<WizardStep>,
    current_step: usize,
    values: HashMap<String, String>,
    can_go_back: bool,
}

impl Wizard {
    pub fn new(title: Option<String>, steps: Vec<WizardStep>) -> Self {
        Self {
            title,
            steps,
            current_step: 0,
            values: HashMap::new(),
            can_go_back: true,
        }
    }

    pub fn from_config_file(path: &str) -> io::Result<Self> {
        let content = fs::read_to_string(path)?;
        let config: WizardConfig = serde_json::from_str(&content)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
        Ok(Self::new(config.title, config.steps))
    }

    pub fn parse_inline_step(step_str: &str) -> io::Result<WizardStep> {
        // Format: "type:id:prompt[:options]"
        let parts: Vec<&str> = step_str.split(':').collect();
        if parts.len() < 3 {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                format!(
                    "Invalid step format: {}. Expected format: type:id:prompt[:options]",
                    step_str
                ),
            ));
        }

        let step_type = match parts[0].to_lowercase().as_str() {
            "input" => StepType::Input,
            "select" => StepType::Select,
            "multiselect" => StepType::MultiSelect,
            "confirm" => StepType::Confirm,
            "summary" => StepType::Summary,
            _ => {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidInput,
                    format!("Unknown step type: {}", parts[0]),
                ))
            }
        };

        let id = parts[1].to_string();
        let prompt = parts[2].to_string();
        let options = if parts.len() > 3 {
            parts[3].split(',').map(|s| s.trim().to_string()).collect()
        } else {
            vec![]
        };

        Ok(WizardStep {
            id,
            step_type,
            prompt,
            options,
            placeholder: None,
            password: false,
            validate: None,
        })
    }

    pub fn run(&mut self, output_format: &str) -> io::Result<String> {
        // Check for interactive terminal
        if !std::io::stdin().is_terminal() {
            return Err(io::Error::other(
                "Wizard requires an interactive terminal (TTY)",
            ));
        }

        let mut stdout = io::stdout();
        terminal::enable_raw_mode()?;
        execute!(stdout, EnterAlternateScreen, Hide)?;

        let result = self.run_wizard(&mut stdout);

        execute!(stdout, Show, LeaveAlternateScreen)?;
        terminal::disable_raw_mode()?;

        match result {
            Ok(_) => self.format_output(output_format),
            Err(e) => Err(e),
        }
    }

    fn run_wizard(&mut self, stdout: &mut io::Stdout) -> io::Result<()> {
        while self.current_step < self.steps.len() {
            let step = self.steps[self.current_step].clone();

            // Render the step
            self.render_step_header(stdout)?;

            // Handle the step based on type
            match step.step_type {
                StepType::Summary => {
                    self.render_summary(stdout)?;
                    // Wait for Enter to continue or Esc to go back
                    if !self.wait_for_confirmation(stdout)? {
                        if self.can_go_back && self.current_step > 0 {
                            self.current_step -= 1;
                            continue;
                        }
                    } else {
                        break;
                    }
                }
                _ => {
                    match self.handle_step_input(stdout, &step) {
                        Ok(Some(value)) => {
                            self.values.insert(step.id.clone(), value);
                            self.current_step += 1;
                        }
                        Ok(None) => {
                            // User pressed Back
                            if self.can_go_back && self.current_step > 0 {
                                self.current_step -= 1;
                            }
                        }
                        Err(e) => return Err(e),
                    }
                }
            }
        }

        Ok(())
    }

    fn render_step_header(&self, stdout: &mut io::Stdout) -> io::Result<()> {
        execute!(stdout, Clear(ClearType::All), MoveTo(0, 0))?;

        // Title
        if let Some(title) = &self.title {
            execute!(
                stdout,
                SetForegroundColor(Color::Cyan),
                Print(format!("🧙 {}\n\n", title).bold()),
                ResetColor
            )?;
        } else {
            execute!(
                stdout,
                SetForegroundColor(Color::Cyan),
                Print("🧙 Wizard\n\n".bold()),
                ResetColor
            )?;
        }

        // Progress indicator
        let progress_text = format!("Step {}/{}", self.current_step + 1, self.steps.len());
        execute!(
            stdout,
            SetForegroundColor(Color::DarkGrey),
            Print(progress_text),
            Print("\n\n"),
            ResetColor
        )?;

        stdout.flush()?;
        Ok(())
    }

    fn handle_step_input(
        &self,
        stdout: &mut io::Stdout,
        step: &WizardStep,
    ) -> io::Result<Option<String>> {
        match &step.step_type {
            StepType::Input => self.input_step(
                stdout,
                &step.prompt,
                step.password,
                step.placeholder.as_deref(),
            ),
            StepType::Select => {
                if step.options.is_empty() {
                    return Err(io::Error::new(
                        io::ErrorKind::InvalidInput,
                        "Select step requires options",
                    ));
                }
                self.select_step(stdout, &step.prompt, &step.options, false)
            }
            StepType::MultiSelect => {
                if step.options.is_empty() {
                    return Err(io::Error::new(
                        io::ErrorKind::InvalidInput,
                        "MultiSelect step requires options",
                    ));
                }
                self.select_step(stdout, &step.prompt, &step.options, true)
            }
            StepType::Confirm => {
                let result = self.confirm_step(stdout, &step.prompt)?;
                Ok(result.map(|b| if b { "true" } else { "false" }.to_string()))
            }
            StepType::Summary => Ok(Some(String::new())),
        }
    }

    fn input_step(
        &self,
        stdout: &mut io::Stdout,
        prompt: &str,
        password: bool,
        placeholder: Option<&str>,
    ) -> io::Result<Option<String>> {
        execute!(
            stdout,
            SetForegroundColor(Color::Cyan),
            Print(prompt),
            Print(": "),
            ResetColor,
            Show
        )?;

        if let Some(ph) = placeholder {
            execute!(
                stdout,
                SetForegroundColor(Color::DarkGrey),
                Print(ph),
                ResetColor
            )?;
            // Move cursor back
            for _ in 0..ph.len() {
                execute!(stdout, crossterm::cursor::MoveLeft(1))?;
            }
        }

        stdout.flush()?;

        let mut input = String::new();

        loop {
            if let Event::Key(KeyEvent {
                code, modifiers, ..
            }) = event::read()?
            {
                match code {
                    KeyCode::Enter => {
                        execute!(stdout, Print("\n\n"), Hide)?;
                        return Ok(Some(input));
                    }
                    KeyCode::Esc => {
                        if self.can_go_back && self.current_step > 0 {
                            execute!(stdout, Print("\n\n"), Hide)?;
                            return Ok(None);
                        }
                    }
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
                            // Redisplay placeholder if input is now empty
                            if input.is_empty() {
                                if let Some(ph) = placeholder {
                                    execute!(
                                        stdout,
                                        SetForegroundColor(Color::DarkGrey),
                                        Print(ph),
                                        ResetColor
                                    )?;
                                    for _ in 0..ph.len() {
                                        execute!(stdout, crossterm::cursor::MoveLeft(1))?;
                                    }
                                }
                            }
                        }
                    }
                    KeyCode::Char(c)
                        if modifiers == KeyModifiers::NONE || modifiers == KeyModifiers::SHIFT =>
                    {
                        // Clear placeholder on first char
                        if input.is_empty() && placeholder.is_some() {
                            execute!(stdout, Clear(ClearType::UntilNewLine))?;
                        }
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
    }

    fn select_step(
        &self,
        stdout: &mut io::Stdout,
        prompt: &str,
        options: &[String],
        multi: bool,
    ) -> io::Result<Option<String>> {
        execute!(
            stdout,
            SetForegroundColor(Color::Cyan),
            Print(prompt),
            Print("\n\n"),
            ResetColor
        )?;

        let mut selected_idx = 0;
        let mut selected_items = HashSet::new();

        loop {
            execute!(stdout, MoveTo(0, 4))?; // Account for header

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
                    "↑↓: Navigate • Space: Toggle • Enter: Next • Esc: Back"
                } else {
                    "↑↓: Navigate • Enter: Next • Esc: Back"
                }),
                ResetColor
            )?;
            stdout.flush()?;

            if let Event::Key(KeyEvent { code, .. }) = event::read()? {
                match code {
                    KeyCode::Up => {
                        selected_idx = selected_idx.saturating_sub(1);
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
                            return Ok(Some(result.join(",")));
                        } else {
                            return Ok(Some(options[selected_idx].clone()));
                        }
                    }
                    KeyCode::Esc => {
                        if self.can_go_back && self.current_step > 0 {
                            return Ok(None);
                        }
                    }
                    _ => {}
                }
            }
        }
    }

    fn confirm_step(&self, stdout: &mut io::Stdout, prompt: &str) -> io::Result<Option<bool>> {
        execute!(
            stdout,
            SetForegroundColor(Color::Cyan),
            Print(prompt),
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
                        if self.can_go_back && self.current_step > 0 {
                            execute!(stdout, Print("\n\n"), Hide)?;
                            return Ok(None);
                        } else {
                            continue;
                        }
                    }
                    _ => None,
                };

                if let Some(answer) = result {
                    execute!(
                        stdout,
                        SetForegroundColor(if answer { Color::Green } else { Color::Red }),
                        Print(if answer { "y" } else { "n" }),
                        Print("\n\n"),
                        ResetColor,
                        Hide
                    )?;
                    return Ok(Some(answer));
                }
            }
        }
    }

    fn render_summary(&self, stdout: &mut io::Stdout) -> io::Result<()> {
        execute!(
            stdout,
            SetForegroundColor(Color::Cyan),
            Print("📋 Summary\n\n".bold()),
            ResetColor
        )?;

        for step in self.steps.iter() {
            if matches!(step.step_type, StepType::Summary) {
                continue;
            }

            if let Some(value) = self.values.get(&step.id) {
                execute!(
                    stdout,
                    SetForegroundColor(Color::Green),
                    Print("✓ "),
                    ResetColor,
                    SetForegroundColor(Color::White),
                    Print(format!("{}: ", step.prompt).bold()),
                    ResetColor,
                    Print(if step.password {
                        "********".to_string()
                    } else {
                        value.clone()
                    }),
                    Print("\n")
                )?;
            }
        }

        execute!(
            stdout,
            Print("\n"),
            SetForegroundColor(Color::DarkGrey),
            Print("Enter: Confirm • Esc: Back"),
            ResetColor
        )?;
        stdout.flush()?;

        Ok(())
    }

    fn wait_for_confirmation(&self, _stdout: &mut io::Stdout) -> io::Result<bool> {
        loop {
            if let Event::Key(KeyEvent { code, .. }) = event::read()? {
                match code {
                    KeyCode::Enter => return Ok(true),
                    KeyCode::Esc => {
                        if self.can_go_back && self.current_step > 0 {
                            return Ok(false);
                        }
                    }
                    _ => {}
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
            _ => Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                format!("Unknown output format: {}", format),
            )),
        }
    }
}

pub fn render(
    step_args: Vec<String>,
    config: Option<String>,
    title: Option<String>,
    output_format: String,
) -> io::Result<()> {
    let mut wizard = if let Some(config_path) = config {
        Wizard::from_config_file(&config_path)?
    } else {
        let steps: Result<Vec<_>, _> = step_args
            .iter()
            .map(|s| Wizard::parse_inline_step(s))
            .collect();
        Wizard::new(title, steps?)
    };

    match wizard.run(&output_format) {
        Ok(output) => {
            println!("{}", output);
            Ok(())
        }
        Err(e) => Err(e),
    }
}
