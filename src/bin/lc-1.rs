use std::{collections::{HashMap, HashSet}, vec};

struct Solution;

#[allow(unused)]
impl Solution {
    // hash map
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut h = HashMap::new();
        for (i, e) in nums.iter().enumerate() {
            let c = target - e;
            match h.contains_key(&c) {
                true => {
                    return vec![h[&c] as i32, i as i32];
                },
                false => {
                    h.insert(e, i);
                }
            }
        }
        unreachable!()
    }
    // hash set 
    // pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    //     let mut h = HashSet::new();
    //     for (i, num) in nums.iter().enumerate() {
    //         let c = target - num;
    //         match h.contains(&c) {
    //             true => {
    //                 let j = nums.iter().position(|&x| x == c).unwrap();
    //                 return vec![j as i32, i as i32];
    //             }
    //             false => {h.insert(num);},
    //         }
    //     }
    //     vec![]
    // }
    // simple O(n2) solution
    // pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    //     for i in 0..nums.len() {
    //         for j in (i+1)..nums.len() {
    //             if nums[i] + nums[j] == target {
    //                 return vec![i as i32, j as i32];
    //             }
    //         }
    //     }
    //     vec![]
    // }
}

fn main() {}

#[test]
fn test_solution() {
    assert_eq!(
        Solution::two_sum(vec![2, 7, 11, 15], 9)
            .into_iter()
            .collect::<HashSet<i32>>(),
        HashSet::from([0, 1])
    );
    assert_eq!(
        Solution::two_sum(vec![3, 2, 4], 6)
            .into_iter()
            .collect::<HashSet<i32>>(),
        HashSet::from([1, 2])
    );
    assert_eq!(
        Solution::two_sum(vec![3, 3], 6)
            .into_iter()
            .collect::<HashSet<i32>>(),
        HashSet::from([0, 1])
    );
    assert_eq!(
        Solution::two_sum(vec![-2, 6, 11, 15], 9)
            .into_iter()
            .collect::<HashSet<i32>>(),
        HashSet::from([0, 2])
    );
}
