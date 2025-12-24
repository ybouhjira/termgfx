use clap::{Parser, Subcommand};

mod output;
mod charts;
mod image;
mod interactive;

#[derive(Parser)]
#[command(name = "termgfx")]
#[command(author = "Youssef Bouhjira")]
#[command(version = "0.1.0")]
#[command(about = "Beautiful terminal graphics - styled boxes, charts, images, and prompts", long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Display a styled box with message
    Box {
        /// The message to display
        message: String,
        /// Style: info, success, warning, danger, gradient
        #[arg(short, long, default_value = "info")]
        style: String,
        /// Border style: single, double, rounded, thick
        #[arg(short, long, default_value = "rounded")]
        border: String,
        /// Emoji to display
        #[arg(short, long)]
        emoji: Option<String>,
    },
    /// Display a styled banner
    Banner {
        /// The title text
        title: String,
        /// Gradient colors (e.g., "cyan-purple")
        #[arg(short, long)]
        gradient: Option<String>,
    },
    /// Show a loading spinner
    Spinner {
        /// Loading message
        message: String,
        /// Spinner style: dots, circle, bounce, moon
        #[arg(short, long, default_value = "dots")]
        style: String,
    },
    /// Display a progress bar
    Progress {
        /// Progress percentage (0-100)
        percent: u8,
        /// Style: gradient, animated, blocks
        #[arg(short, long, default_value = "gradient")]
        style: String,
    },
    /// Display a chart
    Chart {
        #[command(subcommand)]
        chart_type: ChartCommands,
    },
    /// Display an image in terminal
    Image {
        /// Path or URL to image
        path: String,
        /// Protocol: auto, kitty, sixel, halfblock
        #[arg(short, long, default_value = "auto")]
        protocol: String,
    },
    /// Prompt for text input
    Input {
        /// The prompt question
        prompt: String,
        /// Placeholder text
        #[arg(short = 'P', long)]
        placeholder: Option<String>,
        /// Password mode (hide input)
        #[arg(long)]
        password: bool,
    },
    /// Select from a list of options
    Select {
        /// The prompt question
        prompt: String,
        /// Options to choose from
        options: Vec<String>,
    },
    /// Multi-select from options
    Choose {
        /// The prompt question
        prompt: String,
        /// Options to choose from
        options: Vec<String>,
        /// Enable multi-select
        #[arg(long)]
        multi: bool,
    },
    /// Yes/No confirmation prompt
    Confirm {
        /// The confirmation question
        prompt: String,
        /// Default answer
        #[arg(short, long, default_value = "yes")]
        default: String,
        /// Style: normal, danger
        #[arg(short = 'S', long, default_value = "normal")]
        style: String,
    },
    /// Display a sparkline
    Sparkline {
        /// Comma-separated values
        data: String,
    },
    /// Show file differences side-by-side or unified
    Diff {
        /// First file path
        file1: String,
        /// Second file path
        file2: String,
        /// Use unified diff format
        #[arg(long)]
        unified: bool,
        /// Context lines for unified format
        #[arg(long)]
        context: Option<usize>,
    },
    /// Display a formatted table from data
    Table {
        /// CSV headers (comma-separated)
        #[arg(long)]
        headers: Option<String>,
        /// Row data (pipe-separated rows, comma-separated columns)
        #[arg(long)]
        rows: Option<String>,
        /// CSV file path
        #[arg(short, long)]
        file: Option<String>,
        /// Border style: single, double, rounded, none
        #[arg(long, default_value = "single")]
        border: String,
        /// Column alignment: left, center, right
        #[arg(long, default_value = "left")]
        alignment: String,
    },
    /// Display a tree structure
    Tree {
        /// Tree data (e.g., "root>child1,child2>grandchild")
        data: Option<String>,
        /// JSON file path
        #[arg(short, long)]
        path: Option<String>,
    },
    /// Record, play, or export terminal sessions
    Record {
        #[command(subcommand)]
        record_command: RecordCommands,
    },
    /// Typewriter effect animation
    Typewriter {
        /// Message to animate
        message: String,
        /// Speed in milliseconds per character
        #[arg(short, long, default_value = "50")]
        speed: u64,
    },
}

#[derive(Subcommand)]
enum ChartCommands {
    /// Line chart
    Line {
        /// Comma-separated values
        #[arg(short, long)]
        data: String,
        /// Chart title
        #[arg(short, long)]
        title: Option<String>,
    },
    /// Bar chart
    Bar {
        /// Data in format "Label:Value,Label:Value"
        #[arg(short, long)]
        data: String,
    },
    /// Pie chart (ASCII)
    Pie {
        /// Data in format "Label:Value,Label:Value"
        #[arg(short, long)]
        data: String,
    },
}

#[derive(Subcommand)]
enum RecordCommands {
    /// Start recording terminal session
    Start {
        /// Output file path
        output: String,
    },
    /// Play terminal recording
    Play {
        /// Recording file path
        input: String,
        /// Playback speed multiplier
        #[arg(short, long, default_value = "1.0")]
        speed: f64,
    },
    /// Export recording to other formats
    Export {
        /// Input recording file
        input: String,
        /// Output format: gif, json
        #[arg(short, long)]
        format: String,
        /// Output file path
        output: String,
    },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Box { message, style, border, emoji } => {
            output::styled_box::render(&message, &style, &border, emoji.as_deref());
        }
        Commands::Banner { title, gradient } => {
            output::banner::render(&title, gradient.as_deref());
        }
        Commands::Spinner { message, style } => {
            output::spinner::render(&message, &style);
        }
        Commands::Progress { percent, style } => {
            output::progress::render(percent, &style);
        }
        Commands::Chart { chart_type } => {
            match chart_type {
                ChartCommands::Line { data, title } => {
                    charts::line::render(&data, title.as_deref());
                }
                ChartCommands::Bar { data } => {
                    charts::bar::render(&data);
                }
                ChartCommands::Pie { data } => {
                    charts::pie::render(&data);
                }
            }
        }
        Commands::Image { path, protocol } => {
            image::render(&path, &protocol);
        }
        Commands::Input { prompt, placeholder, password } => {
            interactive::input::render(&prompt, placeholder.as_deref(), password);
        }
        Commands::Select { prompt, options } => {
            interactive::select::render(&prompt, &options);
        }
        Commands::Choose { prompt, options, multi } => {
            interactive::choose::render(&prompt, &options, multi);
        }
        Commands::Confirm { prompt, default, style } => {
            interactive::confirm::render(&prompt, &default, &style);
        }
        Commands::Sparkline { data } => {
            charts::sparkline::render(&data);
        }
        Commands::Diff { file1, file2, unified, context } => {
            output::diff::render(&file1, &file2, unified, context);
        }
        Commands::Table { headers, rows, file, border, alignment } => {
            output::table::render(
                headers.as_deref(),
                rows.as_deref(),
                file.as_deref(),
                &border,
                &alignment,
            );
        }
        Commands::Tree { data, path } => {
            output::tree::render(data.as_deref(), path.as_deref());
        }
        Commands::Record { record_command } => {
            match record_command {
                RecordCommands::Start { output } => {
                    output::record::start(&output);
                }
                RecordCommands::Play { input, speed } => {
                    output::record::play(&input, speed);
                }
                RecordCommands::Export { input, format, output } => {
                    output::record::export(&input, &format, &output);
                }
            }
        }
        Commands::Typewriter { message, speed } => {
            output::typewriter::render(&message, speed);
        }
    }
}
