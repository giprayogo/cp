use std::{io, collections::HashSet};

fn main() {
    let lines = io::stdin().lines().flatten().collect::<Vec<_>>();
    for line in lines.iter() {
        let mut charset = HashSet::new();
        for (i, c) in line.chars().enumerate() {
            // should be 4 and in there
            // match charset.contains(&c) {
            //     True => 
            // }
            charset.insert(c);
        }
    }
}
