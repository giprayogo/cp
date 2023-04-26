use std::{fmt, io, str::FromStr};
use std::f64::consts::PI;

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
    let n = take_single();
    println!("{1:.0$}", n, PI);
}