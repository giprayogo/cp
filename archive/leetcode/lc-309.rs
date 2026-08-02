struct Solution;

impl Solution {
    // TLE yay! means the formalization is probably correct...with bad implementation
    // #[allow(clippy::needless_range_loop)]
    // pub fn max_profit(prices: Vec<i32>) -> i32 {
    //     let n = prices.len();
    //     let mut bs = vec![vec![0; n]; n];

    //     for i in 0..n {
    //         for j in 0..i {
    //             bs[i][j] = prices[i] - prices[j];
    //         }
    //     }
    //     let mut max = 0;
    //     for i in 0..n {
    //         for j in 0..i {
    //             bs[i][j] += {
    //                 if j.checked_sub(2).is_some() {
    //                     let mut max = 0;
    //                     for _i in 0..(j - 1) {
    //                         for _j in 0..(j - 2) {
    //                             max = max.max(bs[_i][_j])
    //                         }
    //                     }
    //                     max
    //                 } else {
    //                     0
    //                 }
    //             };
    //             max = max.max(bs[i][j]);
    //         }
    //     }
    //     max
    // }

    // I was stupid -> muuuuch simpler than that "grid"
    // pub fn max_profit(prices: Vec<i32>) -> i32 {
    //     let n = prices.len();
    //     let mut rowmax = vec![0; n];
    //     let mut max = 0;

    //     for i in 0..n {
    //         for j in 0..i {
    //             // Sell at day i buy at day j
    //             let profit = prices[i] - prices[j]
    //                 // Plus max profit from selling up to 2 days ago (bcoz cooldown)
    //                 + if j.checked_sub(2).is_some() {
    //                     rowmax[j - 2]
    //                 } else {
    //                     0
    //                 };
    //             max = max.max(profit);
    //         }
    //         rowmax[i] = max; // Max possible profit up to day i
    //     }
    //     max
    // }

    // Beaten by neetcode again
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut p1 = 0;
        let mut p2 = 0;

        // i.e. sell just before peak (yesterday) if selling today is less profitable
        for x in 1..prices.len() {
            let temp = p1; // Previous p1
            p1 = p2.max(p1 + prices[x] - prices[x - 1]); // Sell 2 days ago or keep current; choose most profitable
            p2 = p2.max(temp); // Previous cumulative; for checking 2 days ago tommorrow
        }

        p1.max(p2)
    }
}

fn main() {
    assert_eq!(Solution::max_profit(vec![1, 2, 3, 0, 2]), 3);
    assert_eq!(Solution::max_profit(vec![1]), 0);
    assert_eq!(Solution::max_profit(vec![1, 4, 2]), 3);
}
