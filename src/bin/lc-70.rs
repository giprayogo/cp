// use std::collections::HashMap;
use std::iter::successors;

struct Solution;

impl Solution {
    /// 1st attempt (2nd really, 1st one was without memoization): technically a top-down DP
    // pub fn climb_stairs(n: i32) -> i32 {
    //     let mut map = HashMap::new();
    //     map.insert(1, 1);
    //     map.insert(2, 2);

    //     Solution::_climb_stairs(&mut map, n)
    // }
    // fn _climb_stairs(map: &mut HashMap<i32, i32>, n: i32) -> i32 {
    //     match map.get(&n) {
    //         Some(v) => *v,
    //         None => {
    //             let v = Solution::_climb_stairs(map, n - 1) + Solution::_climb_stairs(map, n - 2);
    //             map.insert(n, v);
    //             v
    //         }
    //     }
    // }

    /// after reading solutions: "optimized bottom-up"
    // pub fn climb_stairs(n: i32) -> i32 {
    //     let (mut prev, mut prev_2) = (1, 1); // 1, 0
    //     for _ in 1..n {
    //         let cur = prev + prev_2; // 2
    //         prev_2 = prev; // shift down
    //         prev = cur;
    //     }
    //     prev
    // }

    /// more extreme (neetcode-like; not exactly the same) -> full functional solution
    pub fn climb_stairs(n: i32) -> i32 {
        successors(Some((1, 1)), |prevs| Some((prevs.1, prevs.0 + prevs.1)))
            .nth(n as usize)
            .unwrap()
            .0
    }
}

fn main() {
    assert_eq!(Solution::climb_stairs(2), 2);
    assert_eq!(Solution::climb_stairs(3), 3);
}
