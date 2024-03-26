impl Solution {
    // NOTE: but why?
    pub fn single_number(nums: Vec<i32>) -> i32 {
        let mut n = 0;
        for num in nums {
            n ^= num;
        }
        n
    }
    pub fn single_number_2(nums: Vec<i32>) -> i32 {
        nums.into_iter().fold(0, |acc, x| acc ^ x)
    }
}

struct Solution;

fn main() {
    for f in [Solution::single_number, Solution::single_number_2] {
        assert_eq!(f(vec![2, 2, 1]), 1);
        assert_eq!(f(vec![4, 1, 2, 1, 2]), 4);
        assert_eq!(f(vec![1]), 1);
    }
}
