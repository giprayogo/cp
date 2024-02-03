// use std::cmp::Ordering;
struct Solution;

impl Solution {
    // pub fn rob(mut nums: Vec<i32>) -> i32 {
    //     match nums.len().cmp(&3) {
    //         Ordering::Less => nums.into_iter().max().unwrap(),
    //         Ordering::Equal => nums[1].max(nums[0] + nums[2]),
    //         Ordering::Greater => {
    //             nums[2] += nums[0];
    //             for i in 3..nums.len() {
    //                 nums[i] += nums[i - 2].max(nums[i - 3]);
    //             }
    //             nums[nums.len() - 1].max(nums[nums.len() - 2])
    //         }
    //     }
    // }

    /// neetcode: it is actually possible to fit this problem into the "standard pattern"
    /// Also it is not so much as "actual amount possible to be taken including this point",
    /// but rather "amount possible taken up-to this point"
    /* pub fn rob(nums: Vec<i32>) -> i32 {
        let mut prev = 0;
        let mut prev_2 = 0;
        for num in &nums {
            let current = prev.max(num + prev_2);
            prev_2 = prev;
            prev = current;
        }
        prev
    } */

    // functional variant of above
    pub fn rob(nums: Vec<i32>) -> i32 {
        nums.iter()
            .fold((0, 0), |acc, x| (acc.1, acc.1.max(x + acc.0)))
            .1
    }
}

fn main() {
    assert_eq!(Solution::rob([1, 2, 3, 1].into()), 4);
    assert_eq!(Solution::rob([2, 7, 9, 3, 1].into()), 12);
}
