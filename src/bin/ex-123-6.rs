use std::io;

fn main() {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    let L: Vec<i64> = buf
        .trim()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    let v:i64 = buf.trim().parse().unwrap();

    if match L.binary_search(&v) { Ok(_) => true, _ => false,} {
        print!("yes");
    } else {
        print!("no");
    }
}
