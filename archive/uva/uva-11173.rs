use std::error;
use std::io;

use itertools::Itertools;

fn main() -> Result<(), Box<dyn error::Error>> {
    let mut buf = String::new();
    let mut out_buf = vec![];
    let stdin = io::stdin();
    stdin.read_line(&mut buf)?;

    let n = buf.trim().parse()?;

    for _ in 0..n {
        buf.clear();
        stdin.read_line(&mut buf)?;
        let (_n, k) = match buf.split_whitespace().collect_vec()[..] {
            [n, k] => {
                let n: i32 = n.parse()?;
                let k: i32 = k.parse()?;
                (n, k)
            }
            _ => panic!("Invalid entry: {:?}", buf),
        };

        // Solution from: https://cp-algorithms.com/algebra/gray-code.html
        out_buf.push(format!("{}", k ^ (k >> 1)));
    }

    println!("{}", out_buf.join("\n"));


    Ok(())
}
