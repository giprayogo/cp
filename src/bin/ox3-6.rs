use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

/*
 * Complete the 'migratoryBirds' function below.
 *
 * The function is expected to return an INTEGER.
 * The function accepts INTEGER_ARRAY arr as parameter.
 */

fn migratoryBirds(arr: &[i32]) -> i32 {
    let mut h = HashMap::new();
    for i in arr {
        h.entry(*i).and_modify(|e| { *e += 1}).or_insert(1);
    }
    let mut mc = i32::MIN;
    let mut mk = i32::MAX;
    for (k, v) in h.iter() {
        if *v > mc {
            mc = *v;
            mk = *k;
        } else if *v == mc && *k < mk {
            mk = mk.min(*k);
        }
    }
    mk
}

#[test]
fn test() {
    assert_eq!(migratoryBirds(&vec![1, 2, 3, 4, 5, 4, 3, 2, 1, 3, 4][..]), 3);
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let _arr_count = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    let arr: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i32>().unwrap())
        .collect();

    let result = migratoryBirds(&arr);

    writeln!(&mut fptr, "{}", result).ok();
}
