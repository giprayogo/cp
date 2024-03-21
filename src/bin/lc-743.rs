// use std::collections::{HashMap, HashSet, VecDeque};
// use std::collections::{BTreeSet, HashMap, HashSet};
use std::cmp::Reverse;
use std::collections::{BTreeSet, BinaryHeap, HashMap, HashSet};

impl Solution {
    pub fn network_delay_time_1(times: Vec<Vec<i32>>, n: i32, k: i32) -> i32 {
        // let mut adj_list = HashMap::new();
        let n = n as usize;
        let mut adj_list = vec![BTreeSet::new(); n]; // note: actually hashmap a bit faster??
                                                     // let mut s: HashSet<_> = (0..n).collect(); // NOTE: not as useful
        let k = (k - 1) as usize;
        for time in times {
            if let [u, v, w] = time[..] {
                let u = (u - 1) as usize;
                let v = (v - 1) as usize;
                // adj_list.entry(u).or_insert_with(Vec::new).push((w, v));
                // adj_list
                //     .entry(u)
                //     .or_insert_with(BTreeSet::new)
                //     .insert((w, v));
                adj_list[u].insert((w, v));
                // s.remove(&(v));
            }
        }
        // if s.len() > 1 {
        //     return -1;
        // }

        // correct (maybe) but as-is: TLE!
        // update: no longer TLE but slooow -> time for dp?
        // Much better with BTreeSet (my hunch is correct about where it's slow)
        fn dfs(
            // adj_list: &HashMap<i32, Vec<(i32, i32)>>,
            // adj_list: &HashMap<i32, BTreeSet<(i32, i32)>>,
            adj_list: &Vec<BTreeSet<(i32, usize)>>,
            // visited: &mut HashSet<i32>,
            visited: &mut HashSet<usize>,
            min_times: &mut Vec<i32>,
            // n: i32,
            n: usize,
            t: i32,
        ) {
            // if visited.contains(&n) || t >= min_times[n as usize] {
            if visited.contains(&n) || t >= min_times[n] {
                return;
            }
            // min_times[n as usize] = t;
            // if let Some(edges) = adj_list.get(&n) {
            //     for (w, v) in edges.iter() {
            min_times[n] = t;
            for (w, v) in adj_list[n].iter() {
                visited.insert(n);
                dfs(adj_list, visited, min_times, *v, t + w);
                visited.remove(&n);
            }
            // }
        }
        let mut min_times = vec![i32::MAX; n];
        let mut visited = HashSet::new();
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

    // neetcode: can just use normal MST algorithms...remember I need to reach -all-
    // and their minimum time; also for prim's the tree is ~ visited list in DFS
    pub fn network_delay_time_2(times: Vec<Vec<i32>>, n: i32, k: i32) -> i32 {
        let k = (k - 1) as usize;
        let mut adj_list = HashMap::new();
        for time in times {
            if let [u, v, w] = time[..] {
                let u = (u - 1) as usize;
                let v = (v - 1) as usize;
                adj_list
                    .entry(u)
                    .or_insert_with(Vec::new)
                    .push((Reverse(w), v));
            }
        }

        let mut h = BinaryHeap::new();
        let mut f = HashSet::new();
        h.push((Reverse(0), k));
        let mut __w = 0;
        while let Some((w, n)) = h.pop() {
            if f.contains(&n) {
                continue;
            }
            __w = w.0;
            f.insert(n);
            if let Some(edges) = adj_list.get(&n) {
                for (_w, _n) in edges.iter() {
                    h.push((Reverse(w.0 + _w.0), *_n));
                }
            }
        }

        if f.len() < n as usize {
            -1
        } else {
            __w
        }
    }
}
struct Solution;

fn main() {
    for f in [
        Solution::network_delay_time_1,
        Solution::network_delay_time_2,
    ] {
        assert_eq!(
            f(vec![vec![2, 1, 1], vec![2, 3, 1], vec![3, 4, 1]], 4, 2),
            2
        );
        assert_eq!(f(vec![vec![1, 2, 1]], 2, 1), 1);
        assert_eq!(f(vec![vec![1, 2, 1]], 2, 2), -1);
        assert_eq!(
            f(vec![vec![1, 2, 1], vec![2, 3, 2], vec![1, 3, 4]], 3, 1),
            3
        );
        assert_eq!(
            f(
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
}
