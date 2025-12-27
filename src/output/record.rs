use serde::{Deserialize, Serialize};
use std::fs;
use std::io::{self, Write};
use std::path::PathBuf;
use std::process::{Command, Stdio};
use std::thread;
use std::time::{Duration, Instant};

#[derive(Serialize, Deserialize, Debug)]
pub struct Recording {
    version: u8,
    width: u16,
    height: u16,
    events: Vec<Event>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Event {
    time: f64,
    event_type: String,
    data: String,
}

impl Event {
    fn new(time: f64, data: String) -> Self {
        Self {
            time,
            event_type: "o".to_string(),
            data,
        }
    }
}

pub fn start(output: &str) {
    let output_path = PathBuf::from(output);

    println!("🔴 Recording started...");
    println!("Output: {}", output_path.display());
    println!("\nPress Ctrl+C to stop recording\n");

    // Get terminal size
    let (width, height) = get_terminal_size();

    let mut recording = Recording {
        version: 1,
        width,
        height,
        events: Vec::new(),
    };

    let start_time = Instant::now();

    // Use script command to record terminal session
    #[cfg(target_os = "macos")]
    let mut child = Command::new("script")
        .arg("-q")
        .arg("/dev/null")
        .stdin(Stdio::inherit())
        .stdout(Stdio::piped())
        .stderr(Stdio::inherit())
        .spawn()
        .expect("Failed to start script command");

    #[cfg(target_os = "linux")]
    let mut child = Command::new("script")
        .arg("-q")
        .arg("-c")
        .arg("bash")
        .arg("/dev/null")
        .stdin(Stdio::inherit())
        .stdout(Stdio::piped())
        .stderr(Stdio::inherit())
        .spawn()
        .expect("Failed to start script command");

    if let Some(stdout) = child.stdout.take() {
        use std::io::BufRead;
        let reader = io::BufReader::new(stdout);

        for line in reader.lines() {
            match line {
                Ok(data) => {
                    let elapsed = start_time.elapsed().as_secs_f64();
                    print!("{}", data);
                    io::stdout().flush().unwrap();
                    recording.events.push(Event::new(elapsed, data + "\n"));
                }
                Err(_) => break,
            }
        }
    }

    child.wait().ok();

    // Save recording
    let json = serde_json::to_string_pretty(&recording).expect("Failed to serialize recording");
    fs::write(&output_path, json).expect("Failed to write recording file");

    println!("\n\n✅ Recording saved to {}", output_path.display());
    println!("Events captured: {}", recording.events.len());
}

pub fn play(input: &str, speed: f64) {
    let content = fs::read_to_string(input).expect("Failed to read recording file");
    let recording: Recording = serde_json::from_str(&content).expect("Failed to parse recording");

    println!("▶️  Playing recording: {}", input);
    println!("Speed: {}x", speed);
    println!(
        "Duration: {:.2}s",
        recording.events.last().map(|e| e.time).unwrap_or(0.0)
    );
    println!("\n{}", "=".repeat(recording.width as usize));

    thread::sleep(Duration::from_millis(500));

    let mut last_time = 0.0;

    for event in &recording.events {
        let wait_time = (event.time - last_time) / speed;
        if wait_time > 0.0 {
            thread::sleep(Duration::from_secs_f64(wait_time));
        }

        print!("{}", event.data);
        io::stdout().flush().unwrap();

        last_time = event.time;
    }

    println!("\n{}", "=".repeat(recording.width as usize));
    println!("✅ Playback complete");
}

pub fn export(input: &str, format: &str, output: &str) {
    let recording_path = PathBuf::from(input);

    if !recording_path.exists() {
        eprintln!("❌ Recording file not found: {}", input);
        std::process::exit(1);
    }

    match format {
        "gif" => {
            // Try to use external tools for GIF export
            if Command::new("agg").output().is_ok() {
                println!("🎬 Exporting to GIF using 'agg'...");
                let status = Command::new("agg").arg(input).arg(output).status();

                match status {
                    Ok(s) if s.success() => {
                        println!("✅ GIF exported to {}", output);
                    }
                    _ => {
                        eprintln!("❌ Failed to export GIF with agg");
                        std::process::exit(1);
                    }
                }
            } else if Command::new("vhs").output().is_ok() {
                println!("🎬 Exporting to GIF using 'vhs'...");
                println!("💡 Note: vhs requires a .tape file. Creating one...");

                let tape_content = format!("Output {}\nPlayback {}\nSleep 1s", output, input);

                let tape_file = input.replace(".recording", ".tape");
                fs::write(&tape_file, tape_content).expect("Failed to write tape file");

                let status = Command::new("vhs").arg(&tape_file).status();

                match status {
                    Ok(s) if s.success() => {
                        println!("✅ GIF exported to {}", output);
                        fs::remove_file(&tape_file).ok();
                    }
                    _ => {
                        eprintln!("❌ Failed to export GIF with vhs");
                        std::process::exit(1);
                    }
                }
            } else {
                eprintln!("❌ No GIF export tool found!");
                eprintln!("\n💡 Install one of these tools:");
                eprintln!("  • agg: cargo install agg");
                eprintln!("  • vhs: brew install vhs");
                std::process::exit(1);
            }
        }
        "json" => {
            // Just copy the recording as-is
            fs::copy(input, output).expect("Failed to copy recording");
            println!("✅ Recording copied to {}", output);
        }
        _ => {
            eprintln!("❌ Unsupported format: {}", format);
            eprintln!("Supported formats: gif, json");
            std::process::exit(1);
        }
    }
}

fn get_terminal_size() -> (u16, u16) {
    use std::os::fd::AsRawFd;

    unsafe {
        let mut size: libc::winsize = std::mem::zeroed();
        libc::ioctl(io::stdout().as_raw_fd(), libc::TIOCGWINSZ, &mut size);

        if size.ws_col > 0 && size.ws_row > 0 {
            (size.ws_col, size.ws_row)
        } else {
            (80, 24) // Default fallback
        }
    }
}
