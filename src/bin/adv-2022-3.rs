use std::{collections::{HashSet, HashMap}, io};

fn main() {
    let lines = io::stdin().lines().collect::<Vec<_>>();
    let mut wrongs = Vec::new();
    for line in lines.iter().flatten() {
        let chars: Vec<_> = line.chars().collect();
        let len = chars.len();
        let comp_a: HashSet<_> = chars[..len / 2].iter().collect();
        let comp_b: HashSet<_> = chars[len / 2..].iter().collect();
        for e in comp_a.intersection(&comp_b) {
            wrongs.push(*(*e));
        }
    }
    
    let map: HashMap<_, _> = ('a'..='z').chain('A'..='Z').zip(1..=52).collect();
    println!("{}", wrongs.iter().map(|e| map.get(e).unwrap()).sum::<i32>());
}
