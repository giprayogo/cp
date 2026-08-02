use std::collections::HashMap;

struct Solution;

impl Solution {
    // DFS with memoization; Not tLE! But not the best...
    pub fn find_target_sum_ways_dfs(nums: Vec<i32>, target: i32) -> i32 {
        let mut memo = HashMap::new();

        fn dfs(
            memo: &mut HashMap<(usize, i32), i32>,
            nums: &Vec<i32>,
            target: i32,
            i: usize,
            n: i32,
        ) -> i32 {
            match memo.get(&(i, n)) {
                Some(v) => *v,
                None => {
                    if i >= nums.len() {
                        if n == target {
                            1
                        } else {
                            0
                        }
                    } else {
                        let _n = n - nums[i];
                        let __n = n + nums[i];
                        let res = dfs(memo, nums, target, i + 1, _n)
                            + dfs(memo, nums, target, i + 1, __n);
                        memo.insert((i, n), res);
                        res
                    }
                }
            }
        }

        dfs(&mut memo, &nums, target, 0, 0)
    }

    // DP
    #[allow(clippy::needless_range_loop)]
    pub fn find_target_sum_ways_dp(nums: Vec<i32>, target: i32) -> i32 {
        let sum: i32 = nums.iter().sum();
        if sum < target.abs() {
            return 0;
        }

        let mut dp = vec![0; sum as usize * 2 + 1];
        dp[sum as usize] = 1;

        for n in nums {
            let n = n as usize;
            let mut _dp = vec![0; dp.len()];

            for i in 0..dp.len() {
                let _i = i.checked_sub(n);
                let __i = (i + n < dp.len()).then_some(i + n);
                if let Some(___i) = _i {
                    _dp[i] += dp[___i];
                }
                if let Some(___i) = __i {
                    _dp[i] += dp[___i];
                }
            }
            dp = _dp;
        }
        dp[(sum + target) as usize]
    }

    // observation: the tree is always symmmetric; I can "fold" in half... right?
    #[allow(clippy::needless_range_loop)]
    pub fn find_target_sum_ways_dp_2(nums: Vec<i32>, mut target: i32) -> i32 {
        let sum: i32 = nums.iter().sum();
        target = target.abs();
        if sum < target {
            return 0;
        }

        let mut dp = vec![0; sum as usize + 1];
        dp[0] = 1;

        for n in nums {
            let mut _dp = vec![0; dp.len()];
            let n = n as usize;

            for i in 0.._dp.len() {
                let _i = (i as isize - n as isize).unsigned_abs();
                if _i < dp.len() {
                    _dp[i] += dp[_i];
                }
                if i + n < dp.len() {
                    _dp[i] += dp[i + n];
                }
            }
            dp = _dp;
        }
        dp[target as usize]
    }
}

fn main() {
    // DFS
    assert_eq!(
        Solution::find_target_sum_ways_dfs(vec![1, 1, 1, 1, 1], 3),
        5
    );
    assert_eq!(
        Solution::find_target_sum_ways_dfs(vec![1, 1, 1, 1, 1], -3),
        5
    );
    assert_eq!(Solution::find_target_sum_ways_dfs(vec![1], 1), 1);
    assert_eq!(Solution::find_target_sum_ways_dfs(vec![1, 1], 0), 2);

    // DP
    assert_eq!(Solution::find_target_sum_ways_dp(vec![1, 1], 0), 2);
    assert_eq!(Solution::find_target_sum_ways_dp(vec![1, 1, 1, 1, 1], 3), 5);
    assert_eq!(
        Solution::find_target_sum_ways_dp(vec![1, 1, 1, 1, 1], -3),
        5
    );
    assert_eq!(Solution::find_target_sum_ways_dp(vec![1], 1), 1);
    assert_eq!(Solution::find_target_sum_ways_dp(vec![100], -200), 0);

    // Symmetric DP
    assert_eq!(Solution::find_target_sum_ways_dp_2(vec![1, 1], 0), 2);
    assert_eq!(
        Solution::find_target_sum_ways_dp_2(vec![1, 1, 1, 1, 1], 3),
        5
    );
    assert_eq!(
        Solution::find_target_sum_ways_dp_2(vec![1, 1, 1, 1, 1], -3),
        5
    );
    assert_eq!(Solution::find_target_sum_ways_dp_2(vec![1], 1), 1);
    assert_eq!(Solution::find_target_sum_ways_dp_2(vec![100], -200), 0);
}
