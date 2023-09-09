use core::panic;
use std::error;
use std::io;
// use std::iter;

use itertools::Itertools;

// use itertools::Itertools;

#[path = "../utils.rs"]
mod utils;

fn main() -> Result<(), Box<dyn error::Error>> {
    let mut buf = String::new();
    let stdin = io::stdin();
    let mut output_buf = Vec::new();

    loop {
        let row: Vec<_> = stdins!(buf, stdin).map(|x| x.parse().unwrap()).collect();
        let [a, b, c, x, y] = match row[..] {
            [0, 0, 0, 0, 0] => break,
            [a, b, c, x, y] => [a, b, c, x, y],
            _ => panic!("Invalid card row"),
        };

        let cess_cards = vec![a, b, c];
        let ce_cards = vec![x, y];
        let cess_perms: Vec<_> = cess_cards.iter().permutations(3).collect();
        let ce_perms: Vec<_> = ce_cards.iter().permutations(2).collect();

        let mut winning = 0;

        'all: for cess in &cess_perms {
            // print!("{:?}", cess);

            for ce in &ce_perms {
                let mut win = 0;

                for i in 0..2 {
                    // stupid but work
                    if ce[i] > cess[i] {
                        win += 1;
                    }
                }

                match win {
                    0 => {
                        winning = -1;
                        break 'all;
                    }
                    1 => {
                        let mut proposal = cess[2] + 1;
                        loop {
                            if proposal > 52 {
                                winning = -1;
                                break 'all;
                            }
                            if [a, b, c, x, y].contains(&proposal) {
                                proposal += 1;
                            } else {
                                if proposal > winning {
                                    winning = proposal;
                                }
                                break;
                            }
                        }
                    }
                    2 => {
                        let mut proposal = 1;
                        loop {
                            if proposal > 52 {
                                winning = -1;
                                break 'all;
                            }
                            if [a, b, c, x, y].contains(&proposal) {
                                proposal += 1;
                            } else {
                                if proposal > winning {
                                    winning = proposal;
                                }
                                break;
                            }
                        }
                    },
                    _ => panic!("Must have forgot to quit somewhere"),
                }
            }
        }

        output_buf.push(winning.to_string());

        // let mut shot_at_win = 0;
        // let mut abs_win = 0;
        // for ce in [x, y] {
        //     let mut winnable_count = 0;
        //     for cess in [a, b, c] {
        //         if ce > cess {
        //             winnable_count += 1;
        //         }
        //     }
        //     if winnable_count == 2 {
        //         shot_at_win += 1;
        //     } else if winnable_count == 3 {
        //         abs_win += 1;
        //     }
        // }

        // if shot_at_win < 2 && abs_win == 0 {
        //     output_buf.push((-1).to_string());
        // } else {
        //     if abs_win == 1 {
        //         output_buf.push("middle+1".to_string());
        //     } else {
        //         let max_cess = *[a, b, c].iter().max().unwrap();
        //         if max_cess < 52 {
        //             output_buf.push((max_cess + 1).to_string());
        //         } else {
        //             output_buf.push((-1).to_string());
        //         }
        //     }
        // }
    }

    for line in output_buf {
        println!("{}", line);
    }

    Ok(())
}
