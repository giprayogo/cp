#[allow(unused)]
struct Solution;

#[allow(unused)]
impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let mut out = vec![1; nums.len()];
        let mut c = 1;
        for (n, o) in nums.iter().zip(out.iter_mut()) {
            *o *= c;
            c *= n;
        }
        c = 1;
        for (n, o) in nums.iter().rev().zip(out.iter_mut().rev()) {
            *o *= c;
            c *= n;
        }
        out
    }
}

fn main() {}

#[test]
fn test_solution() {
    let nums = vec![1, 2, 3, 4];
    assert_eq!(Solution::product_except_self(nums), vec![24, 12, 8, 6]);
    let nums = vec![-1, 1, 0, -3, 3];
    assert_eq!(Solution::product_except_self(nums), vec![0, 0, 9, 0, 0]);
}
    