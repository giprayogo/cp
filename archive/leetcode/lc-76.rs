use std::collections::HashMap;

impl Solution {
    // Accepted but slow?
    pub fn min_window(s: String, t: String) -> String {
        let cs: Vec<_> = s.chars().collect();
        let ct: Vec<_> = t.chars().collect();
        if ct.len() > cs.len() {
            return "".into();
        }
        let mut ht = HashMap::new();
        let mut hs = HashMap::new();
        for t in ct.iter() {
            ht.entry(*t).and_modify(|e| *e += 1).or_insert(1);
        }
        for s in cs.iter().take(ct.len()) {
            hs.entry(*s).and_modify(|e| *e += 1).or_insert(1);
        }
        if ht == hs {
            return cs.iter().take(ct.len()).collect();
        }
        let mut min_left = 0;
        let mut min_right = 0;
        let mut min_length = usize::MAX;
        let mut left = 0;
        let mut right = ct.len();

        // neetocde: optimization; instead of matching the hashmap, do auxilary nteger count

        fn inside(h1: &mut HashMap<char, i32>, h2: &mut HashMap<char, i32>) -> bool {
            for (k, v) in h1.iter() {
                if let Some(_v) = h2.get(k) {
                    if _v < v {
                        return false;
                    }
                } else {
                    return false;
                }
            }
            true
        }
        loop {
            if inside(&mut ht, &mut hs) {
                let length = right - left;
                if length < min_length {
                    min_length = length;
                    min_left = left;
                    min_right = right;
                }
                let popc = cs[left];
                hs.entry(popc).and_modify(|e| *e -= 1);
                if let Some(v) = hs.get(&popc) {
                    if *v == 0 {
                        hs.remove_entry(&popc);
                    }
                }
                left += 1;
            } else {
                if right == cs.len() {
                    break;
                }
                let pushc = cs[right];
                hs.entry(pushc).and_modify(|e| *e += 1).or_insert(1);
                right += 1;
                right = right.min(cs.len()); // can get stuck
            }
        }
        cs[min_left..min_right].iter().collect()
    }
}

struct Solution;

fn main() {
    assert_eq!(
        Solution::min_window("ADOBECODEBANC".into(), "ABC".into()),
        "BANC"
    );
    assert_eq!(Solution::min_window("a".into(), "a".into()), "a");
    assert_eq!(Solution::min_window("a".into(), "aa".into()), "");
}
