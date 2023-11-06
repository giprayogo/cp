use std::{io, process::exit};

fn main() {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    while !buf.trim().is_empty() {
        let ints: Vec<i64> = buf
            .split_whitespace()
            .map(|x| x.parse().expect("Can't parse int"))
            .collect();
        let count = ints[0] as usize;
        if ints.len() != count + 1 {
            eprintln!("Invalid input!");
            exit(1);
        }
        println!("{}", ints[1..].iter().sum::<i64>());
        buf.clear();
        io::stdin().read_line(&mut buf).unwrap();
    }
}
