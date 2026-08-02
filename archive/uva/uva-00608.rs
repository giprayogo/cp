use std::collections::BTreeMap; // easier to check result: no effect though
// use std::collections::HashMap;
use std::error;
use std::io;

use itertools::Itertools;

#[derive(Debug)]
enum Weight {
    Normal,
    Light,
    Heavy,
}

fn main() -> Result<(), Box<dyn error::Error>> {
    let mut buf = String::new();
    let stdin = io::stdin();

    use Weight::*;

    let charset = "ABCDEFGHIJKL".chars().collect_vec();
    let mut c_map = BTreeMap::new();
    for c in &charset {
        c_map.insert(*c, None);
    }

    stdin.read_line(&mut buf)?;
    let n = buf.trim().parse()?;
    for _ in 0..n {
        for _ in 0..3 {
            buf.clear();
            stdin.read_line(&mut buf)?;
            match buf.split_whitespace().collect_vec()[..] {
                [group_a, group_b, status] => match status {
                    "even" => {
                        for c in group_a.chars().chain(group_b.chars()) {
                            c_map.entry(c).and_modify(|e| *e = Some(Normal));
                        }
                    }
                    "up" => {
                        for c in group_a.chars() {
                            c_map.entry(c).and_modify(|e| match e {
                                Some(_) => {}
                                None => *e = Some(Heavy),
                            });
                        }
                        for c in group_b.chars() {
                            c_map.entry(c).and_modify(|e| match e {
                                Some(_) => {}
                                None => *e = Some(Light),
                            });
                        }
                    },
                    "down" => {
                        for c in group_a.chars() {
                            c_map.entry(c).and_modify(|e| match e {
                                Some(_) => {}
                                None => *e = Some(Light),
                            });
                        }
                        for c in group_b.chars() {
                            c_map.entry(c).and_modify(|e| match e {
                                Some(_) => {}
                                None => *e = Some(Heavy),
                            });
                        }
                    },
                    _ => panic!("Invalid status: {}", status),
                },
                _ => panic!("Invalid input: {}", buf),
            };
        }
    }

    for (k, v) in c_map.iter() {
        // println!("{} - {:?}", k, v);
        match v {
            Some(Normal) => {},
            Some(Light) => {
                println!("{} is the counterfeit coin and it is light.", k);
                break;
            },
            Some(Heavy) => {
                println!("{} is the counterfeit coin and it is heavy.", k);
                break;
            },
            _ => {},
        }
    }

    Ok(())
}
