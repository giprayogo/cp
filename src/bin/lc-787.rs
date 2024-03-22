// use std::cmp::Reverse;
// use std::collections::{BinaryHeap, HashMap, HashSet};
// use std::collections::{BTreeSet, HashMap, HashSet, VecDeque};
use std::collections::HashMap;

impl Solution {
    // neetcode: even simpler than the BFS; you know you can just update array mult times!
    // no need to change graph into adj_list blah blash!
    // Lesson: don't over-complicate things...
    // Also related to Belmann-Ford, but cutoff to k iteration
    pub fn find_cheapest_price_2(
        n: i32,
        flights: Vec<Vec<i32>>,
        src: i32,
        dst: i32,
        k: i32,
    ) -> i32 {
        let n = n as usize;
        let src = src as usize;
        let dst = dst as usize;
        let mut prices = vec![i32::MAX; n];
        prices[src] = 0;
        for _ in 0..(k + 1) {
            let mut _prices = prices.clone();
            for flight in flights.iter() {
                if let [i, j, w] = flight[..] {
                    if prices[i as usize] != i32::MAX {
                        _prices[j as usize] = _prices[j as usize].min(prices[i as usize] + w);
                    }
                }
            }
            prices = _prices;
        }
        if prices[dst] == i32::MAX {
            -1
        } else {
            prices[dst]
        }
    }

    pub fn find_cheapest_price(n: i32, flights: Vec<Vec<i32>>, src: i32, dst: i32, k: i32) -> i32 {
        let mut adj_list = HashMap::with_capacity(n as usize);
        for flight in flights {
            if let [i, j, w] = flight[..] {
                adj_list.entry(i).or_insert_with(Vec::new).push((w, j));
            }
        }

        // Fixed depth BFS
        // Lesson: when you learn new thing... you can get stuck in "having to use shiny thing" mindset.
        // Solution is simpler than you first thought!
        let mut stack = HashMap::from([(src, 0)]);
        let mut res = i32::MAX;
        for _ in 0..(k + 1) {
            let mut _stack = HashMap::new();
            for (n, w) in stack.into_iter() {
                if n == dst {
                    res = res.min(w);
                }
                if let Some(edges) = adj_list.get(&n) {
                    for (_w, _n) in edges {
                        _stack
                            .entry(*_n)
                            .and_modify(|x: &mut i32| *x = (*x).min(w + _w))
                            .or_insert(w + _w);
                    }
                }
            }
            stack = _stack;
        }
        for (n, w) in stack.into_iter() {
            if n == dst {
                res = res.min(w);
            }
        }
        if res != i32::MAX {
            res
        } else {
            -1
        }
        // More correct but TLE! > memoization
        // fn dfs(
        //     adj_list: &HashMap<i32, BTreeSet<(i32, i32)>>,
        //     f: &mut HashSet<i32>,
        //     memo: &mut HashMap<(i32, i32), Option<i32>>,
        //     w: i32,
        //     n: i32,
        //     k: i32,
        //     dst: i32,
        // ) -> Option<i32> {
        //     // memo not always the cheapest...
        //     // also just because it's None doesn't mean it can't update... so the memo is useless?
        //     if let Some(price) = memo.get(&(n, k)) {
        //         if price.is_none() {
        //             return *price;
        //         }
        //     }
        //     if f.contains(&n) || f.len() > (k + 1) as usize {
        //         return None;
        //     }
        //     if n == dst {
        //         return Some(w);
        //     }
        //     f.insert(n);
        //     let mut price = i32::MAX;
        //     if let Some(edges) = adj_list.get(&n) {
        //         for (_w, _n) in edges.iter() {
        //             if let Some(_price) = dfs(adj_list, f, memo, w + _w, *_n, k, dst) {
        //                 price = price.min(_price);
        //             }
        //         }
        //     }
        //     f.remove(&n);
        //     if price != i32::MAX {
        //         memo.insert((n, k), Some(price));
        //         Some(price)
        //     } else {
        //         memo.insert((n, k), None);
        //         None
        //     }
        // }
        // if let Some(price) = dfs(
        //     &adj_list,
        //     &mut HashSet::new(),
        //     &mut HashMap::new(),
        //     0,
        //     src,
        //     k,
        //     dst,
        // ) {
        //     price
        // } else {
        //     -1
        // }
        // Note: modified Dijkstra doesn't work here... or at least i don't know how!
        // let mut f = HashSet::with_capacity(n as usize);
        // work but now TLE..
        // let mut h = BinaryHeap::new();
        // h.push((Reverse(0), -1, src));
        // while let Some((w, _k, n)) = h.pop() {
        //     println!("{},{},{}", w.0, _k, n);
        //     // this: good question design: should not simply quit on continues
        //     // the cheapest one is not always possible because of k
        //     // if f.contains(&n) || _k > k {
        //     // some loop get prioritized? no I got stuck in a loop...probably DFS is the correct approach here....
        //     if _k > k {
        //         continue;
        //     }
        //     if n == dst {
        //         return w.0;
        //     }
        //     // f.insert(n);
        //     if let Some(edges) = adj_list.get(&n) {
        //         for (_w, _n) in edges.iter() {
        //             h.push((Reverse(w.0 + _w), _k + 1, *_n));
        //         }
        //     }
        // }
        // -1
    }
}

