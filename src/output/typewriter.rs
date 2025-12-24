use std::io::{stdout, Write};
use std::thread;
use std::time::Duration;

pub fn render(message: &str, speed: u64) {
    let mut stdout = stdout();
    for ch in message.chars() {
        print!("{}", ch);
        stdout.flush().unwrap();
        thread::sleep(Duration::from_millis(speed));
    }
    println!();
}
