extern crate chrono;

use chrono::prelude::*;

fn main() {
    let time = Local::now();
    println!("{}", time.format("%Y-%m-%d %H:%M:%S").to_string());
}