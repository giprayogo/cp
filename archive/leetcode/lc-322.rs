// use std::collections::HashMap;

struct Solution;

impl Solution {
    // DP here
    pub fn coin_change(mut coins: Vec<i32>, amount: i32) -> i32 {
        if amount == 0 {
            return 0;
        }

        coins.sort(); // NOTE: work without sort: a bit slower on average but less memory
        let mut ns: Vec<Option<i32>> = vec![None; (amount + 1) as usize];
        ns[0] = Some(0);
        for i in 1..ns.len() {
            // NOTE: cf. neetcode: I don't need dedicated mean; just compare and update repeatedly!
            // less magic...
            let min_amount = coins
                .iter()
                .map(|coin| match i.checked_sub(*coin as usize) {
                    Some(j) => ns[j].map(|v| v + 1),
                    None => None, // OOB
                })
                .filter(|x| x.is_some())
                .min()
                .unwrap_or(None);
            ns[i] = min_amount;
        }

        ns[amount as usize].unwrap_or(-1)
    }

    // I do love DFS for some reasons...
    // pub fn coin_change(mut coins: Vec<i32>, amount: i32) -> i32 {
    //     coins.sort();
    //     let mut ns = HashMap::new();

    //     let mut stack: Vec<(i32, i32)> = Vec::new();
    //     stack.push((0, 0));

    //     while let Some((n, a)) = stack.pop() {
    //         println!("amount: {} ({} coins)", n, a);
    //         if let Some(v) = ns.get(&amount) {
    //             if a >= *v {
    //                 continue;
    //             }
    //         }

    //         ns.entry(n).and_modify(|e| *e = a.min(*e)).or_insert(a);
    //         let _a = a + 1;
    //         for coin in coins.iter() {
    //             if let Some(v) = n.checked_add(*coin) {
    //                 if v <= amount {
    //                     stack.push((v, _a))
    //                 }
    //             };
    //         }
    //     }
    //     *ns.get(&amount).unwrap_or(&-1)
    // }
}

fn main() {
    assert_eq!(Solution::coin_change(vec![1, 2, 5], 11), 3);
    assert_eq!(Solution::coin_change(vec![2], 3), -1);
    assert_eq!(Solution::coin_change(vec![1], 0), 0);
    assert_eq!(Solution::coin_change(vec![1, 2147483647], 2), 2);
    assert_eq!(Solution::coin_change(vec![186, 419, 83, 408], 6249), 20); // REALLY SLOW WITH DFS
}
