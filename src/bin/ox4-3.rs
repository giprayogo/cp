use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

/*
 * Complete the 'acmTeam' function below.
 *
 * The function is expected to return an INTEGER_ARRAY.
 * The function accepts STRING_ARRAY topic as parameter.
 */

fn acmTeam(topic: &[String]) -> Vec<i32> {
    let mut max_n = 0;
    let mut count = 0;
    for i in 0..topic.len() {
        for j in 0..i {
            let a = &topic[i];
            let b = &topic[j];
            let n = a
                .chars()
                .zip(b.chars())
                .map(|(_a, _b)| match (_a, _b) {
                    ('0', '0') => 0,
                    _ => 1,
                })
                .sum();
            if n > max_n {
                max_n = n;
                count = 1;
            } else if n == max_n {
                count += 1;
            }
        }
    }
    vec![max_n, count]
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let first_multiple_input: Vec<String> = stdin_iterator
        .next()
        .unwrap()
        .unwrap()
        .split(' ')
        .map(|s| s.to_string())
        .collect();

    let n = first_multiple_input[0].trim().parse::<i32>().unwrap();

    let m = first_multiple_input[1].trim().parse::<i32>().unwrap();

    let mut topic: Vec<String> = Vec::with_capacity(n as usize);

    for _ in 0..n {
        let topic_item = stdin_iterator.next().unwrap().unwrap();
        topic.push(topic_item);
    }

    let result = acmTeam(&topic);

    for i in 0..result.len() {
        write!(&mut fptr, "{}", result[i]).ok();

        if i != result.len() - 1 {
            writeln!(&mut fptr).ok();
        }
    }

    writeln!(&mut fptr).ok();
}

#[test]
fn test() {
    assert_eq!(
        acmTeam(&["10101".into(), "11110".into(), "00010".into()]),
        vec![5, 1]
    );
    assert_eq!(
        acmTeam(&[
            "10101".into(),
            "11100".into(),
            "11010".into(),
            "00101".into()
        ]),
        vec![5, 2]
    )
}
