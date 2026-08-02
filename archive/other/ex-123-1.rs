use std::{fmt, io, str::FromStr};

fn take_single<T>() -> T
where
    T: FromStr,
    <T as FromStr>::Err: fmt::Debug,
{
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().parse().unwrap()
}

fn main() {
    let x: f64 = take_single();
    println!("{:7.3}", x)
}