use clap::{Parser, Subcommand};

mod animation;
mod charts;
mod image;
mod interactive;
mod output;
mod script;

#[derive(Parser)]
#[command(name = "termgfx")]
#[command(author = "Youssef Bouhjira")]
#[command(version = "0.4.0")]
#[command(about = "Beautiful terminal graphics - styled boxes, charts, images, and prompts")]
#[command(propagate_version = true)]
#[command(after_help = r#"
EXAMPLES:
  termgfx box "Hello World" --style success
  termgfx banner "Welcome" --gradient cyan-purple
  termgfx progress 75 --style gradient --animate
  termgfx chart bar --data "Sales:100,Costs:60,Profit:40"
  termgfx sparkline "1,4,2,8,5,7,3,9,6"
  termgfx table --headers "Name,Age" --rows "Alice,30|Bob,25"
  termgfx gauge 75 --label "CPU" --style semicircle
  termgfx tree "root>src,docs>main.rs,lib.rs"

QUICK REFERENCE:
  Output:   box, banner, notification
  Charts:   chart (bar/line/pie), sparkline, gauge, heatmap
  Data:     table, tree, diff, timeline
  Input:    input, select, confirm, file, filter, pager
  Animate:  spinner, progress, typewriter, animate
  Utils:    image, record, script, dashboard, demo

