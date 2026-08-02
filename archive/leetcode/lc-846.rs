use std::collections::BTreeMap;

struct Solution;

impl Solution {
    // accepted but not amazingly fast; not slow tho!
    pub fn is_n_straight_hand_1(hand: Vec<i32>, group_size: i32) -> bool {
        if hand.len() % group_size as usize != 0 {
            return false;
        }

        let mut hand_map: BTreeMap<i32, u32> = BTreeMap::new();
        for card in hand {
            hand_map.entry(card).and_modify(|e| *e += 1).or_insert(1);
        }
        let keys: Vec<_> = hand_map.keys().cloned().collect();
        'key: for key in keys {
            loop {
                for i in 0..group_size {
                    match hand_map.get_mut(&(key + i)) {
                        Some(v) => match v.checked_sub(1) {
                            Some(_v) => *v -= 1,
                            None => {
                                if i == 0 {
                                    continue 'key;
                                } else {
                                    return false;
                                }
                            }
                        },
                        None => return false,
                    }
                }
            }
        }
        true
    }

    // same heuristic but optimized; really I overthink again: technicall all similar solutions
    // just how careful you are!
    pub fn is_n_straight_hand_2(hand: Vec<i32>, group_size: i32) -> bool {
        if hand.len() % group_size as usize != 0 {
            return false;
        }

        let mut hand_map: BTreeMap<i32, u32> = BTreeMap::new();
        for card in hand {
            hand_map.entry(card).and_modify(|e| *e += 1).or_insert(1);
        }
        let keys: Vec<_> = hand_map.keys().cloned().collect();
        for key in keys {
            let value = hand_map.remove(&key).unwrap();
            if value == 0 {
                continue;
            }
            for i in 1..group_size {
                match hand_map.get_mut(&(key + i)) {
                    Some(v) => match v.checked_sub(value) {
                        Some(_v) => *v -= value,
                        None => return false,
                    },
                    None => return false,
                }
            }
        }
        true
    }
}

fn main() {
    for f in [
        Solution::is_n_straight_hand_1,
        Solution::is_n_straight_hand_2,
    ] {
        assert!(f(vec![1, 2, 3, 6, 2, 3, 4, 7, 8], 3));
        assert!(!f(vec![1, 2, 3, 4, 5], 4));
        assert!(f(vec![4], 1));
    }
}
