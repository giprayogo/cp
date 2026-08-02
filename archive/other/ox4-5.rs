use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

/*
 * Complete the 'gridSearch' function below.
 *
 * The function is expected to return a STRING.
 * The function accepts following parameters:
 *  1. STRING_ARRAY G
 *  2. STRING_ARRAY P
 */

fn gridSearch(G: &[String], P: &[String]) -> String {
    let mut huh = false;
    // let mut col = None;
    // let mut cols = vec![None; P.len()];

    // find head first
    let head_chars: Vec<char> = P[0].chars().collect();
    for g in G.iter() {
        let cg: Vec<char> = g.chars().collect();

        
        // let i = 0;
        // for c in head_chars.iter() {
        // cg.contains(c);
        // }

        if let Some(i) = g.find(&P[0]) {
            // try again
        }
    }

    // What if there are multiple rows with the same pattern?
    // iterate rows
    // chekc if first row of pattern in row > check if there are multiple
    // if none continue
    // if (g, p) -> indices
    // check p+1 at row p+1 at those indices
    // if none stop
    // if yes until pattern exhausted turn huh into true
    // for (i, p) in P.iter().enumerate() {
    //     for g in G {
    //         match g.find(p) {
    //             Some(v) => {
    //                 cols[i] = Some(v);
    //                 break;
    //             }
    //             None => {}
    //         };

    // }

    if huh {
        "OK".into()
    } else {
        "NO".into()
    }
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
        let first_multiple_input: Vec<String> = stdin_iterator
            .next()
            .unwrap()
            .unwrap()
            .split(' ')
            .map(|s| s.to_string())
            .collect();

        let R = first_multiple_input[0].trim().parse::<i32>().unwrap();

        let C = first_multiple_input[1].trim().parse::<i32>().unwrap();

        let mut G: Vec<String> = Vec::with_capacity(R as usize);

        for _ in 0..R {
            let G_item = stdin_iterator.next().unwrap().unwrap();
            G.push(G_item);
        }

        let second_multiple_input: Vec<String> = stdin_iterator
            .next()
            .unwrap()
            .unwrap()
            .split(' ')
            .map(|s| s.to_string())
            .collect();

        let r = second_multiple_input[0].trim().parse::<i32>().unwrap();

        let c = second_multiple_input[1].trim().parse::<i32>().unwrap();

        let mut P: Vec<String> = Vec::with_capacity(r as usize);

        for _ in 0..r {
            let P_item = stdin_iterator.next().unwrap().unwrap();
            P.push(P_item);
        }

        let result = gridSearch(&G, &P);

        writeln!(&mut fptr, "{}", result).ok();
    }
}

#[test]
fn test() {
    assert_eq!(
        gridSearch(
            &[
                "7283455864".into(),
                "6731158619".into(),
                "8988242643".into(),
                "3830589324".into(),
                "2229505813".into(),
                "5633845374".into(),
                "6473530293".into(),
                "7053106601".into(),
                "0834282956".into(),
                "4607924137".into(),
            ],
            &["9505".into(), "3845".into(), "3530".into()]
        ),
        "YES"
    );
    assert_eq!(
        gridSearch(
            &[
                "400453592126560".into(),
                "114213133098692".into(),
                "474386082879648".into(),
                "522356951189169".into(),
                "887109450487496".into(),
                "252802633388782".into(),
                "502771484966748".into(),
                "075975207693780".into(),
                "511799789562806".into(),
                "404007454272504".into(),
                "549043809916080".into(),
                "962410809534811".into(),
                "445893523733475".into(),
                "768705303214174".into(),
                "650629270887160".into(),
            ],
            &["99".into(), "99".into()]
        ),
        "NO"
    );
}
