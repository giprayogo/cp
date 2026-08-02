use std::str::FromStr;
use std::{collections::HashMap, io};

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
enum Type {
    Five(Vec<i32>, i32),
    Four(Vec<i32>, i32),
    Full(Vec<i32>, i32),
    Three(Vec<i32>, i32),
    TwoPair(Vec<i32>, i32),
    Pair(Vec<i32>, i32),
    High(Vec<i32>, i32),
}

fn main() {
    let lines = io::stdin().lines().flatten();
    let mut hands = Vec::new();
    for line in lines {
        let mut line_iterator = line.split_whitespace();
        let hand_part = match line_iterator.next() {
            Some(v) => v,
            None => continue,
        };
        let bid = i32::from_str(match line_iterator.next() {
            Some(v) => v,
            None => continue,
        })
        .unwrap();

        let mut hand_counts = HashMap::new();
        let mut hand_order = Vec::new();

        for char in hand_part.chars() {
            let char_value = match char {
                'T' => -10,
                // 'J' => -11, // part 1
                'J' => 0, // part 2
                'Q' => -12,
                'K' => -13,
                'A' => -14,
                v => -i32::from_str(&v.to_string()).unwrap(),
            };
            hand_counts
                .entry(char_value)
                .and_modify(|e| *e += 1)
                .or_insert(1);
            hand_order.push(char_value);
        }
        let hand = 'hand: {
            let mut counts = hand_counts.values().copied().collect::<Vec<_>>();
            counts.sort();
            counts.reverse();
            // part 2
            if let Some(v) = hand_counts.get(&0) {
                if *v == counts[0] {
                    if hand_counts.len() != 1 {
                        counts[0] += counts[1];
                    }
                } else {
                    counts[0] += v;
                }
            }

            match counts[0] {
                5 => Type::Five(hand_order, bid),
                4 => Type::Four(hand_order, bid),
                3 => match counts[1] {
                    2 => break 'hand Type::Full(hand_order, bid),
                    1 => break 'hand Type::Three(hand_order, bid),
                    _ => unreachable!(),
                },
                2 => match counts[1] {
                    2 => break 'hand Type::TwoPair(hand_order, bid),
                    1 => break 'hand Type::Pair(hand_order, bid),
                    _ => unreachable!(),
                },
                1 => Type::High(hand_order, bid),
                _ => {
                    println!("{hand_counts:?}");
                    println!("{counts:?}");
                    unreachable!();
                }
            }
        };
        hands.push(hand);
    }

    hands.sort();
    hands.reverse();
    let mut winnings = 0;
    for (i, hand) in hands.iter().enumerate() {
        let i = i as i32;
        match hand {
            Type::Five(_, bid) => winnings += bid * (i + 1),
            Type::Four(_, bid) => winnings += bid * (i + 1),
            Type::Full(_, bid) => winnings += bid * (i + 1),
            Type::Three(_, bid) => winnings += bid * (i + 1),
            Type::TwoPair(_, bid) => winnings += bid * (i + 1),
            Type::Pair(_, bid) => winnings += bid * (i + 1),
            Type::High(_, bid) => winnings += bid * (i + 1),
        }
    }
    println!("{winnings}");
}
