use chrono::{Datelike, NaiveDate};
use std::io;

fn main() {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    let n: usize = buf.trim().parse().unwrap();

    let mut dates = Vec::with_capacity(n);
    for _ in 0..n {
        let mut buf = String::new();
        io::stdin().read_line(&mut buf).unwrap();
        let tuple: Vec<&str> = buf.trim().splitn(3, ' ').collect();
        let year = tuple[0].parse().unwrap();
        let month = tuple[1].parse().unwrap();
        let day = tuple[2].parse().unwrap();
        let date = NaiveDate::from_ymd_opt(year, month, day);
        dates.push(date);
    }

    println!("By month");
    dates.sort_by_key(|k| k.unwrap().month());
    for date in dates.iter().flatten() {
        println!("{}", date);
    }

    println!("By date");
    dates.sort_by_key(|k| k.unwrap().day());
    for date in dates.iter().flatten() {
        println!("{}", date);
    }

    println!("By age");
    dates.sort();
    for date in dates.iter().flatten() {
        println!("{}", date);
    }
}
