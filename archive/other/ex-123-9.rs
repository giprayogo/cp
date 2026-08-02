use std::io;
use radix_fmt::radix;

fn main() {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();

    let vec: Vec<&str> = buf.split_whitespace().collect();
    if vec.len() != 3 {
        eprintln!("Usage: ex-123-9 \\n NUMBER IN_RADIX OUT_RADIX");
        std::process::exit(1);
    }
    let num_str = vec[0];
    let in_radix = vec[1].parse().unwrap();
    let out_radix = vec[2].parse().unwrap();
    let num = i64::from_str_radix(num_str, in_radix).unwrap();
    println!("{}", radix(num, out_radix));
}
