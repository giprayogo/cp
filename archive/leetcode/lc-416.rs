// use std::cmp::Ordering::{Equal, Greater, Less};

use std::collections::HashSet;

struct Solution;

impl Solution {
    // other alternative: more compact DP by neetcode: put row in increasingly larger hash set
    pub fn can_partition(nums: Vec<i32>) -> bool {
        let sum: i32 = nums.iter().sum();
        if sum % 2 != 0 {
            return false;
        }
        let mut dp = HashSet::new();
        dp.insert(0);
        for num in nums {
            let mut _dp = dp.clone();
            for w in dp {
                if num + w == sum / 2 {
                    return true;
                }
                _dp.insert(num + w);
            }
            dp = _dp;
        }
        false
    }

    // better: DP knapsack
    // pub fn can_partition(nums: Vec<i32>) -> bool {
    //     let sum: i32 = nums.iter().sum();
    //     if sum % 2 != 0 {
    //         return false;
    //     }
    //     let ni = nums.len() + 1;
    //     let nj = ((sum / 2) + 1) as usize;
    //     let mut sums = vec![vec![false; nj]; ni];
    //     sums[0][0] = true;

    //     for i in 1..ni {
    //         for j in 0..nj {
    //             let noclude = sums[i - 1][j];
    //             // NOTE: because it can overflow
    //             let include = match j.checked_sub(nums[i - 1] as usize) {
    //                 None => false, // j < 0
    //                 Some(_j) => sums[i - 1][_j],
    //             };
    //             sums[i][j] = noclude || include;
    //         }
    //     }
    //     *sums.last().unwrap().last().unwrap()
    // }

    // pub fn can_partition(nums: Vec<i32>) -> bool {
    //     let sum: i32 = nums.iter().sum();
    //     if sum % 2 != 0 {
    //         return false;
    //     }
    //     // O(2^n): will it TLE? (it did)
    //     let mut config = Vec::new();
    //     Solution::dfs(&nums, &mut config, sum / 2, 0)
    // }

    // fn dfs(nums: &Vec<i32>, config: &mut Vec<i32>, s: i32, i: usize) -> bool {
    //     let sum = config.iter().sum::<i32>();
    //     if sum >= s || i >= nums.len() {
    //         return sum == s;
    //     }
    //     config.push(nums[i]);
    //     let left = Solution::dfs(nums, config, s, i + 1);
    //     config.pop();
    //     let right = Solution::dfs(nums, config, s, i + 1);
    //     left || right
    // }
}

fn main() {
    assert!(!Solution::can_partition(vec![2]));
    assert!(Solution::can_partition(vec![1, 1]));
    assert!(Solution::can_partition(vec![1, 5, 11, 5]));
    assert!(!Solution::can_partition(vec![1, 2, 3, 5]));
    assert!(Solution::can_partition(vec![2, 2, 1, 1]));
}
