use std::error;
use std::io;

use itertools::Itertools;

// Note: A linked-list solution is perhaps faster;
// revisit this problem later
// or a hash table
fn main() -> Result<(), Box<dyn error::Error>> {
    let mut buf = String::new();
    let stdin = io::stdin();

    let mut soldiers = vec![];
    let mut losses = vec![];
    // let mut out_buf = vec![];

    // input loop
    loop {
        buf.clear();
        stdin.read_line(&mut buf)?;
        let (s, b) = match buf.trim().split_whitespace().collect_vec()[..] {
            ["0", "0"] => break,
            [s, b] => {
                let s = s.parse()?;
                let b = b.parse()?;
                (s, b)
            }
            _ => panic!("Invalid input: {}", buf),
        };
        soldiers.push(s);

        let mut _b = vec![];
        for _ in 0..b {
            buf.clear();
            stdin.read_line(&mut buf)?;
            let (l, r) = match buf.trim().split_whitespace().collect_tuple() {
                Some((l, r)) => {
                    let l = l.parse::<i32>()? - 1;
                    let r = r.parse::<i32>()? - 1;
                    (l, r)
                }
                _ => panic!("Invalid input: {}", buf),
            };
            _b.push((l, r));
        }
        losses.push(_b);
    }

    // the logic
    for (s, _losses) in soldiers.iter().zip(losses.iter()) {
        // TODO: rewrite in hash table? anyway
        // let mut _s = (0..*s).collect_vec();
        let mut _s = vec![1; *s];
        for (l, r) in _losses {
            // println!("{}--h-{},{}----", s,l,r);
            // println!("<< {:?}", _s);
            for i in *l..*r + 1 {
                _s[i as usize] = 0;
            }
            // brute force
            // println!(">> {:?}", _s);
            let mut _l = None;
            for i in (0..*l as usize).rev() {
                if _s[i] == 1 {
                    _l = Some(i);
                    break;
                }
            }
            let _l = match _l {
                Some(v) => (v + 1).to_string(),
                None => "*".to_string(), 
            };

            let mut _r = None;
            for i in *r as usize..*s {
                if _s[i] == 1 {
                    _r = Some(i);
                    break;
                }
            }
            let _r = match _r {
                Some(v) => (v + 1).to_string(),
                None => "*".to_string(), 
            };
            println!("{} {}", _l, _r);
        }

        println!("-");
    }

    Ok(())
}
