struct Solution;

impl Solution {
    // Accepted but relatively slow.. are there other obvious thing I missed?
    pub fn change(amount: i32, coins: Vec<i32>) -> i32 {
        // let ni = coins.len();
        let nj = amount as usize + 1;

        // let mut dp = vec![vec![0; nj]; ni];
        // coins.sort();  // NOT NECESSARY!

        // NOTE: I don't like the triple loop structure
        // for i in 0..ni {
        //     dp[i][0] = 1;
        //     for j in 1..nj {
        //         // for _i in 0..=i {
        //         //     dp[i][j] += if let Some(_j) = j.checked_sub(coins[_i] as usize) {
        //         //         dp[_i][_j]
        //         //     } else {
        //         //         0
        //         //     }
        //         // }
        //         // Neetcode: I was stupid: row directly above is equal to this whatever loop sum
        //         dp[i][j] += if let Some(_i) = i.checked_sub(1) {
        //             dp[_i][j]
        //         } else {
        //             0
        //         } + if let Some(_j) = j.checked_sub(coins[i] as usize) {
        //             dp[i][_j]
        //         } else {
        //             0
        //         }
        //     }
        // }

        // Also because now I only need the previous row, I can make it more memory-efficient
        let mut dp = vec![0; nj];
        dp[0] = 1;

        for c in coins {
            for j in c as usize..nj {
                // if let Some(_j) = j.checked_sub(c as usize) {
                //     dp[j] += dp[_j];
                // }
                // Optimization (or is the compiler smart enough?): I don't need to count the first few!
                dp[j] += dp[j - c as usize];
            }
        }

        // *dp.last().unwrap().last().unwrap()
        *dp.last().unwrap()
    }
}

fn main() {
    assert_eq!(Solution::change(5, vec![1, 2, 5]), 4);
    assert_eq!(Solution::change(3, vec![2]), 0);
    assert_eq!(Solution::change(10, vec![10]), 1);
}
