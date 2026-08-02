use std::io;
use std::str::FromStr;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Input
    let mut buffer = String::new();
    let stdin = io::stdin();
    let mut deps = vec![];
    let mut loans = vec![];
    loop {
        buffer.clear();
        stdin.read_line(&mut buffer)?;
        let words: Vec<_> = buffer.split_whitespace().collect();

        match &words[..] {
            [month, dp, value, n_records] => {
                let months = i32::from_str(month)?;
                if months < 0 {
                    break; // program end
                }
                let months = u32::from_str(month)?;
                let dp = f32::from_str(dp)?;
                let value = f32::from_str(value)?;

                let n_records = usize::from_str(n_records)?;
                let mut v = vec![];
                for _ in 0..n_records {
                    let mut inner_buffer = String::new();
                    inner_buffer.clear();
                    stdin.read_line(&mut inner_buffer)?;
                    let words: Vec<&str> = inner_buffer.split_whitespace().collect();

                    match &words[..] {
                        [month, percent] => {
                            let month = u32::from_str(month)?;
                            // let percent = 1. - f32::from_str(percent)?;
                            let percent = f32::from_str(percent)?;
                            v.push((month, percent));
                        }
                        _ => {
                            println!("{:?}", words);
                            panic!("Invalid depreciation specification")
                        }
                    }
                }
                deps.push(v);

                // let owes = value - dp;
                // let pay_per_month = owes / months as f32;
                loans.push((months, dp, value));
            }
            _ => {
                println!("{:?}", words);
                panic!("Invalid loan input");
            }
        }
    }

    // Logic (stupid but works)
    for (loan, dep) in std::iter::zip(loans.iter(), deps.iter()) {
        // let (owes, months) = loan;
        let (months, dp, value) = loan;
        // println!("{:?}, {:?}", loan, dep);

        let mut owes = value - dp;
        let monthly = owes / *months as f32;
        let mut current_value = *value;
        let mut paid_in = 0;
        let mut paid = false;

        for (from, to) in std::iter::zip(
            dep.iter(),
            dep.iter()
                .skip(1)
                .chain([(*months, dep.last().unwrap().1)].iter()), // last month
        ) {
            // println!("{}-{}: {}", from.0, to.0, from.1);

            if !paid {
                for _ in from.0..to.0 {
                    paid_in += 1;
                    owes -= monthly;
                    current_value *= 1.0 - from.1;
                    if owes < current_value {
                        if paid_in > 1 {
                            println!("{} months", paid_in);
                        } else {
                            println!("{} month", paid_in);
                        }
                        paid = true;
                        break;
                    }
                }
            }
        }
    }

    Ok(())
}
