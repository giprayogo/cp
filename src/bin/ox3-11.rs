use std::collections::BTreeSet;
use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

/*
 * Complete the 'climbingLeaderboard' function below.
 *
 * The function is expected to return an INTEGER_ARRAY.
 * The function accepts following parameters:
 *  1. INTEGER_ARRAY ranked
 *  2. INTEGER_ARRAY player
 */

fn climbing_leaderboard(ranked: &[i32], player: &[i32]) -> Vec<i32> {
    let mut baa = BTreeSet::new();
    for r in ranked {
        baa.insert(*r);
    }
    for p in player {
        baa.insert(*p);
    }
    let vbaa = baa.into_iter().rev().collect::<Vec<_>>();

    let mut l = vec![];
    for p in player {
        for (i, b) in vbaa.iter().enumerate() {
            if *p == *b {
                l.push(i as i32 + 1);
                break;
            }
        }
    }
    l
}

#[test]
fn test() {
    climbing_leaderboard(&[100, 100, 50, 40, 40, 20, 10], &[5, 25, 50, 120]);
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let _ranked_count = stdin_iterator
        .next()
        .unwrap()
        .unwrap()
        .trim()
        .parse::<i32>()
        .unwrap();

    let ranked: Vec<i32> = stdin_iterator
        .next()
        .unwrap()
        .unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i32>().unwrap())
        .collect();

    let _player_count = stdin_iterator
        .next()
        .unwrap()
        .unwrap()
        .trim()
        .parse::<i32>()
        .unwrap();

    let player: Vec<i32> = stdin_iterator
        .next()
        .unwrap()
        .unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i32>().unwrap())
        .collect();

    let result = climbing_leaderboard(&ranked, &player);

    for i in 0..result.len() {
        write!(&mut fptr, "{}", result[i]).ok();

        if i != result.len() - 1 {
            writeln!(&mut fptr).ok();
        }
    }

    writeln!(&mut fptr).ok();
}
