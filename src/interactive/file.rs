use crossterm::{
    cursor::{Hide, MoveTo, Show},
    event::{self, Event, KeyCode, KeyEvent, KeyModifiers},
    execute,
    style::{Color, Print, ResetColor, SetForegroundColor, Stylize},
};
use crossterm::terminal::{self, Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen};
use std::{borrow::Cow,
    collections::HashSet,
    fs,
    io::{self, Write},
    path::{Path, PathBuf}
};

// --- Constants and Icons ---
const SELECTED_COLOR: Color = Color::Cyan;
const DIRECTORY_COLOR: Color = Color::Blue;
const EXECUTABLE_COLOR: Color = Color::Green;
const FILE_COLOR: Color = Color::Reset;
const FILTER_COLOR: Color = Color::Yellow;
const ERROR_COLOR: Color = Color::Red;

const ICON_DIRECTORY: &str = "📁";
const ICON_FILE: &str = "📄";
const ICON_SELECTED: &str = "❯";
const ICON_UNSELECTED: &str = " ";

// --- FilePicker Struct ---
pub struct FilePicker {
    current_path: PathBuf,
    items: Vec<DirEntry>,
    selected_index: usize,
    filter: String,
    only_dirs: bool,
    allowed_extensions: Option<HashSet<String>>,
    height: Option<usize>,
    error_message: Option<String>,
}

#[derive(Debug, Clone)]
struct DirEntry {
    path: PathBuf,
    is_dir: bool,
    is_executable: bool,
}

impl DirEntry {
    fn new(path: PathBuf) -> Self {
        let is_dir = path.is_dir();
        let is_executable = if !is_dir {
            #[cfg(unix)]
            {
                use std::os::unix::fs::PermissionsExt;
                fs::metadata(&path)
                    .map(|meta| meta.permissions().mode() & 0o111 != 0)
                    .unwrap_or(false)
            }
            #[cfg(windows)]
            {
                // Windows doesn't have a direct executable bit like Unix.
                // For simplicity, we might consider .exe, .bat, .cmd etc.
                // or just return false. Returning false for now.
                false
            }
            #[cfg(not(any(unix, windows)))]
            {
                false
            }
        } else {
            false
        };
        DirEntry {
            path,
            is_dir,
            is_executable,
        }
    }

    fn file_name(&self) -> Cow<str> {
        self.path
            .file_name()
            .and_then(|s| s.to_str())
            .map_or_else(|| Cow::from("."), Cow::from)
    }
}

// --- Render Function (Public API) ---
pub fn render(
    path: Option<String>,
    only_dirs: bool,
    ext: Option<String>,
    height: Option<usize>,
) -> io::Result<PathBuf> {
    let initial_path = path
        .map(PathBuf::from)
        .unwrap_or_else(|| std::env::current_dir().unwrap_or_else(|_| PathBuf::from(".")));

    let allowed_extensions = ext.map(|s| {
        s.split(',')
            .map(|e| e.trim().to_lowercase())
            .filter(|e| !e.is_empty())
            .collect::<HashSet<String>>()
    });

    let mut picker = FilePicker::new(initial_path, only_dirs, allowed_extensions, height)?;
    picker.run()
}

// --- FilePicker Implementation ---
impl FilePicker {
    fn new(
        initial_path: PathBuf,
        only_dirs: bool,
        allowed_extensions: Option<HashSet<String>>,
        height: Option<usize>,
    ) -> io::Result<Self> {
        let mut picker = FilePicker {
            current_path: initial_path,
            items: Vec::new(),
            selected_index: 0,
            filter: String::new(),
            only_dirs,
            allowed_extensions,
            height,
            error_message: None,
        };
        picker.load_current_path_items()?;
        Ok(picker)
    }

    fn load_current_path_items(&mut self) -> io::Result<()> {
        self.items.clear();
        self.selected_index = 0;
        self.error_message = None;

        // Add parent directory ".."
        if self.current_path.parent().is_some() {
            self.items.push(DirEntry::new(self.current_path.join(".."))); // Placeholder path for ".."
        } else {
            // If at root, still show ".." but it will navigate to itself
            self.items.push(DirEntry::new(self.current_path.clone()));
        }

        let mut entries: Vec<DirEntry> = fs::read_dir(&self.current_path)?
            .filter_map(|entry| entry.ok())
            .map(|entry| DirEntry::new(entry.path()))
            .filter(|entry| {
                // Apply 'only_dirs' filter
                if self.only_dirs && !entry.is_dir {
                    return false;
                }
                // Apply extension filter
                if let Some(allowed_exts) = &self.allowed_extensions {
                    if !entry.is_dir {
                        if let Some(extension) = entry.path.extension().and_then(|s| s.to_str()) {
                            if !allowed_exts.contains(&extension.to_lowercase()) {
                                return false;
                            }
                        } else {
                            // If it's a file but has no extension, and we have filters, it's out
                            return false;
                        }
                    }
                }
                true
            })
            .collect();

        entries.sort_by_key(|e| (e.is_dir.to_string(), e.file_name().to_lowercase())); // Sort dirs first, then alphabetically

        self.items.extend(entries);

        // Filter items based on current filter string
        if !self.filter.is_empty() {
            let filter_lower = self.filter.to_lowercase();
            self.items.retain(|item| {
                item.file_name().to_lowercase().contains(&filter_lower)
            });
        }

        if self.items.is_empty() && !self.filter.is_empty() {
             self.error_message = Some(format!("No matches for \"{}\"", self.filter));
        } else if self.items.is_empty() {
            self.error_message = Some("Current directory is empty or inaccessible.".to_string());
        }

        Ok(())
    }

