use std::collections::HashMap;

struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn is_anagram_sort(s: String, t: String) -> bool {
        let mut s: Vec<_> = s.chars().collect();
        let mut t: Vec<_> = t.chars().collect();
        s.sort();
        t.sort();
        s == t
    }
    pub fn is_anagram_hash(s: String, t: String) -> bool {
        // mini optimization -> cost memory!
        // if s.len() != t.len() {
        //     return false;
        // }
        let mut h = HashMap::new();
        let mut i = HashMap::new();
        for _s in s.chars() {
            h.entry(_s).and_modify(|e| *e += 1).or_insert(0);
        }
        for _t in t.chars() {
            i.entry(_t).and_modify(|e| *e += 1).or_insert(0);
        }
        h == i
    }
    // this one is necessary with this technique
    pub fn is_anagram(s: String, t: String) -> bool {
        if s.len() != t.len() {
            return false;
        }
        let mut h: HashMap<char, i32> = HashMap::new();
        for (_s, _t) in s.chars().zip(t.chars()) {
            *h.entry(_s).or_default() += 1;
            *h.entry(_t).or_default() -= 1;
        }
        h.into_values().all(|x| x == 0)
    }
}

fn main() {}

#[test]
fn test_solution() {
    let s = "anagram".to_string();
    let t = "nagaram".to_string();
    assert!(Solution::is_anagram(s, t));
    let s = "rat".to_string();
    let t = "car".to_string();
    assert!(!Solution::is_anagram(s, t));
}
