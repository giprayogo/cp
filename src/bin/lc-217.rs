use std::collections::HashSet;

struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        let mut k = HashSet::new();
        for num in nums {
            match k.contains(&num) {
                true => return true,
                false => {k.insert(num);},
            }
        }
        false
    }
}

fn main() {}

#[test]
fn test_solution() {
    let nums = vec![1, 2, 3, 1];
    assert_eq!(Solution::contains_duplicate(nums), true);
    let nums = vec![1, 2, 3, 4];
    assert_eq!(Solution::contains_duplicate(nums), false);
    let nums = vec![1, 1, 1, 3, 3, 4, 3, 2, 4, 2];
    assert_eq!(Solution::contains_duplicate(nums), true);
}