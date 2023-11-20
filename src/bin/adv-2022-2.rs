use std::{str::FromStr, io};

fn main() {
    let lines = io::stdin().lines();
    let mut total = 0;
    for line in lines {
        total += score(&line.expect("Invalid line"));
    }
    println!("Total is: {total}");
}

enum Rps {
    Rock,
    Paper,
    Scissors,
}

#[derive(Debug)]
struct RPSParseError;

impl FromStr for Rps {
    type Err = RPSParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" | "X" => Ok(Rps::Rock),
            "B" | "Y" => Ok(Rps::Paper),
            "C" | "Z" => Ok(Rps::Scissors),
            _ => Err(RPSParseError),
        }
    }
}

impl Rps {
    fn fight(&self, other: Self) -> i64 {
        match self {
            Rps::Rock => {
                let base = 1;
                match other {
                    Rps::Rock => base + 3,
                    Rps::Paper => base,
                    Rps::Scissors => base + 6,
                }
            }
            Rps::Paper => {
                let base = 2;
                match other {
                    Rps::Rock => base + 6,
                    Rps::Paper => base + 3,
                    Rps::Scissors => base,
                }
            }
            Rps::Scissors => {
                let base = 3;
                match other {
                    Rps::Rock => base,
                    Rps::Paper => base + 6,
                    Rps::Scissors => base + 3,
                }
            }
        }
    }
}

fn score(line: &str) -> i64 {
    let splits = line.trim().split_ascii_whitespace().collect::<Vec<_>>();
    let mut score = 0;
    score += match splits[..] {
        [opp, me] => {
            let opp = match Rps::from_str(opp) {
                Ok(v) => v,
                Err(_) => {
                    println!("Invalid opp: {opp}");
                    return 0;
                }
            };
            let me = match Rps::from_str(me) {
                Ok(v) => v,
                Err(_) => {
                    println!("Invalid me: {me}");
                    return 0;
                }
            };
            me.fight(opp)
        }
        _ => {
            println!("Invalid line: {line}");
            0 
        },
    };
    score
}

#[test]
fn test_small() {
    let lines = ["A Y", "B X", "C Z"];
    let mut total = 0;
    for line in lines {
        total += score(line)
    }
    assert_eq!(total, 15);
}
