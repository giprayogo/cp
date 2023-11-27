use std::collections::BTreeMap;
use std::io;
use std::str::FromStr;

use itertools::Itertools;
use regex::Regex;

fn main() {
    let lines = io::stdin().lines().flatten();
    let mut stacks = BTreeMap::new();

    // crate notation parser
    let crates_re = Regex::new(r"\[[A-Z]\]").unwrap();
    let labels_re = Regex::new(r"\s[0-9]+\s").unwrap();
    let command_re = Regex::new(r"move\s([0-9]+)\sfrom\s([0-9]+)\sto\s([0-9]+)").unwrap();
    let mut defined = false;

    for line in lines {
        if !defined && crates_re.is_match(&line) {
            for c in crates_re.find_iter(&line) {
                let text = c
                    .as_str()
                    .trim_start_matches('[')
                    .trim_end_matches(']')
                    .to_string();
                stacks
                    .entry(c.start() / 4 + 1)
                    .and_modify(|e: &mut Vec<_>| e.push(text.clone()))
                    .or_insert(vec![text]);
            }
        } else if !defined && labels_re.is_match(&line) {
            for numchars in line.split_whitespace() {
                // to match empty stacks
                let numchars = i32::from_str(numchars).expect("Invalid number string");
                stacks
                    .entry(numchars as usize)
                    .and_modify(|e| e.reverse())
                    .or_insert(Vec::new());
            }
            defined = true;
            println!("{stacks:?}");
            println!(">> {line}");
        } else if command_re.is_match(&line) {
            let caps = command_re.captures(&line).unwrap();
            let move_howmany =
                usize::from_str(caps.get(1).expect("Can't capture move what").as_str()).unwrap();
            let from = usize::from_str(caps.get(2).expect("Can't capture from").as_str()).unwrap();
            let to = usize::from_str(caps.get(3).expect("Can't capture to").as_str()).unwrap();
            println!("{caps:?}");

            // CrateMover 9000
            // for _ in 0..move_howmany {
            //     let take = match stacks.get_mut(&from).unwrap().pop() {
            //         Some(v) => v,
            //         None => {
            //             println!("{:?}", stacks);
            //             panic!("Can't take from empty stack")
            //         }
            //     };
            //     stacks.get_mut(&to).unwrap().push(take);
            // }

            // CrateMover 9001
            let select = stacks.get_mut(&from).unwrap();
            let stack_height = select.len();
            let take = select.drain((stack_height - move_howmany)..).collect::<Vec<_>>();
            stacks.get_mut(&to).unwrap().extend(take);
        }
    }
    println!("{stacks:?}");
    println!("{}", stacks.values_mut().map(|e| e.pop().unwrap()).join(""));
}
