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
        /// Spinner style: dots, line, arc, bouncing, clock, circle, bounce, moon
        #[arg(short, long, default_value = "dots")]
        style: String,
        /// Duration in seconds (auto-stop after N seconds)
        #[arg(short, long)]
        duration: Option<u64>,
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

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Box { message, style, border, emoji } => {
            output::styled_box::render(&message, &style, &border, emoji.as_deref());
        }
        Commands::Banner { title, gradient } => {
            output::banner::render(&title, gradient.as_deref());
        }
        Commands::Spinner { message, style, duration } => {
            output::spinner::render(&message, &style, duration);
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
    }
}
