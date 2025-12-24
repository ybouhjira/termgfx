use std::io::{stdout, IsTerminal, Write};
use std::thread;
use std::time::Duration;

pub fn render(message: &str, speed: u64) {
    // If not a TTY (piped/captured), just print the full message
    if !stdout().is_terminal() {
        println!("{}", message);
        return;
    }

    let mut stdout = stdout();
    for ch in message.chars() {
        print!("{}", ch);
        stdout.flush().unwrap();
        thread::sleep(Duration::from_millis(speed));
    }
    println!();
}
