use std::collections::HashMap;

fn main() {}

#[allow(unused)]
struct Solution;

#[allow(unused)]
impl Solution {
    pub fn character_replacement(s: String, k: i32) -> i32 {
        let mut charmap = HashMap::new();
        let mut maxlen = 0;

        let mut l = s.chars();
        for c in s.chars() {
            charmap.entry(c).and_modify(|e| *e += 1).or_insert(1);
            let most = *charmap.values().max().unwrap();
            let extra = charmap.values().sum::<i32>() - most;
            if extra > k {
                let drop = l.next().unwrap();
                charmap.entry(drop).and_modify(|e| *e -= 1);
            }
            maxlen = maxlen.max(charmap.values().sum::<i32>());
        }
        maxlen 
    }
}

#[test]
fn test_solution() {
    assert_eq!(Solution::character_replacement("ABAB".into(), 2), 4);
    assert_eq!(Solution::character_replacement("AABABBA".into(), 1), 4);
}