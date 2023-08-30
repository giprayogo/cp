use std::io;

fn main() {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    let t: usize = buf.trim().parse().unwrap();
    buf.clear();
    let mut outs = Vec::new();
    for _ in 0..t {
        io::stdin().read_line(&mut buf).unwrap();
        let ab: Vec<i64> = buf
            .trim()
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        let a = ab[0];
        let b = ab[1];
        if a < b {
            outs.push("<");
        } else if a > b {
            outs.push(">");
        } else {
            outs.push("=");
        }
        buf.clear();
    }
    for out in outs {
        println!("{}", out);
    }
}
