// use std::collections::{HashMap, HashSet, VecDeque};
use std::collections::{HashMap, HashSet};

impl Solution {
    pub fn network_delay_time(times: Vec<Vec<i32>>, n: i32, k: i32) -> i32 {
        let mut adj_list = HashMap::new();
        let mut s: HashSet<_> = (0..n).collect();
        let mut min_times = vec![i32::MAX; n as usize];
        let k = k - 1;
        for time in times {
            if let [u, v, w] = time[..] {
                let u = u - 1;
                let v = v - 1;
                adj_list.entry(u).or_insert_with(Vec::new).push((w, v));
                s.remove(&(v));
            }
        }
        if s.len() > 1 {
            return -1;
        }
        let mut visited = HashSet::new();
        min_times[k as usize] = 0;

        // correct (maybe) but as-is: TLE!
        fn dfs(
            adj_list: &HashMap<i32, Vec<(i32, i32)>>,
            visited: &mut HashSet<i32>,
            min_times: &mut Vec<i32>,
            n: i32,
            t: i32,
        ) {
            let time = min_times[n as usize].min(t);
            min_times[n as usize] = time;
            if visited.contains(&n) {
                return;
            }
            if let Some(edges) = adj_list.get(&n) {
                for (w, v) in edges.iter() {
                    visited.insert(n);
                    dfs(adj_list, visited, min_times, *v, t + w);
                    visited.remove(&n);
                }
            }
        }
        dfs(&adj_list, &mut visited, &mut min_times, k, 0);
        let mut ret = 0;
        for t in min_times {
            if t != i32::MAX {
                ret = ret.max(t)
            } else {
                return -1;
            }
        }
        ret

        // Lesson: early optimization thinking "it would be more efficient this way"
        // let mut stack = VecDeque::new();
        // stack.push_back((k, k, 0));
        // while let Some((_n, n, w)) = stack.pop_front() {
        //     let time = min_times[n as usize].min(min_times[_n as usize] + w);
        //     min_times[n as usize] = time;
        //     if visited.contains(&n) {
        //         continue;
        //     }
        //     visited.insert(n);
        //     if let Some(edges) = adj_list.get(&n) {
        //         for (w, v) in edges.iter() {
        //             stack.push_back((n, *v, *w));
        //         }
        //     }
        // }
        // println!("{min_times:?}");
        // if visited.len() < n as usize {
        // -1
        // } else {
    }
}
struct Solution;

fn main() {
    assert_eq!(
        Solution::network_delay_time(vec![vec![2, 1, 1], vec![2, 3, 1], vec![3, 4, 1]], 4, 2),
        2
    );
    assert_eq!(Solution::network_delay_time(vec![vec![1, 2, 1]], 2, 1), 1);
    assert_eq!(Solution::network_delay_time(vec![vec![1, 2, 1]], 2, 2), -1);
    assert_eq!(
        Solution::network_delay_time(vec![vec![1, 2, 1], vec![2, 3, 2], vec![1, 3, 4]], 3, 1),
        3
    );
    assert_eq!(
        Solution::network_delay_time(
            vec![
                vec![4, 2, 76],
                vec![1, 3, 79],
                vec![3, 1, 81],
                vec![4, 3, 30],
                vec![2, 1, 47],
                vec![1, 5, 61],
                vec![1, 4, 99],
                vec![3, 4, 68],
                vec![3, 5, 46],
                vec![4, 1, 6],
                vec![5, 4, 7],
                vec![5, 3, 44],
                vec![4, 5, 19],
                vec![2, 3, 13],
                vec![3, 2, 18],
                vec![1, 2, 0],
                vec![5, 1, 25],
                vec![2, 5, 58],
                vec![2, 4, 77],
                vec![5, 2, 74]
            ],
            5,
            3
        ),
        59
    );
}
