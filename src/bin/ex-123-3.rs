use std::io;
use chrono::NaiveDate;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let date = NaiveDate::parse_from_str(&input.trim(), "%F").unwrap();
    println!("{}", date.format("%A"));
}