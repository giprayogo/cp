use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

/*
 * Complete the 'dayOfProgrammer' function below.
 *
 * The function is expected to return a STRING.
 * The function accepts INTEGER year as parameter.
 */

enum What {
    Julian,
    Gregorian,
    Weird,
}

fn day_of_programmer(year: i32) -> String {
    let what = {
        if year <= 1917 {
            What::Julian
        } else if year == 1918 {
            What::Weird
        } else {
            What::Gregorian
        }
    };

    let k = match what {
        What::Julian => {
            if year % 4 == 0 {
                29
            } else {
                28
            }
        }
        What::Weird => 14,
        What::Gregorian => {
            if year % 4 == 0 || year % 400 == 0 {
                29
            } else {
                28
            }
        }
    };

    let that = [31, k, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
    let mut month = 0;
    let mut day = 0;

    let mut acc = 0;
    for (i, t) in that.iter().enumerate() {
        if acc + t > 256 {
            month = i as i32 + 1;
            day = 256 - acc;
            break;
        }
        acc += t;
    }

    format!("{day}.{month:02}.{year}")
}

#[test]
fn test() {
    assert_eq!(day_of_programmer(2017), "13.09.2017".to_string());
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let year = stdin_iterator
        .next()
        .unwrap()
        .unwrap()
        .trim()
        .parse::<i32>()
        .unwrap();

    let result = day_of_programmer(year);

    writeln!(&mut fptr, "{}", result).ok();
}
