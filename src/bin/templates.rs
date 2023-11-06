use std::{fmt, io, str::FromStr};

#[allow(unused)]
fn take_single<T>() -> T
where
    T: FromStr,
    <T as FromStr>::Err: fmt::Debug,
{
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().parse().unwrap()
}

#[allow(unused)]
fn take_vector<T>() -> Vec<T> 
where
    T: FromStr,
    <T as FromStr>::Err: fmt::Debug,
{
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect()
}

fn main() {
    // let number1: i64 = take_single();
    // let number2: i64 = take_single();
    // println!("{}", number1 * number2);
    // let vec: Vec<i64> = take_vector();
    // let mut sum = 0;
    // for i in 0..vec.len() {
    //     sum += vec[i];
    // }
    // println!("{}", sum);
    println!("OK!");
}
