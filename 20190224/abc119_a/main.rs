extern crate chrono;
use chrono::{Utc, Local, DateTime, Date};

fn main () {
    let input = lead_input();
    println!("input : {}", input);
}

fn lead_input() -> String {
    let mut result = String::new();
    std::io::stdin().read_line(&mut result).unwrap();
    result
}
