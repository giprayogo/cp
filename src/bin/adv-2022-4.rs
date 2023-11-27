use std::collections::HashSet;
use std::io;
use std::str::FromStr;

fn main() {
    let lines = io::stdin().lines().flatten();
    let mut covers = 0;
    let mut overlaps = 0;
    for line in lines {
        if line.is_empty() {
            continue;
        }
        let (elf_a, elf_b) = match &line
            .split(',')
            .map(|e| e.split('-'))
            .map(|e| match e.clone().collect::<Vec<_>>()[..] {
                [start, stop] => {
                    let start = i32::from_str(start).unwrap();
                    let stop = i32::from_str(stop).unwrap();
                    (start..=stop).collect::<HashSet<_>>()
                }
                _ => panic!("Invalid dash split: {:?}", e),
            })
            .collect::<Vec<_>>()[..]
        {
            [a, b] => (a.clone(), b.clone()),
            _ => panic!("Invalid comma split: {}", line),
        };
        if elf_a.is_superset(&elf_b) || elf_a.is_subset(&elf_b) {
            covers += 1;
        }
        if !elf_a.is_disjoint(&elf_b) {
            overlaps += 1;
        }
    }
    println!("bad: {covers}");
    println!("somewhat bad: {overlaps}");
}
