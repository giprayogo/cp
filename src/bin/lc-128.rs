use std::collections::{BinaryHeap, HashSet};

#[allow(unused)]
struct Solution;

#[allow(unused)]
impl Solution {
    // neetcode style check in
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        let h: HashSet<_> = nums.into_iter().collect();
        let mut max_run = 0;
        for e in h.iter() {
            let mut run = 0;
            if !h.contains(&(*e - 1)) && h.contains(e) {
                run += 1;
                let mut f = e + 1;
                while h.contains(&f) {
                    f += 1;
                    run += 1;
                }
                if run > max_run {
                    max_run = run;
                }
            }
        }
        max_run
    }
    // technically n log n? but pass and fast :)
    // pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
    //     let nl = nums.len();
    //     match nl {
    //         0 => return 0,
    //         1 => return 1,
    //         _ => {}
    //     }

    //     let mut h = BinaryHeap::new();
    //     h.reserve_exact(nl);
    //     for n in nums {
    //         h.push(n);
    //     }

    //     let mut max_run = 1;
    //     let mut run = 1;
    //     let mut prev_e = h.pop().unwrap();
    //     while let Some(e) = h.pop() {
    //         match e {
    //             v if v == (prev_e - 1) => {
    //                 run += 1;
    //             }
    //             v if v == prev_e => {}
    //             _ => {
    //                 if run > max_run {
    //                     max_run = run;
    //                 }
    //                 run = 1;
    //             }
    //         }
    //         prev_e = e;
    //     }
    //     if run > max_run {
    //         max_run = run;
    //     }
    //     max_run
    // }
}

fn main() {}

#[test]
fn test_solution() {
    let nums = [100, 4, 200, 1, 3, 2];
    assert_eq!(Solution::longest_consecutive(nums.into()), 4);
    let nums = [0, 3, 7, 2, 5, 8, 4, 6, 0, 1];
    assert_eq!(Solution::longest_consecutive(nums.into()), 9);
    let nums = [1, 2, 0, 1];
    assert_eq!(Solution::longest_consecutive(nums.into()), 3);
}
