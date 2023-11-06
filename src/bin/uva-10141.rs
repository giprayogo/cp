use std::error;
use std::io;

fn main() -> Result<(), Box<dyn error::Error>> {
    let mut buf = String::new();
    let stdin = io::stdin();

    let mut i = 0;
    loop {
        i += 1;
        buf.clear();
        stdin.read_line(&mut buf)?;
        let (n, p) = match buf.split_whitespace().collect::<Vec<_>>()[..] {
            [n, p] => {
                let n: u32 = n.parse()?;
                let p: u32 = p.parse()?;
                if n == 0 || p == 0 {
                    break;
                }
                (n, p)
            }
            _ => panic!("Invalid buffer for n/p: {}", buf),
        };

        let mut rfp = Vec::new();
        for _ in 0..n {
            buf.clear();
            stdin.read_line(&mut buf)?;
            rfp.push(buf.trim().to_string());
        }

        let mut max_r = 0;
        let mut min_d = f64::MAX;
        let mut choice = String::new();
        for _ in 0..p {
            // TODO: Macro or dedicated function for this
            buf.clear();
            stdin.read_line(&mut buf)?;
            let name = buf.trim().to_string();

            buf.clear();
            stdin.read_line(&mut buf)?;
            let (d, r) = match buf.split_whitespace().collect::<Vec<_>>()[..] {
                [d, r] => {
                    let d: f64 = d.parse()?;
                    let r: u32 = r.parse()?;
                    (d, r)
                }
                _ => panic!("Invalid buffer for d/r: {}", buf),
            };

            // Useless: just skip
            for _ in 0..r {
                stdin.read_line(&mut buf)?;
            }

            if r > max_r {
                max_r = r;
                min_d = d;
                choice = name;
            } else if r == max_r && d < min_d {
                min_d = d;
                choice = name;
            }
        }

        println!("RFP #{}", i);
        println!("{}\n", choice);
    }

    Ok(())
}
