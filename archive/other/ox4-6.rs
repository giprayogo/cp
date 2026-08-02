use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

/*
 * Complete the 'twoPluses' function below.
 *
 * The function is expected to return an INTEGER.
 * The function accepts STRING_ARRAY grid as parameter.
 */

fn twoPluses(grid: &[String]) -> i32 {
    let mut nicer_grid: Vec<Vec<char>> = grid.iter().map(|x| x.chars().collect()).collect();
    let yy = nicer_grid.len();
    let xx = nicer_grid[0].len();

    // up left down right =
    let u: (isize, isize) = (1, 0);
    let d: (isize, isize) = (-1, 0);
    let l: (isize, isize) = (0, -1);
    let r: (isize, isize) = (0, 1);
    let idk = vec![u, d, l, r];

    for i in 0..yy {
        for j in 0..xx {
            let mut cross_degree = 0;
            for shift in idk.iter() {
                let check = (i + cross_degree * shift.0, j + cross_degree * shift.1);
                // i + shift.0
                // j + shift.0
            }
        }
    }
    0
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

    let mut grid: Vec<String> = Vec::with_capacity(n as usize);

    for _ in 0..n {
        let grid_item = stdin_iterator.next().unwrap().unwrap();
        grid.push(grid_item);
    }

    let result = twoPluses(&grid);

    writeln!(&mut fptr, "{}", result).ok();
}

#[test]
fn test() {
    assert_eq!(
        twoPluses(&[
            "GGGGGG".into(),
            "GBBBGB".into(),
            "GGGGGG".into(),
            "GGBBGB".into(),
            "GGGGGG".into()
        ]),
        5
    );
    assert_eq!(
        twoPluses(&[
            "BGBBGB".into(),
            "GGGGGG".into(),
            "BGBBGB".into(),
            "GGGGGG".into(),
            "BGBBGB".into(),
            "BGBBGB".into(),
        ]),
        25
    );
}
