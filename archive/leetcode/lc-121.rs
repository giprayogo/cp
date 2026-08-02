fn main() {}

#[allow(unused)]
struct Solution;

#[allow(unused)]
impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut min = i32::MAX;
        let mut gap = 0;
        for p in prices {
            // if p < min {
            //     min = p;
            // }
            min = min.min(p);
            gap = gap.max(p - min);
        }
        gap
    }
}

#[test]
fn test_solution() {
    assert_eq!(Solution::max_profit(vec![7, 1, 5, 3, 6, 4]), 5);
    assert_eq!(Solution::max_profit(vec![7, 6, 4, 3, 1]), 0);
}

