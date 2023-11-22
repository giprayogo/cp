use std::io::{self, BufRead};

/*
 * Complete the 'plusMinus' function below.
 *
 * The function accepts INTEGER_ARRAY arr as parameter.
 */

fn plus_minus(arr: &[i32]) -> (f64, f64, f64) {
    let mut pos: f64 = 0.;
    let mut neg: f64 = 0.;
    let mut zero: f64 = 0.;
    for i in arr {
        if i.is_positive() {
            pos += 1.;
        } else if i.is_negative() {
            neg += 1.;
        } else {
            zero += 1.;
        }
    }
    let l = arr.len() as f64;
    (pos / l, neg / l, zero / l)
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let _n = stdin_iterator
        .next()
        .unwrap()
        .unwrap()
        .trim()
        .parse::<i32>()
        .unwrap();

    let arr: Vec<i32> = stdin_iterator
        .next()
        .unwrap()
        .unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i32>().unwrap())
        .collect();

    let (a, b, c) = plus_minus(&arr);
    println!("{a:.6}\n{b:.6}\n{c:.6}")
}
