use std::collections::{BTreeMap, HashMap};

#[allow(unused)]
struct Solution;

#[allow(unused)]
impl Solution {
    // raw pointers!
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut m = HashMap::new();
        let offset = -1 * 'a' as isize;

        for s in strs.iter() {
            let mut h = vec![0; 26];
            unsafe {
                let p = h.as_mut_ptr().offset(offset);
                for c in s.bytes() {
                    *p.offset(c as isize) += 1;
                }
            }
            m.entry(h)
                .and_modify(|v: &mut Vec<_>| v.push(s.to_owned()))
                .or_insert(vec![s.to_owned()]);
        }
        m.into_values().collect()
    }
    // // proper entry use
    // pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
    //     let mut m = HashMap::new();
    //     let offset = 'a' as usize;

    //     for s in strs.iter() {
    //         let mut h = vec![0; 26];
    //         for c in s.chars() {
    //             h[c as usize - offset] += 1;
    //         }
    //         m.entry(h)
    //             .and_modify(|v: &mut Vec<_>| v.push(s.to_owned()))
    //             .or_insert(vec![s.to_owned()]);
    //     }
    //     m.into_values().collect::<Vec<_>>()
    // }
    // Smarter with fixed size vector
    // pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
    //     let mut m = HashMap::new();
    //     let offset = 'a' as usize;

    //     for s in strs.iter() {
    //         let mut h = vec![0; 26];
    //         for c in s.chars() {
    //             h[c as usize - offset] += 1;
    //         }
    //         if m.contains_key(&h) {
    //             m.entry(h).and_modify(|v: &mut Vec<_>| v.push(s.to_owned()));
    //         } else {
    //             m.insert(h, vec![s.to_owned()]);
    //         }
    //     }
    //     m.into_values().collect::<Vec<_>>()
    // }
    // Naive double map solution
    // pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
    //     let mut m = HashMap::new();
    //     for s in strs.iter() {
    //         let mut h = BTreeMap::new();
    //         for c in s.chars() {
    //             h.entry(c).and_modify(|e| *e += 1).or_insert(0);
    //         }
    //         if m.contains_key(&h) {
    //             m.entry(h).and_modify(|v: &mut Vec<_>| v.push(s.to_owned()));
    //         } else {
    //             m.insert(h, vec![s.to_owned()]);
    //         }
    //     }
    //     m.into_values().collect::<Vec<_>>()
    // }
}

fn main() {}

#[test]
fn test_solution() {
    let strs: Vec<_> = ["eat", "tea", "tan", "ate", "nat", "bat"]
        .iter()
        .map(|&x| x.to_string())
        .collect();
    println!("{:?}", Solution::group_anagrams(strs));
    let strs: Vec<_> = ["ddddddddddg", "dgggggggggg"]
        .iter()
        .map(|&x| x.to_string())
        .collect();
    println!("{:?}", Solution::group_anagrams(strs));
    let strs: Vec<_> = [""].iter().map(|&x| x.to_string()).collect();
    println!("{:?}", Solution::group_anagrams(strs));
    let strs: Vec<_> = ["a"].iter().map(|&x| x.to_string()).collect();
    println!("{:?}", Solution::group_anagrams(strs));
}
