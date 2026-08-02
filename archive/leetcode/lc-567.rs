use std::collections::HashMap;

impl Solution {
    pub fn check_inclusion(s1: String, s2: String) -> bool {
        let c1: Vec<_> = s1.chars().collect();
        let c2: Vec<_> = s2.chars().collect();
        let n = c1.len();
        let d = match c2.len().checked_sub(c1.len()) {
            Some(v) => v,
            None => return false,
        };
        let mut h1 = HashMap::new();
        let mut h2 = HashMap::new();
        for c in c1.iter() {
            h1.entry(*c).and_modify(|e| *e += 1).or_insert(1);
        }
        for c in c2.iter().take(n) {
            h2.entry(*c).and_modify(|e| *e += 1).or_insert(1);
        }
        for shift in 0..=d {
            if h1 == h2 {
                return true;
            }
            let c_head = c2[shift];
            let c_tail = match c2.get(shift + n) {
                Some(v) => *v,
                None => return false,
            };

            h2.entry(c_head).and_modify(|e| *e -= 1);
            if let Some(v) = h2.get(&c_head) {
                if *v == 0 {
                    h2.remove(&c_head);
                }
            }
            h2.entry(c_tail).and_modify(|e| *e += 1).or_insert(1);
        }
        false
    }

    // neetcode: basically the same but uses vector instead of hashmap: probably more efficient as only
    // one is checked at a time?
    pub fn check_inclusion_2(s1: String, s2: String) -> bool {
        let c1: Vec<_> = s1.chars().collect();
        let c2: Vec<_> = s2.chars().collect();
        let n = c1.len();
        let d = match c2.len().checked_sub(c1.len()) {
            Some(v) => v,
            None => return false,
        };
        let mut v1 = [0; 26];
        let mut v2 = [0; 26];
        for c in c1.iter() {
            v1[*c as usize - 'a' as usize] += 1;
        }
        for c in c2.iter().take(n) {
            v2[*c as usize - 'a' as usize] += 1;
        }
        let mut matches = 0;
        for (_v1, _v2) in v1.iter().zip(v2.iter()) {
            if *_v1 == *_v2 {
                matches += 1;
            }
        }
        if matches == 26 {
            return true;
        }
        for shift in 0..=d {
            let c_head = c2[shift];
            let c_tail = match c2.get(shift + n) {
                Some(v) => *v,
                None => return false,
            };

            let i_head = c_head as usize - 'a' as usize;
            let i_tail = c_tail as usize - 'a' as usize;
            v2[i_head] -= 1;
            v2[i_tail] += 1;
            if v1[i_head] == v2[i_head] {
                matches += 1;
            } else if v1[i_head] - 1 == v2[i_head] {
                // this logic is fragile though; and does not work if e.g. the count go back up/down
                // does not work with the latest checkes!
                matches -= 1;
            }
            if v1[i_tail] == v2[i_tail] {
                matches += 1;
            } else if v1[i_tail] + 1 == v2[i_tail] {
                matches -= 1;
            }
            if matches == 26 {
                return true;
            }
        }
        false
    }
}

struct Solution;

fn main() {
    for f in [Solution::check_inclusion, Solution::check_inclusion_2] {
        // assert!(f("ab".into(), "eidbaooo".into()));
        // assert!(!f("ab".into(), "eidboaoo".into()));
        // assert!(f("adc".into(), "dcda".into()));
        assert!(f("abc".into(), "bbbca".into()));
    }
}
