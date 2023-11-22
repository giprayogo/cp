use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

/*
 * Complete the 'countingValleys' function below.
 *
 * The function is expected to return an INTEGER.
 * The function accepts following parameters:
 *  1. INTEGER steps
 *  2. STRING path
 */

fn counting_valleys(_steps: i32, path: &str) -> i32 {
    let mut level = 0;
    let mut valleys = 0;

    for p in path.chars() {
        match p {
            'U' => level += 1,
            'D' => level -= 1,
            _ => unreachable!(),
        }
        if level == 0 && p == 'U' {
            valleys += 1;
        }
    }
    valleys
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let steps = stdin_iterator
        .next()
        .unwrap()
        .unwrap()
        .trim()
        .parse::<i32>()
        .unwrap();

    let path = stdin_iterator.next().unwrap().unwrap();

    let result = counting_valleys(steps, &path);

    writeln!(&mut fptr, "{}", result).ok();
}
