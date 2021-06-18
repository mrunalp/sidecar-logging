use std::{thread, time};
use std::io;
use std::io::ErrorKind;
use std::io::Write;
use chrono::Utc;

fn main() {
    let one_second =  time::Duration::from_secs(1);
    let mut stdout = io::stdout();
    loop {
        if let Err(e) = writeln!(stdout, "Hello stdout!") {
            if e.kind() != ErrorKind::BrokenPipe {
                eprintln!("Broken pipe!");
            }
        }
        thread::sleep(one_second);
        eprintln!("{:?}: Hello stderr!", Utc::now());
        thread::sleep(one_second);
    }
}
