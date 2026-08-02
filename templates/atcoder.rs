use std::io::stdin;

use itertools::izip;

#[allow(unused)]
fn read_one<T>() -> T
where
    T: FromStr,
    <T as FromStr>::Err: fmt::Debug,
{
    let mut buf = String::new();
    stdin().read_line(&mut buf).unwrap();

    buf.trim().parse().expect(&format!("Invalid string: {buf}"))
}

#[allow(unused)]
fn read_many<T>() -> Vec<T>
where
    T: FromStr,
    <T as FromStr>::Err: fmt::Debug,
{
    let mut buf = String::new();
    stdin().read_line(&mut buf).unwrap();

    buf.trim()
        .split_whitespace()
        .map(|x| x.parse().expect(&format!("Invalid string: {x}")))
        .collect()
}

fn main() {
    // write here
}
