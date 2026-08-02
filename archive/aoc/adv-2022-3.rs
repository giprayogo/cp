use std::{
    collections::{HashMap, HashSet},
    io,
};

fn main() {
    let lines = io::stdin().lines().flatten().collect::<Vec<_>>();
    let mut wrongs = Vec::new();
    for line in lines.iter() {
        let chars: Vec<_> = line.chars().collect();
        let len = chars.len();
        let comp_a: HashSet<_> = chars[..len / 2].iter().collect();
        let comp_b: HashSet<_> = chars[len / 2..].iter().collect();
        for e in comp_a.intersection(&comp_b) {
            wrongs.push(*(*e));
        }
    }

    let map: HashMap<_, _> = ('a'..='z').chain('A'..='Z').zip(1..=52).collect();
    println!(
        "{}",
        wrongs.iter().map(|e| map.get(e).unwrap()).sum::<i32>()
    );

    let mut sum = 0;
    for group in lines.chunks(3) {
        if group.len() != 3 {
            println!("WARNING: odd group size");
            println!("{group:?}");
            break;
        }
        let shared = group
            .iter()
            .map(|e| e.chars().collect::<HashSet<_>>())
            .reduce(|acc, e| acc.intersection(&e).map(|e| e.to_owned()).collect());
        if let Some(mut value) = shared {
            println!("{:?}", value);
            assert!(value.len() == 1); 
            for e in value.drain() {
                sum += map.get(&e).unwrap();
            }
        }
    }
    println!("2nd sum: {sum}");
}
