use std::io;
use rand::prelude::*;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let n: usize = input.trim().parse().unwrap();

    let mut rng = rand::thread_rng();
    let mut v: Vec<u8> = (0..n).map(|_| rng.gen()).collect();
    v.sort();
    v.dedup();

    let vs: Vec<String> = v.into_iter().map(|x: u8| x.to_string()).collect();    
    println!("{}", vs.join(" "));
}