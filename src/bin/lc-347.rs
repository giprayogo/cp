use std::collections::{HashMap, BinaryHeap};

#[allow(unused)]
struct Solution;

#[allow(unused)]
impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut h = HashMap::new();
        for n in nums {
            h.entry(n).and_modify(|e| *e += 1).or_insert(1);
        }
        let mut b = BinaryHeap::new();
        for (k, v) in h {
            b.push((v, k));
        }
        let mut v = Vec::with_capacity(k as usize);
        for _ in 0..k {
            v.push(b.pop().unwrap().1);
        }
        v
    }
}

fn main() {}

#[test]
fn test_solution() {
    let nums = vec![1, 1, 1, 2, 2, 3];
    assert_eq!(Solution::top_k_frequent(nums, 2), vec![1, 2]);
    let nums = vec![1];
    assert_eq!(Solution::top_k_frequent(nums, 1), vec![1]);
}
