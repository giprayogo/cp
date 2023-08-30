use std::io;

fn main() {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    let l: Vec<i64> = buf
        .trim()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    let v:i64 = buf.trim().parse().unwrap();

    if match l.binary_search(&v) { Ok(_) => true, _ => false,} {
        print!("yes");
    } else {
        print!("no");
    }
}