    fn run(&mut self) -> io::Result<PathBuf> {
        let mut stdout = io::stdout();

        terminal::enable_raw_mode()?;
        execute!(stdout, EnterAlternateScreen, Hide)?;

        let result = loop {
            self.draw(&mut stdout)?;

            if let Event::Key(key_event) = event::read()? {
                match key_event.code {
                    KeyCode::Up => {
                        if self.selected_index > 0 {
                            self.selected_index -= 1;
                        }
                    }
                    KeyCode::Down => {
                        if self.selected_index < self.items.len() - 1 {
                            self.selected_index += 1;
                        }
                    }
                    KeyCode::Enter => {
                        if !self.items.is_empty() {
                            let selected_entry = &self.items[self.selected_index];
                            if selected_entry.is_dir {
                                // Handle ".." navigation specifically
                                if selected_entry.file_name() == ".." {
                                    if let Some(parent) = self.current_path.parent() {
                                        self.current_path = parent.to_path_buf();
                                    } else {
                                        // Already at root, stay put
                                    }
                                } else {
                                    self.current_path = selected_entry.path.clone();
                                }
                                self.filter.clear(); // Clear filter on directory change
                                self.load_current_path_items()?;
                            } else {
                                // Selected a file
                                break Ok(selected_entry.path.clone());
                            }
                        }
                    }
                    KeyCode::Backspace => {
                        if !self.filter.is_empty() {
                            self.filter.pop();
                            self.load_current_path_items()?;
                        } else {
                            // If filter is empty, navigate up
                            if let Some(parent) = self.current_path.parent() {
                                self.current_path = parent.to_path_buf();
                                self.load_current_path_items()?;
                            }
                        }
                    }
                    KeyCode::Char(c) => {
                        if key_event.modifiers.contains(KeyModifiers::CONTROL) {
                            match c {
                                'c' => break Err(io::Error::new(io::ErrorKind::Interrupted, "Cancelled by user")),
                                _ => {}
                            }
                        } else {
                            self.filter.push(c);
                            self.load_current_path_items()?;
                        }
                    }
                    KeyCode::Esc => {
                        break Err(io::Error::new(io::ErrorKind::Interrupted, "Cancelled"))
                    }
                    _ => {}
                }
            }
        };

        execute!(stdout, Show, LeaveAlternateScreen)?;
        terminal::disable_raw_mode()?;

        result
    }

    fn draw(&mut self, stdout: &mut io::Stdout) -> io::Result<()> {
        let (cols, rows) = terminal::size()?;
        execute!(stdout, Clear(ClearType::All), MoveTo(0, 0))?;

        // Header: Current Path
        execute!(
            stdout,
            SetForegroundColor(Color::DarkGrey),
            Print("Path: "),
            SetForegroundColor(SELECTED_COLOR),
            Print(self.current_path.display().to_string().bold()),
            Print("\n"),
            ResetColor
        )?;

        // Filter input area
        execute!(
            stdout,
            SetForegroundColor(FILTER_COLOR),
            Print("Filter: "),
            SetForegroundColor(Color::White),
            Print(self.filter.clone()),
            Print("_"), // Indicate cursor
            Print("\n"),
            ResetColor
        )?;

        if let Some(msg) = &self.error_message {
            execute!(
                stdout,
                SetForegroundColor(ERROR_COLOR),
                Print(format!("Error: {}\n", msg)),
                ResetColor
            )?;
        } else {
            execute!(stdout, Print("\n"))?;
        }

        // Calculate visible items range
        let start_row_for_items = 3 + (if self.error_message.is_some() { 1 } else { 0 });
        let max_items_display = self.height.unwrap_or(rows as usize - start_row_for_items - 3); // 3 for path, filter, and help

        let mut start_index = 0;
        if self.selected_index >= max_items_display {
            start_index = self.selected_index - max_items_display + 1;
        }
        let end_index = (start_index + max_items_display).min(self.items.len());

        // Display items
        for (i, item) in self.items[start_index..end_index].iter().enumerate() {
            let actual_idx = start_index + i;
            let is_selected = actual_idx == self.selected_index;

            let icon = if item.is_dir { ICON_DIRECTORY } else { ICON_FILE };
            let selector = if is_selected { ICON_SELECTED } else { ICON_UNSELECTED };

            let item_name = item.file_name();

            let mut style = SetForegroundColor(FILE_COLOR);
            if item.is_dir {
                style = SetForegroundColor(DIRECTORY_COLOR);
            } else if item.is_executable {
                style = SetForegroundColor(EXECUTABLE_COLOR);
            }

            execute!(
                stdout,
                Print(format!("{} {} ", selector, icon)),
                style,
                Print(item_name),
                ResetColor,
                Print("\n")
            )?;
        }

        // Fill remaining lines if any
        for _ in end_index..start_index + max_items_display {
            execute!(stdout, Print("\n"))?;
        }

        // Help text
        execute!(
            stdout,
            Print("\n"),
            SetForegroundColor(Color::DarkGrey),
            Print("↑↓: Navigate • Enter: Select/Open • Backspace: Up/Delete Filter • Esc: Cancel • Type: Filter"),
            ResetColor
        )?;

        stdout.flush()?;
        Ok(())
    }
}
