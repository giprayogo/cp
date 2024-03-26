impl Solution {
    pub fn missing_number(nums: Vec<i32>) -> i32 {
        let mut x = 0;
        x ^= nums.len() as i32;
        for (i, n) in nums.into_iter().enumerate() {
            x ^= n;
            x ^= i as i32;
        }
        x
    }
    // one fold
    pub fn missing_number_2(nums: Vec<i32>) -> i32 {
        nums.iter()
            .enumerate()
            .fold(nums.len() as i32, |acc, (i, x)| acc ^ x ^ i as i32)
    }
}

struct Solution;

fn main() {
    for f in [Solution::missing_number, Solution::missing_number_2] {
        assert_eq!(f(vec![3, 0, 1]), 2);
        assert_eq!(f(vec![0, 1]), 2);
        assert_eq!(f(vec![9, 6, 4, 2, 3, 5, 7, 0, 1]), 8);
    }
}
