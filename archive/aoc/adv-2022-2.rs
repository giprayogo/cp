use std::{io, str::FromStr};

fn main() {
    let lines = io::stdin().lines().collect::<Vec<_>>();
    let mut total = 0;
    for line in lines.iter() {
        total += score(line.as_ref().expect("Invalid line"));
    }
    println!("Total is: {total}");
    total = 0;
    for line in lines {
        total += score_round_2(&line.expect("Invalid line"));
    }
    println!("Total round 2 is: {total}");
}

enum Rps {
    Rock,
    Paper,
    Scissors,
}

/// For part B
enum Outcome {
    Win,
    Lose,
    Draw,
}

#[derive(Debug)]
struct OutcomeParseError;

impl FromStr for Outcome {
    type Err = OutcomeParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "X" => Ok(Outcome::Lose),
            "Y" => Ok(Outcome::Draw),
            "Z" => Ok(Outcome::Win),
            _ => Err(OutcomeParseError),
        }
    }
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

    fn fight_me_with_what(&self, outcome: Outcome) -> Rps {
        match self {
            Rps::Rock => match outcome {
                Outcome::Win => Rps::Paper,
                Outcome::Draw => Rps::Rock,
                Outcome::Lose => Rps::Scissors,
            },
            Rps::Paper => match outcome {
                Outcome::Win => Rps::Scissors,
                Outcome::Draw => Rps::Paper,
                Outcome::Lose => Rps::Rock,
            },
            Rps::Scissors => match outcome {
                Outcome::Win => Rps::Rock,
                Outcome::Draw => Rps::Scissors,
                Outcome::Lose => Rps::Paper,
            },
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
        }
    };
    score
}

fn score_round_2(line: &str) -> i64 {
    let splits = line.trim().split_ascii_whitespace().collect::<Vec<_>>();
    let mut score = 0;
    score += match splits[..] {
        [opp, outcome] => {
            let opp = match Rps::from_str(opp) {
                Ok(v) => v,
                Err(_) => {
                    println!("Invalid (opp): {opp}");
                    return 0;
                }
            };
            let outcome = match Outcome::from_str(outcome) {
                Ok(v) => v,
                Err(_) => {
                    println!("Invalid outcome: {outcome}");
                    return 0;
                }
            };
            let me = opp.fight_me_with_what(outcome);
            me.fight(opp)
        }
        _ => {
            println!("Invalid line: {line}");
            0
        }
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
    // for round 2
    assert_eq!(score_round_2(lines[0]), 4);
    assert_eq!(score_round_2(lines[1]), 1);
    assert_eq!(score_round_2(lines[2]), 7);
}
