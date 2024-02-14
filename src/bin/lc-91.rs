// use std::collections::{HashMap, HashSet};
use std::collections::HashMap;

struct Solution;

impl Solution {
    // What if I don't use valid set but hard-coded rules? Also probably I don't need Vec<char> hash map...
    // Technically BFS not DP --try DP solution later
    pub fn num_decodings(s: String) -> i32 {
        let chars: Vec<char> = s.chars().collect();
        let mut cache = HashMap::new();
        cache.insert(chars.len(), 1);

        Solution::n(&mut cache, &chars, 0)
    }

    // NOTE: using index might be a bit clunky, but it avoids clone when using slice as map keyk
    fn n(cache: &mut HashMap<usize, i32>, chars: &[char], i: usize) -> i32 {
        match cache.get(&i) {
            Some(v) => *v,
            None => {
                if chars[i] == '0' {
                    cache.insert(i, 0);
                    return 0;
                }

                let mut v = Solution::n(cache, chars, i + 1);
                v += if i + 1 < chars.len() {
                    match chars[i] {
                        '1' => Solution::n(cache, chars, i + 2),
                        '2' => match chars[i + 1] {
                            '7' | '8' | '9' => 0,
                            _ => Solution::n(cache, chars, i + 2),
                        },
                        _ => 0,
                    }
                } else {
                    0
                };
                cache.insert(i, v);
                v
            }
        }
    }

    // ========================================================
    // Not TLE but kind of slow??
    /*     pub fn num_decodings(s: String) -> i32 {
        let chars: Vec<char> = s.chars().collect();
        let valid: HashSet<_> = (1..=26)
            .map(|x| x.to_string().chars().collect::<Vec<_>>())
            .collect();
        let mut cache: HashMap<Vec<char>, i32> = HashMap::new();
        cache.insert(vec![], 1);

        Solution::n(&mut cache, &valid, &chars[..])
    }

    fn n(cache: &mut HashMap<Vec<char>, i32>, valid: &HashSet<Vec<char>>, n: &[char]) -> i32 {
        match cache.get(n) {
            Some(v) => *v,
            None => {
                let left = if valid.contains(&n[0..1]) {
                    Solution::n(cache, valid, &n[1..])
                } else {
                    0
                };
                let right = if n.len() >= 2 && valid.contains(&n[0..2]) {
                    Solution::n(cache, valid, &n[2..])
                } else {
                    0
                };
                cache.insert(n.to_owned(), left + right);
                left + right
            }
        }
    } */

    // ========================================================
    // At this point really I should have learnt about early optimization!
    // let mut stack = Vec::new();
    // stack.push((0..1, 1..chars.len()));
    // stack.push((0..2, 2..chars.len()));
    // while let Some((select, remain)) = stack.pop() {
    //     println!("[{:?}] < {:?}", cache, stack);
    //     match cache.get(&remain) {
    //         Some(_) => {}
    //         None => match remain.end.cmp(&remain.start) {
    //             Less => {}
    //             Equal => {
    //                 if valid.contains(&chars[select.clone()]) {
    //                     cache.insert(select, 1);
    //                 }
    //             }
    //             Greater => {
    //                 // println!("[{}..{}]", select.start, select.end);
    //                 if valid.contains(&chars[select.clone()]) {
    //                     // at least some summation should be here!
    //                     stack.push((select.clone(), remain.clone()));
    //                     stack.push((
    //                         remain.start..remain.start + 1,
    //                         remain.start + 1..remain.end,
    //                     ));
    //                     stack.push((
    //                         remain.start..remain.start + 2,
    //                         remain.start + 2..remain.end,
    //                     ));
    //                     cache.insert(
    //                         select,
    //                         cache.get(&(remain.start + 1..remain.end)).unwrap()
    //                             + cache.get(&(remain.start + 2..remain.end)).unwrap(),
    //                     );
    //                 }
    //             }
    //         },
    //     }
    // }
    // println!("{:?}", cache);
    // *cache.get(&(1..chars.len())).unwrap()
    // }
    // TLE naive solution
    // pub fn num_decodings(s: String) -> i32 {
    //     let chars: Vec<_> = s.chars().collect();
    //     let mut count = 0;
    //     let mut stack = Vec::new();
    //     stack.push(0);
    //     while let Some(index) = stack.pop() {
    //         match index.cmp(&chars.len()) {
    //             std::cmp::Ordering::Less => match chars[index] {
    //                 '0' => {}
    //                 '1' => {
    //                     stack.push(index + 1);
    //                     stack.push(index + 2);
    //                 }
    //                 '2' => {
    //                     stack.push(index + 1);
    //                     if index + 1 < chars.len() {
    //                         match chars[index + 1] {
    //                             '0' | '1' | '2' | '3' | '4' | '5' | '6' => stack.push(index + 2),
    //                             _ => {}
    //                         }
    //                     }
    //                 }
    //                 _ => {
    //                     stack.push(index + 1);
    //                 }
    //             },
    //             std::cmp::Ordering::Greater => {}
    //             std::cmp::Ordering::Equal => {
    //                 count += 1;
    //             }
    //         }
    //     }
    //     count
    // }
}

fn main() {
    assert_eq!(Solution::num_decodings("12".to_string()), 2);
    assert_eq!(Solution::num_decodings("226".to_string()), 3);
    assert_eq!(Solution::num_decodings("06".to_string()), 0);
    println!(
        "{}",
        Solution::num_decodings("111111111111111111111111111111111111111111111".to_string())
    );
}
