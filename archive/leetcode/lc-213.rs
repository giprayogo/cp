struct Solution;

impl Solution {
    // took hint; but at the end of the day the conclusion is:
    // no avoiding two selections (omitting last or first?)
    pub fn rob(nums: Vec<i32>) -> i32 {
        let with_first = &nums[..nums.len() - 1];
        let no_first = &nums[1..];
        Solution::_rob(with_first)
            .max(Solution::_rob(no_first))
            .max(nums[0])
    }

    fn _rob(nums: &[i32]) -> i32 {
        nums.iter()
            .fold((0, 0), |acc, x| (acc.1, acc.1.max(x + acc.0)))
            .1
    }
}

fn main() {
    assert_eq!(Solution::rob([2, 3, 2].into()), 3);
    assert_eq!(Solution::rob([1, 2, 3, 1].into()), 4);
    assert_eq!(Solution::rob([1, 2, 3].into()), 3);
    assert_eq!(Solution::rob([1].into()), 1);
    assert_eq!(Solution::rob([1, 4].into()), 4);
}
