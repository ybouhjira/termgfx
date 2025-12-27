use crossterm::{
    cursor::{Hide, MoveTo, MoveToColumn, Show},
    terminal::{Clear, ClearType},
    ExecutableCommand,
};
use std::io::{stdout, Write};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::{Duration, Instant};

/// Core animation engine that handles frame timing and terminal control
#[allow(dead_code)]
pub struct Animator {
    running: Arc<AtomicBool>,
    frame_delay: Duration,
}

#[allow(dead_code)]
impl Animator {
    pub fn new(frame_delay_ms: u64) -> Self {
        let running = Arc::new(AtomicBool::new(true));
        let r = running.clone();

        // Set up Ctrl+C handler
        let _ = ctrlc::set_handler(move || {
            r.store(false, Ordering::SeqCst);
        });

        Self {
            running,
            frame_delay: Duration::from_millis(frame_delay_ms),
        }
    }

    /// Run animation loop for a fixed duration
    /// render_fn takes (frame_index, progress 0.0-1.0) and returns content to display
    pub fn run_timed<F>(&self, duration_secs: f64, mut render_fn: F)
    where
        F: FnMut(usize, f64) -> String,
    {
        let mut stdout = stdout();
        stdout.execute(Hide).unwrap();

        let start = Instant::now();
        let duration = Duration::from_secs_f64(duration_secs);
        let mut frame = 0;

        while self.running.load(Ordering::SeqCst) {
            let elapsed = start.elapsed();
            if elapsed >= duration {
                break;
            }

            let progress = elapsed.as_secs_f64() / duration_secs;

            // Clear current line and render
            stdout.execute(MoveToColumn(0)).unwrap();
            stdout.execute(Clear(ClearType::CurrentLine)).unwrap();

            let content = render_fn(frame, progress);
            print!("{}", content);
            stdout.flush().unwrap();

            frame += 1;
            thread::sleep(self.frame_delay);
        }

        // Final frame at 100%
        stdout.execute(MoveToColumn(0)).unwrap();
        stdout.execute(Clear(ClearType::CurrentLine)).unwrap();
        let final_content = render_fn(frame, 1.0);
        println!("{}", final_content);

        stdout.execute(Show).unwrap();
        stdout.flush().unwrap();
    }

    /// Run animation that updates multiple lines
    pub fn run_multiline<F>(&self, duration_secs: f64, lines: usize, mut render_fn: F)
    where
        F: FnMut(usize, f64) -> Vec<String>,
    {
        let mut stdout = stdout();
        stdout.execute(Hide).unwrap();

        // Print initial empty lines
        for _ in 0..lines {
            println!();
        }

        let start = Instant::now();
        let duration = Duration::from_secs_f64(duration_secs);
        let mut frame = 0;

        while self.running.load(Ordering::SeqCst) {
            let elapsed = start.elapsed();
            if elapsed >= duration {
                break;
            }

            let progress = elapsed.as_secs_f64() / duration_secs;
            let contents = render_fn(frame, progress);

            // Move up and redraw all lines
            for (i, content) in contents.iter().enumerate() {
                let line_num = lines - contents.len() + i;
                stdout
                    .execute(MoveTo(
                        0,
                        (line_num as u16).saturating_sub(lines as u16 - 1),
                    ))
                    .unwrap();
                stdout.execute(Clear(ClearType::CurrentLine)).unwrap();
                print!("{}", content);
            }
            stdout.flush().unwrap();

            frame += 1;
            thread::sleep(self.frame_delay);
        }

        stdout.execute(Show).unwrap();
        stdout.flush().unwrap();
    }

    /// Check if animation is still running (not cancelled)
    pub fn is_running(&self) -> bool {
        self.running.load(Ordering::SeqCst)
    }
}

impl Default for Animator {
    fn default() -> Self {
        Self::new(50) // 50ms = 20fps for smooth animations
    }
}
