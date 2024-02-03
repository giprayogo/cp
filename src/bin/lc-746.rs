// use std::cmp::min;

struct Solution {}

impl Solution {
    // pub fn min_cost_climbing_stairs(cost: Vec<i32>) -> i32 {
    //     let n = cost.len();
    //     let (mut prev_2, mut prev) = (cost[0], cost[1]);
    //     for c in &cost[2..n] {
    //         let cur = c + min(prev_2, prev);
    //         prev_2 = prev;
    //         prev = cur;
    //     }
    //     min(prev_2, prev)
    // }

    /// lesson from neetcode: often simple is better! I need not overcomplicate things
    pub fn min_cost_climbing_stairs(mut cost: Vec<i32>) -> i32 {
        let n = cost.len();
        for i in 2..n {
            cost[i] += cost[i - 1].min(cost[i - 2]);
        }
        cost[n - 1].min(cost[n - 2])
    }
}

fn main() {
    assert_eq!(Solution::min_cost_climbing_stairs([10, 15, 20].into()), 15);
    assert_eq!(
        Solution::min_cost_climbing_stairs([1, 100, 1, 1, 1, 100, 1, 1, 100, 1].into()),
        6
    );
}
