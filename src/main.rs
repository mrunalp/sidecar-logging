use std::{thread, time};
use chrono::Utc;

fn main() {
    let one_second =  time::Duration::from_secs(1);
    loop {
        println!("{:?}: Hello stdout!", Utc::now());
        thread::sleep(one_second);
        eprintln!("{:?}: Hello stderr!", Utc::now());
        thread::sleep(one_second);
    }
}
