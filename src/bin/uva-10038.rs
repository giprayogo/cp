use std::io;
use std::error;

fn main() -> Result<(), Box<dyn error::Error>> {
    let mut buf = String::new();
    let mut output_buf = vec![];
    let stdin = io::stdin();

    'a: loop {
        buf.clear();
        stdin.read_line(&mut buf)?;

        let num: Vec<i32> = buf
            .trim()
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();

        if num.len() < 2 {
            break;
        }

        let n = num[0];
        let mut check = vec![false; n as usize];

        let mut pi = num[1];
        // println!("{:?}", num);
        for i in num.iter().skip(1) {
            // println!("{i}, {pi}");
            let delta = (i - pi).abs();
            if !(delta < n) {
                output_buf.push("Not jolly");
                continue 'a;
            } 
            check[delta as usize] = true;
            pi = *i;
        }
        // println!("jolly");
        // output_buf.push("Jolly")
        for c in check.iter() {
            if *c == false {
                output_buf.push("Not jolly");
                continue 'a;
            }
        }
        output_buf.push("Jolly");
    }

    println!("{}", output_buf.join("\n"));

    Ok(())
}