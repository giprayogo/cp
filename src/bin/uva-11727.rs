use std::io;

fn main() {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    let t: usize = buf.trim().parse().unwrap();
    buf.clear();
    let mut outputs = Vec::new();
    for i in 0..t {
        io::stdin().read_line(&mut buf).unwrap();
        let mut case: Vec<i32> = buf
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        case.sort();
        outputs.push(format!("Case {}: {}", i+1, case[1]));
        buf.clear();
    }
    for output in outputs.iter() {
        println!("{}", output);
    }
}