For command details: termgfx <command> --help
"#)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Display a styled box with message
    ///
    /// Example: termgfx box "Success!" --style success --border rounded
    #[command(
        after_help = "Styles: info, success, warning, danger, gradient\nBorders: single, double, rounded, thick, ascii"
    )]
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
        /// Animate the box drawing
        #[arg(short, long)]
        animate: bool,
        /// Total animation duration in ms (default: 500)
        #[arg(long, default_value = "500")]
        animation_time: u64,
        /// Show a demo of this command
        #[arg(long, help = "Show a demo of this command")]
        demo: bool,
    },
    /// Display a styled banner with gradient colors
    ///
    /// Example: termgfx banner "Welcome" --gradient cyan-purple
    #[command(after_help = "Gradients: cyan-purple, red-orange, green-cyan, pink-yellow")]
    Banner {
        /// The title text
        title: String,
        /// Gradient colors (e.g., "cyan-purple")
        #[arg(short, long)]
        gradient: Option<String>,
        /// Animate the banner drawing
        #[arg(short, long)]
        animate: bool,
        /// Total animation duration in ms (default: 500)
        #[arg(long, default_value = "500")]
        animation_time: u64,
        /// Show a demo of this command
        #[arg(long, help = "Show a demo of this command")]
        demo: bool,
    },
    ///
    /// Example: termgfx spinner "Loading..." --style dots --duration 3
    #[command(after_help = "Styles: dots, line, arc, bouncing, clock, circle, bounce, moon")]
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
    ///
    /// Example: termgfx progress 75 --style gradient --animate
    #[command(after_help = "Styles: gradient, modern, animated, blocks, classic, thin")]
    Progress {
        /// Progress percentage (0-100)
        percent: u8,
        /// Style: gradient, modern, animated, blocks, classic, thin
        #[arg(short, long, default_value = "gradient")]
        style: String,
        /// Start color for gradient (hex: #3fb950 or name: red, green, blue, cyan, magenta, yellow)
        #[arg(long)]
        from: Option<String>,
        /// End color for gradient (hex: #58a6ff or name: red, green, blue, cyan, magenta, yellow)
        #[arg(long)]
        to: Option<String>,
        /// Animate from 0 to percent
        #[arg(short, long)]
        animate: bool,
        /// Total animation duration in ms (default: 1000)
        #[arg(long, default_value = "1000")]
        duration: u64,
        /// Show a demo of this command
        #[arg(long, help = "Show a demo of this command")]
        demo: bool,
    },
    ///
    /// Example: termgfx chart bar --data "Sales:100,Costs:60,Profit:40"
    #[command(after_help = "Types: bar, line, pie")]
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
    /// Display a sparkline mini-chart
    ///
    /// Example: termgfx sparkline "1,4,2,8,5,7,3,9,6" --animate
    Sparkline {
        /// Comma-separated values
        data: String,
        /// Animate the sparkline building
        #[arg(short, long)]
        animate: bool,
        /// Total animation duration in ms (default: 500)
        #[arg(long, default_value = "500")]
        animation_time: u64,
        /// Show a demo of this command
        #[arg(long, help = "Show a demo of this command")]
        demo: bool,
    },
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
    ///
    /// Example: termgfx table --headers "Name,Age,City" --rows "Alice,30,NYC|Bob,25,LA"
    #[command(
        after_help = "Borders: single, double, rounded, none\nAlignment: left, center, right"
    )]
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
        /// Animate rows appearing one by one
        #[arg(short, long)]
        animate: bool,
        /// Total animation duration in ms (default: 500)
        #[arg(long, default_value = "500")]
        animation_time: u64,
        /// Show a demo of this command
        #[arg(long, help = "Show a demo of this command")]
        demo: bool,
    },
    ///
    /// Example: termgfx tree "root>src,docs>main.rs,lib.rs"
    Tree {
        /// Tree data (e.g., "root>child1,child2>grandchild")
        data: Option<String>,
        /// JSON file path
        #[arg(short, long)]
        path: Option<String>,
        /// Animate tree nodes expanding
        #[arg(short, long)]
        animate: bool,
        /// Total animation duration in ms (default: 500)
        #[arg(long, default_value = "500")]
        animation_time: u64,
    },
    /// Record, play, or export terminal sessions
    Record {
        #[command(subcommand)]
        record_command: RecordCommands,
    },

    /// Run animation sequences from script files
    Script {
        /// Script file path
        #[arg(short, long)]
        file: Option<String>,
        /// Inline script commands (semicolon-separated)
        #[arg(short, long)]
        inline: Option<String>,
    },
    /// Run animation effects
    Animate {
        /// Animation type: progress, typewriter, counter, chart-build, bars
        #[arg(short = 't', long)]
        effect_type: String,
        /// Text content (for typewriter)
        #[arg(long)]
        text: Option<String>,
        /// Data (for chart-build, bars)
        #[arg(short, long)]
        data: Option<String>,
        /// Duration in seconds
        #[arg(short = 'D', long, default_value = "2.0")]
        duration: f64,
        /// Speed (chars per second for typewriter)
        #[arg(long, default_value = "30.0")]
        speed: f64,
        /// From value (for counter)
        #[arg(long, default_value = "0")]
        from: i64,
        /// To value (for counter)
        #[arg(long, default_value = "100")]
        to: i64,
        /// Style (for progress)
        #[arg(short, long, default_value = "gradient")]
        style: String,
        /// Prefix (for counter)
        #[arg(long)]
        prefix: Option<String>,
        /// Suffix (for counter)
        #[arg(long)]
        suffix: Option<String>,
    },
    /// Run interactive demo showcase
    Demo {
        /// Demo section: boxes, charts, progress, animation, all
        #[arg(short, long)]
        section: Option<String>,
    },
    /// Display a horizontal timeline
    ///
    /// Example: termgfx timeline --events "Start,Middle,End" --style arrow
    #[command(after_help = "Styles: arrow, line, dots")]
    Timeline {
        /// Events: "Start,Middle,End" or "2024-01:Start,2024-06:Middle,2024-12:End"
        #[arg(short, long)]
        events: String,
        /// Style: arrow, line, dots
        #[arg(short, long, default_value = "arrow")]
        style: String,
        /// Color: red, green, blue, yellow, magenta, cyan, white
        #[arg(long)]
        color: Option<String>,
        /// Animate the timeline
        #[arg(short, long)]
        animate: bool,
        /// Render vertically
        #[arg(long)]
        vertical: bool,
    },
    /// Desktop + terminal alerts
    Notification {
        /// Notification message
        message: String,
        /// Notification title
        #[arg(short, long)]
        title: Option<String>,
        /// Style: info, success, warning, error
        #[arg(short, long, default_value = "info")]
        style: String,
        /// Play sound with desktop notification
        #[arg(long)]
        sound: bool,
        /// Show only terminal notification
        #[arg(long)]
        terminal_only: bool,
        /// Show only desktop notification
        #[arg(long)]
        desktop_only: bool,
    },
    /// Display a radial/dial gauge indicator
    ///
    /// Example: termgfx gauge 75 --label "CPU" --style semicircle --animate
    /// Example (watch): termgfx gauge 0 --watch 1s --command "get_cpu.sh"
    #[command(
        after_help = "Styles: semicircle, full, minimal\nWatch: --watch <interval> --command <cmd>"
    )]
    Gauge {
        /// Value to display
        value: f64,
        /// Minimum value for the gauge range
        #[arg(long, default_value = "0")]
        min: f64,
        /// Maximum value for the gauge range
        #[arg(long, default_value = "100")]
        max: f64,
        /// Label to display with the gauge
        #[arg(short, long)]
        label: Option<String>,
        /// Gauge style: semicircle, full, minimal
        #[arg(short, long, default_value = "semicircle")]
        style: String,
        /// Color: red, green, blue, yellow, cyan, magenta, white, grey
        #[arg(long)]
        color: Option<String>,
        /// Animate the gauge from 0 to value
        #[arg(short, long)]
        animate: bool,
        /// Watch mode: update interval (e.g., "1s", "500ms")
        #[arg(short, long)]
        watch: Option<String>,
        /// Command to execute for watch mode (must output a number)
        #[arg(long)]
        command: Option<String>,
        /// Show a demo of this command
        #[arg(long, help = "Show a demo of this command")]
        demo: bool,
    },
    Dashboard {
        /// Layout: "2x2" or "3x1"
        #[arg(short, long, default_value = "2x2")]
        layout: String,
        /// Dashboard title
        #[arg(short, long)]
        title: Option<String>,
        /// Panels: "box:Hello,progress:75,sparkline:1;2;3,gauge:50"
        #[arg(short, long)]
        panels: Option<String>,
        /// Config file path (JSON)
        #[arg(short, long)]
        config: Option<String>,
        /// Border style: single, double, rounded
        #[arg(long, default_value = "single")]
        border: String,
    },
    /// Display a 2D heatmap visualization
    ///
    /// Example: termgfx heatmap --data "1,2,3;4,5,6;7,8,9" --colors viridis
    #[command(after_help = "Colors: blue-red, green-red, viridis, magma")]
    Heatmap {
        /// 2D data: "1,2,3;4,5,6;7,8,9" (semicolon separates rows)
        #[arg(short, long)]
        data: Option<String>,
        /// CSV file path
        #[arg(short, long)]
        file: Option<String>,
        /// X-axis labels (comma-separated)
        #[arg(long)]
        x_labels: Option<String>,
        /// Y-axis labels (comma-separated)
        #[arg(long)]
        y_labels: Option<String>,
        /// Chart title
        #[arg(short, long)]
        title: Option<String>,
        /// Color scheme: blue-red, green-red, viridis, magma
        #[arg(long, default_value = "blue-red")]
        colors: String,
        /// Animate the heatmap rendering
        #[arg(short, long)]
        animate: bool,
    },
    /// Interactice file/directory picker
    ///
    /// Example: termgfx file --path /var --directory --ext rs,toml
    File {
        /// Initial path to start the picker
        #[arg(short, long)]
        path: Option<String>,
        /// Only allow selecting directories
        #[arg(short, long)]
        directory: bool,
        /// Comma-separated list of allowed file extensions (e.g., "txt,md")
        #[arg(short, long)]
        ext: Option<String>,
        /// Maximum height of the picker in terminal lines
        #[arg(long)]
        height: Option<usize>,
    },
    /// Fuzzy filter items from stdin (like fzf/gum filter)
    ///
    /// Example: ls | termgfx filter --prompt "Select file:"
    #[command(after_help = "Pipe items to filter: cat list.txt | termgfx filter")]
    Filter {
        /// Custom prompt text
        #[arg(short, long)]
        prompt: Option<String>,
        /// Enable multi-select mode (space to toggle)
        #[arg(short, long)]
        multi: bool,
        /// Maximum height of the list
        #[arg(long)]
        height: Option<usize>,
    },
    /// Scrollable pager for viewing content (like less)
    ///
    /// Example: cat file.txt | termgfx pager --line-numbers
    #[command(after_help = "Keys: ↑/↓ scroll, PgUp/PgDn page, g/G top/bottom, q quit")]
    Pager {
        /// Show line numbers
        #[arg(short, long)]
        line_numbers: bool,
        /// Title to display in header
        #[arg(short, long)]
        title: Option<String>,
    },
    /// Multi-field interactive form for collecting inputs
    ///
    /// Example: termgfx form --field "name:text:Your name" --field "role:select:Role:Admin,User"
    #[command(
        after_help = "Field types: text, password, select, multiselect, confirm, number\nOutput formats: json, env, csv"
    )]
    Form {
        /// Form fields in format "name:type:label[:options]"
        #[arg(short, long)]
        field: Vec<String>,
        /// JSON config file path
        #[arg(short, long)]
        config: Option<String>,
        /// Output format: json, env, csv
        #[arg(short, long, default_value = "json")]
        output: String,
    },
    /// Join content horizontally or vertically
    ///
    /// Example: termgfx join "Column A" "Column B" --gap 4
    #[command(after_help = "Alignment: left, center, right")]
    Join {
        /// Content pieces to join (optional if using stdin)
        inputs: Vec<String>,
        /// Read from stdin as additional input
        #[arg(long)]
        stdin: bool,
        /// Join vertically instead of horizontally
        #[arg(short, long)]
        vertical: bool,
        /// Gap between joined items (spaces/lines)
        #[arg(short, long, default_value = "2")]
        gap: usize,
        /// Alignment: left, center, right
        #[arg(short, long, default_value = "left")]
        align: String,
    },
    /// Split stdin content into columns
    ///
    /// Example: cat file.txt | termgfx columns --widths 20,30,20
    #[command(after_help = "Widths: comma-separated column widths in characters")]
    Columns {
        /// Column widths (comma-separated, e.g., "20,30,20")
        #[arg(short, long)]
        widths: String,
        /// Gap between columns (spaces)
        #[arg(short, long, default_value = "2")]
        gap: usize,
    },
    /// Stack content vertically with alignment
    ///
    /// Example: termgfx stack "Header" "Content" "Footer" --align center
    #[command(after_help = "Alignment: left, center, right")]
    Stack {
        /// Content pieces to stack (optional if using stdin)
        inputs: Vec<String>,
        /// Read from stdin as additional input
        #[arg(long)]
        stdin: bool,
        /// Alignment: left, center, right
        #[arg(short, long, default_value = "left")]
        align: String,
        /// Gap between stacked items (blank lines)
        #[arg(short, long, default_value = "1")]
        gap: usize,
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
        /// Animate line drawing point by point
        #[arg(short, long)]
        animate: bool,
        /// Total animation duration in ms (default: 500)
        #[arg(long, default_value = "500")]
        animation_time: u64,
    },
    /// Bar chart
    Bar {
        /// Data in format "Label:Value,Label:Value"
        #[arg(short, long)]
        data: String,
        /// Animate bars growing
        #[arg(short, long)]
        animate: bool,
        /// Show a demo of this command
        #[arg(long, help = "Show a demo of this command")]
        demo: bool,
    },
    /// Pie chart (ASCII)
    Pie {
        /// Data in format "Label:Value,Label:Value"
        #[arg(short, long)]
        data: String,
        /// Animate slices appearing one by one
        #[arg(short, long)]
        animate: bool,
        /// Total animation duration in ms (default: 500)
        #[arg(long, default_value = "500")]
        animation_time: u64,
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
    // Handle `--help <command>` or `-h <command>` as `help <command>`
    let args: Vec<String> = std::env::args().collect();
    if args.len() >= 3 && (args[1] == "--help" || args[1] == "-h") && !args[2].starts_with('-') {
        // Rewrite args to: termgfx <command> --help
        let new_args = vec![args[0].clone(), args[2].clone(), "--help".to_string()];
        Cli::parse_from(new_args);
        return; // parse_from will print help and exit
    }

    let cli = Cli::parse();

    match cli.command {
        Commands::Box {
            message,
            style,
            border,
            emoji,
            animate,
            animation_time,
            demo,
        } => {
            if demo {
                println!("Example: termgfx box \"Hello\" --style success");
                println!();
                // Run with demo values
                output::styled_box::render_animated(
                    "Hello World!",
                    "success",
                    "rounded",
                    None,
                    true,
                    500,
                );
                return;
            }
            output::styled_box::render_animated(
                &message,
                &style,
                &border,
                emoji.as_deref(),
                animate,
                animation_time,
            );
        }
        Commands::Banner {
            title,
            gradient,
            animate,
            animation_time,
            demo,
        } => {
            if demo {
                println!("Example: termgfx banner \"Welcome\" --gradient cyan-purple");
                println!();
                // Run with demo values
                output::banner::render_animated("Welcome", Some("cyan-purple"), true, 500);
                return;
            }
            output::banner::render_animated(&title, gradient.as_deref(), animate, animation_time);
        }
        Commands::Spinner {
            message,
            style,
            duration,
        } => {
            output::spinner::render(&message, &style, duration);
        }
        Commands::Progress {
            percent,
            style,
            from,
            to,
            animate,
            duration,
            demo,
        } => {
            if demo {
                println!("Example: termgfx progress 75 --style gradient --animate");
                println!();
                // Run with demo values
                output::progress::render_animated_progress(75, "gradient", None, None, 1000);
                return;
            }
            if animate {
                output::progress::render_animated_progress(
                    percent,
                    &style,
                    from.as_deref(),
                    to.as_deref(),
                    duration,
                );
            } else {
                output::progress::render(percent, &style, from.as_deref(), to.as_deref());
            }
        }
        Commands::Chart { chart_type } => {
            match chart_type {
                ChartCommands::Line {
                    data,
                    title,
                    animate,
                    animation_time,
                } => {
                    let line_chart = charts::line::LineChart::new(
                        &data,
                        title.as_deref(),
                        animate,
                        animation_time,
                    );
                    line_chart.render();
                }
                ChartCommands::Bar {
                    data,
                    animate,
                    demo,
                } => {
                    if demo {
                        println!(
                            "Example: termgfx chart bar --data \"Sales:100,Costs:60,Profit:40\""
                        );
                        println!();
                        // Run with demo values
                        charts::bar::render_animated("Sales:100,Costs:60,Profit:40", true);
                        return;
                    }
                    charts::bar::render_animated(&data, animate);
                }
                ChartCommands::Pie {
                    data,
                    animate,
                    animation_time,
                } => {
                    let pie_chart = charts::pie::PieChart::new(&data, animate, animation_time);
                    pie_chart.render();
                }
            }
        }
        Commands::Image { path, protocol } => {
            image::render(&path, &protocol);
        }
        Commands::Input {
            prompt,
            placeholder,
            password,
        } => {
            interactive::input::render(&prompt, placeholder.as_deref(), password);
        }
        Commands::Select {
            prompt,
            options,
            multi,
        } => {
            interactive::select::render(&prompt, &options, multi);
        }
        Commands::Confirm {
            prompt,
            default,
            style,
        } => {
            interactive::confirm::render(&prompt, &default, &style);
        }
        Commands::Sparkline {
            data,
            animate,
            animation_time,
            demo,
        } => {
            if demo {
                println!("Example: termgfx sparkline \"1,4,2,8,5,7,3,9,6\"");
                println!();
                // Run with demo values
                charts::sparkline::render_animated("1,4,2,8,5,7,3,9,6", true, 500);
                return;
            }
            charts::sparkline::render_animated(&data, animate, animation_time);
        }
        Commands::Diff {
            file1,
            file2,
            unified,
            context,
        } => {
            output::diff::render(&file1, &file2, unified, context);
        }
        Commands::Table {
            headers,
            rows,
            file,
            border,
            alignment,
            animate,
            animation_time,
            demo,
        } => {
            if demo {
                println!(
                    "Example: termgfx table --headers \"Name,Age\" --rows \"Alice,30|Bob,25\""
                );
                println!();
                // Run with demo values
                output::table::render_animated(
                    Some("Name,Age"),
                    Some("Alice,30|Bob,25"),
                    None,
                    "single",
                    "left",
                    true,
                    500,
                );
                return;
            }
            output::table::render_animated(
                headers.as_deref(),
                rows.as_deref(),
                file.as_deref(),
                &border,
                &alignment,
                animate,
                animation_time,
            );
        }
        Commands::Tree {
            data,
            path,
            animate,
            animation_time,
        } => {
            output::tree::render_animated(
                data.as_deref(),
                path.as_deref(),
                animate,
                animation_time,
            );
        }
        Commands::Record { record_command } => match record_command {
            RecordCommands::Start { output } => {
                output::record::start(&output);
            }
            RecordCommands::Play { input, speed } => {
                output::record::play(&input, speed);
            }
            RecordCommands::Export {
                input,
                format,
                output,
            } => {
                output::record::export(&input, &format, &output);
            }
        },

        Commands::Script { file, inline } => {
            script::run(file.as_deref(), inline.as_deref());
        }
        Commands::Animate {
            effect_type,
            text,
            data,
            duration,
            speed,
            from,
            to,
            style,
            prefix,
            suffix,
        } => {
            animation::effects::run(
                &effect_type,
                text.as_deref(),
                data.as_deref(),
                duration,
                speed,
                from,
                to,
                &style,
                prefix.as_deref(),
                suffix.as_deref(),
            );
        }
        Commands::Demo { section } => {
            animation::demo::run_demo(section.as_deref());
        }
        Commands::Timeline {
            events,
            style,
            color,
            animate,
            vertical,
        } => {
            let args = output::timeline::TimelineArgs {
                events,
                style,
                color,
                animate,
                vertical,
            };
            if let Err(e) = output::timeline::render_timeline(&args) {
                eprintln!("Error: {}", e);
                std::process::exit(1);
            }
        }
        Commands::Notification {
            message,
            title,
            style,
            sound,
            terminal_only,
            desktop_only,
        } => {
            output::notification::render(
                &message,
                title.as_deref(),
                &style,
                sound,
                terminal_only,
                desktop_only,
            );
        }
        Commands::Gauge {
            value,
            min,
            max,
            label,
            style,
            color,
            animate,
            demo,
        } => {
            if demo {
                println!("Example: termgfx gauge 75 --label \"CPU\" --style semicircle");
                println!();
                // Run with demo values
                output::gauge::render(75.0, 0.0, 100.0, Some("CPU"), "semicircle", None, true);
                return;
            }
            output::gauge::render(
                value,
                min,
                max,
                label.as_deref(),
                &style,
                color.as_deref(),
                animate,
            );
        }
        Commands::Dashboard {
            layout,
            title,
            panels,
            config,
            border,
        } => {
            output::dashboard::render(
                &layout,
                title.as_deref(),
                panels.as_deref(),
                config.as_deref(),
                &border,
            );
        }
        Commands::Heatmap {
            data,
            file,
            x_labels,
            y_labels,
            title,
            colors,
            animate,
        } => {
            output::heatmap::render(
                data.as_deref(),
                file.as_deref(),
                x_labels.as_deref(),
                y_labels.as_deref(),
                title.as_deref(),
                &colors,
                animate,
            );
        }
        Commands::File {
            path,
            directory,
            ext,
            height,
        } => match interactive::file::render(path, directory, ext, height) {
            Ok(selected_path) => {
                println!("{}", selected_path.display());
            }
            Err(e) => {
                eprintln!("Error: {}", e);
                std::process::exit(1);
            }
        },
        Commands::Filter {
            prompt,
            multi,
            height,
        } => {
            interactive::filter::render(prompt, multi, height);
        }
        Commands::Pager {
            line_numbers,
            title,
        } => {
            interactive::pager::render(line_numbers, title);
        }
        Commands::Form {
            field,
            config,
            output,
        } => {
            if field.is_empty() && config.is_none() {
                eprintln!("Error: Provide at least one --field or a --config file");
                std::process::exit(1);
            }
            if let Err(e) = interactive::form::render(field, config, output) {
                eprintln!("Error: {}", e);
                std::process::exit(1);
            }
        }
        Commands::Join {
            inputs,
            stdin,
            vertical,
            gap,
            align,
        } => {
            if let Err(e) = output::layout::handle_join(inputs, stdin, vertical, gap, &align) {
                eprintln!("Error: {}", e);
                std::process::exit(1);
            }
        }
        Commands::Columns { widths, gap } => {
            let widths_vec: Result<Vec<usize>, _> = widths
                .split(',')
                .map(|s| s.trim().parse::<usize>())
                .collect();

            match widths_vec {
                Ok(w) => {
                    if let Err(e) = output::layout::handle_columns(w, gap) {
                        eprintln!("Error: {}", e);
                        std::process::exit(1);
                    }
                }
                Err(_) => {
                    eprintln!("Error: Invalid widths format (use comma-separated numbers, e.g., '20,30,20')");
                    std::process::exit(1);
                }
            }
        }
        Commands::Stack {
            inputs,
            stdin,
            align,
            gap,
        } => {
            if let Err(e) = output::layout::handle_stack(inputs, stdin, &align, gap) {
                eprintln!("Error: {}", e);
                std::process::exit(1);
            }
        }
    }
}
