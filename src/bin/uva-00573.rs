use std::error;
use std::io;

#[derive(Debug)]
struct HUDF {
    height: f64,
    up: f64,
    down: f64,
    fatigue_pct: f64,
}

fn main() -> Result<(), Box<dyn error::Error>> {
    let mut hudf_container = vec![];

    // User input
    let mut buf = String::new();
    let stdin = io::stdin();
    loop {
        stdin.read_line(&mut buf)?;
        let line: Vec<_> = buf.trim().split_whitespace().collect();
        match line[..] {
            [height, up, down, fatigue_pct] => {
                let height = height.parse()?;
                let up = up.parse()?;
                let down = down.parse()?;
                let fatigue_pct = fatigue_pct.parse()?;

                if height == 0. {
                    break;
                }
                hudf_container.push(HUDF {
                    height,
                    up,
                    down,
                    fatigue_pct,
                });
                buf.clear();
            }
            _ => panic!("Invalid input"),
        }
    }

    // App logic
    for hudf in hudf_container {
        let mut height = 0.;
        let mut day = 0;
        let mut up = hudf.up;
        let fatigue_drop = (hudf.fatigue_pct / 100.0) * up;
        // println!("---------------");
        // println!("{:?}", &hudf);
        // println!("---------------");
        loop {
            day += 1;
            // println!("  day {}-------", day);
            height += up;
            // println!("  up to height: {}", height);
            if height > hudf.height {
                println!("Success on day {}", day);
                break;
            }
            height -= hudf.down;
            // println!("  slip to height: {}", height);
            if height < 0. {
                println!("Failure on day {}", day);
                break;
            }
            // up *= (100. - hudf.fatigue_pct) / 100.;
            up -= fatigue_drop;
            // if up <= 0. {
            //     println!("Failure on day {}", day);
            //     break;
            // }
        }
        // if height >= 0. {
        //     println!("Success on day {}", day);
        // } else {
        //     println!("Failure on day {}", day);
        // }
    }

    Ok(())
}
