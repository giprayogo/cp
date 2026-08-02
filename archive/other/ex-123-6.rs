use std::io;

fn main() {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    let l: Vec<i64> = buf
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    let v:i64 = buf.trim().parse().unwrap();

    if matches!(l.binary_search(&v), Ok(_)) {
        print!("yes");
    } else {
        print!("no");
    }
}
