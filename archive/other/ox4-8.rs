use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

/*
 * Complete the 'stringSimilarity' function below.
 *
 * The function is expected to return an INTEGER.
 * The function accepts STRING s as parameter.
 */
// aabc
// abc
// bc
// c

fn stringSimilarity(s: &str) -> i32 {
    let c: Vec<char> = s.chars().collect();
    let mut ns = vec![];
    for offset in 0..c.len() {
        let mut n = 0;
        for (a, b) in c.iter().zip(c[offset..].iter()) {
            if a == b {
                n += 1;
            } else {
                break;
            }
        }
        ns.push(n);
    }
    ns.iter().sum::<i32>()
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let t = stdin_iterator
        .next()
        .unwrap()
        .unwrap()
        .trim()
        .parse::<i32>()
        .unwrap();

    for _ in 0..t {
        let s = stdin_iterator.next().unwrap().unwrap();

        let result = stringSimilarity(&s);

        writeln!(&mut fptr, "{}", result).ok();
    }
}

#[test]
fn test() {
    assert_eq!(stringSimilarity("ababaa".into()), 11);
    assert_eq!(stringSimilarity("aa".into()), 3);
}
