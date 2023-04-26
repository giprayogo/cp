use std::io;
use eval::eval;

fn main() {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();

    let val = eval(&buf.trim()).unwrap();
    println!("{}", val);
}