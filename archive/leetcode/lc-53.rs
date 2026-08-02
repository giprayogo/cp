struct Solution;

impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let mut sum = 0;
        let mut max = i32::MIN;
        for n in nums {
            sum = n.max(sum + n);
            max = max.max(sum);
        }
        max
    }
}

fn main() {
    assert_eq!(
        Solution::max_sub_array(vec![-2, 1, -3, 4, -1, 2, 1, -5, 4]),
        6
    );
    assert_eq!(Solution::max_sub_array(vec![1]), 1);
    assert_eq!(Solution::max_sub_array(vec![5, 4, -1, 7, 8]), 23);
}