struct Solution;

fn main() {
    for f in [
        Solution::find_cheapest_price,
        Solution::find_cheapest_price_2,
    ] {
        assert_eq!(
            f(
                5,
                vec![
                    vec![0, 1, 5],
                    vec![1, 2, 5],
                    vec![0, 3, 2],
                    vec![3, 1, 2],
                    vec![1, 4, 1],
                    vec![4, 2, 1]
                ],
                0,
                2,
                2
            ),
            7
        );
        assert_eq!(
            f(
                3,
                vec![vec![0, 1, 100], vec![1, 2, 100], vec![0, 2, 500]],
                0,
                2,
                1
            ),
            200
        );
        assert_eq!(
            f(
                3,
                vec![vec![0, 1, 100], vec![1, 2, 100], vec![0, 2, 500]],
                0,
                2,
                0
            ),
            500
        );
        assert_eq!(
            f(
                4,
                vec![vec![0, 1, 1], vec![0, 2, 5], vec![1, 2, 1], vec![2, 3, 1]],
                0,
                3,
                1
            ),
            6
        );
        assert_eq!(
            f(
                13,
                vec![
                    vec![11, 12, 74],
                    vec![1, 8, 91],
                    vec![4, 6, 13],
                    vec![7, 6, 39],
                    vec![5, 12, 8],
                    vec![0, 12, 54],
                    vec![8, 4, 32],
                    vec![0, 11, 4],
                    vec![4, 0, 91],
                    vec![11, 7, 64],
                    vec![6, 3, 88],
                    vec![8, 5, 80],
                    vec![11, 10, 91],
                    vec![10, 0, 60],
                    vec![8, 7, 92],
                    vec![12, 6, 78],
                    vec![6, 2, 8],
                    vec![4, 3, 54],
                    vec![3, 11, 76],
                    vec![3, 12, 23],
                    vec![11, 6, 79],
                    vec![6, 12, 36],
                    vec![2, 11, 100],
                    vec![2, 5, 49],
                    vec![7, 0, 17],
                    vec![5, 8, 95],
                    vec![3, 9, 98],
                    vec![8, 10, 61],
                    vec![2, 12, 38],
                    vec![5, 7, 58],
                    vec![9, 4, 37],
                    vec![8, 6, 79],
                    vec![9, 0, 1],
                    vec![2, 3, 12],
                    vec![7, 10, 7],
                    vec![12, 10, 52],
                    vec![7, 2, 68],
                    vec![12, 2, 100],
                    vec![6, 9, 53],
                    vec![7, 4, 90],
                    vec![0, 5, 43],
                    vec![11, 2, 52],
                    vec![11, 8, 50],
                    vec![12, 4, 38],
                    vec![7, 9, 94],
                    vec![2, 7, 38],
                    vec![3, 7, 88],
                    vec![9, 12, 20],
                    vec![12, 0, 26],
                    vec![10, 5, 38],
                    vec![12, 8, 50],
                    vec![0, 2, 77],
                    vec![11, 0, 13],
                    vec![9, 10, 76],
                    vec![2, 6, 67],
                    vec![5, 6, 34],
                    vec![9, 7, 62],
                    vec![5, 3, 67]
                ],
                10,
                1,
                10
            ),
            -1
        );
        assert_eq!(
            f(
                10,
                vec![
                    vec![3, 4, 4],
                    vec![2, 5, 6],
                    vec![4, 7, 10],
                    vec![9, 6, 5],
                    vec![7, 4, 4],
                    vec![6, 2, 10],
                    vec![6, 8, 6],
                    vec![7, 9, 4],
                    vec![1, 5, 4],
                    vec![1, 0, 4],
                    vec![9, 7, 3],
                    vec![7, 0, 5],
                    vec![6, 5, 8],
                    vec![1, 7, 6],
                    vec![4, 0, 9],
                    vec![5, 9, 1],
                    vec![8, 7, 3],
                    vec![1, 2, 6],
                    vec![4, 1, 5],
                    vec![5, 2, 4],
                    vec![1, 9, 1],
                    vec![7, 8, 10],
                    vec![0, 4, 2],
                    vec![7, 2, 8]
                ],
                6,
                0,
                7
            ),
            14
        );
        println!(
            "{}",
            f(
                17,
                vec![
                    vec![0, 12, 28],
                    vec![5, 6, 39],
                    vec![8, 6, 59],
                    vec![13, 15, 7],
                    vec![13, 12, 38],
                    vec![10, 12, 35],
                    vec![15, 3, 23],
                    vec![7, 11, 26],
                    vec![9, 4, 65],
                    vec![10, 2, 38],
                    vec![4, 7, 7],
                    vec![14, 15, 31],
                    vec![2, 12, 44],
                    vec![8, 10, 34],
                    vec![13, 6, 29],
                    vec![5, 14, 89],
                    vec![11, 16, 13],
                    vec![7, 3, 46],
                    vec![10, 15, 19],
                    vec![12, 4, 58],
                    vec![13, 16, 11],
                    vec![16, 4, 76],
                    vec![2, 0, 12],
                    vec![15, 0, 22],
                    vec![16, 12, 13],
                    vec![7, 1, 29],
                    vec![7, 14, 100],
                    vec![16, 1, 14],
                    vec![9, 6, 74],
                    vec![11, 1, 73],
                    vec![2, 11, 60],
                    vec![10, 11, 85],
                    vec![2, 5, 49],
                    vec![3, 4, 17],
                    vec![4, 9, 77],
                    vec![16, 3, 47],
                    vec![15, 6, 78],
                    vec![14, 1, 90],
                    vec![10, 5, 95],
                    vec![1, 11, 30],
                    vec![11, 0, 37],
                    vec![10, 4, 86],
                    vec![0, 8, 57],
                    vec![6, 14, 68],
                    vec![16, 8, 3],
                    vec![13, 0, 65],
                    vec![2, 13, 6],
                    vec![5, 13, 5],
                    vec![8, 11, 31],
                    vec![6, 10, 20],
                    vec![6, 2, 33],
                    vec![9, 1, 3],
                    vec![14, 9, 58],
                    vec![12, 3, 19],
                    vec![11, 2, 74],
                    vec![12, 14, 48],
                    vec![16, 11, 100],
                    vec![3, 12, 38],
                    vec![12, 13, 77],
                    vec![10, 9, 99],
                    vec![15, 13, 98],
                    vec![15, 12, 71],
                    vec![1, 4, 28],
                    vec![7, 0, 83],
                    vec![3, 5, 100],
                    vec![8, 9, 14],
                    vec![15, 11, 57],
                    vec![3, 6, 65],
                    vec![1, 3, 45],
                    vec![14, 7, 74],
                    vec![2, 10, 39],
                    vec![4, 8, 73],
                    vec![13, 5, 77],
                    vec![10, 0, 43],
                    vec![12, 9, 92],
                    vec![8, 2, 26],
                    vec![1, 7, 7],
                    vec![9, 12, 10],
                    vec![13, 11, 64],
                    vec![8, 13, 80],
                    vec![6, 12, 74],
                    vec![9, 7, 35],
                    vec![0, 15, 48],
                    vec![3, 7, 87],
                    vec![16, 9, 42],
                    vec![5, 16, 64],
                    vec![4, 5, 65],
                    vec![15, 14, 70],
                    vec![12, 0, 13],
                    vec![16, 14, 52],
                    vec![3, 10, 80],
                    vec![14, 11, 85],
                    vec![15, 2, 77],
                    vec![4, 11, 19],
                    vec![2, 7, 49],
                    vec![10, 7, 78],
                    vec![14, 6, 84],
                    vec![13, 7, 50],
                    vec![11, 6, 75],
                    vec![5, 10, 46],
                    vec![13, 8, 43],
                    vec![9, 10, 49],
                    vec![7, 12, 64],
                    vec![0, 10, 76],
                    vec![5, 9, 77],
                    vec![8, 3, 28],
                    vec![11, 9, 28],
                    vec![12, 16, 87],
                    vec![12, 6, 24],
                    vec![9, 15, 94],
                    vec![5, 7, 77],
                    vec![4, 10, 18],
                    vec![7, 2, 11],
                    vec![9, 5, 41]
                ],
                13,
                4,
                13
            )
        )
    }
}
