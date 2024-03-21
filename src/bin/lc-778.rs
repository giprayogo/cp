use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashSet};

impl Solution {
    // OK! Just rather slow...
    pub fn swim_in_water(mut grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len() - 1;
        let mut v = HashSet::new();
        let mut h = BinaryHeap::new();
        h.push((Reverse(grid[0][0]), (0usize, 0usize)));
        while let Some((w, (i, j))) = h.pop() {
            if v.contains(&(i, j)) {
                continue;
            }
            if i == n && j == n {
                return w.0;
            }
            v.insert((i, j));
            grid[i][j] = w.0;

            let _i = i.checked_sub(1);
            let _j = j.checked_sub(1);
            let __i = (i < n).then_some(i + 1);
            let __j = (j < n).then_some(j + 1);

            if let Some(_i) = _i {
                let cost = w.0 + 0.max(grid[_i][j] - grid[i][j]);
                h.push((Reverse(cost), (_i, j)));
            }
            if let Some(__i) = __i {
                let cost = w.0 + 0.max(grid[__i][j] - grid[i][j]);
                h.push((Reverse(cost), (__i, j)));
            }
            if let Some(_j) = _j {
                let cost = w.0 + 0.max(grid[i][_j] - grid[i][j]);
                h.push((Reverse(cost), (i, _j)));
            }
            if let Some(__j) = __j {
                let cost = w.0 + 0.max(grid[i][__j] - grid[i][j]);
                h.push((Reverse(cost), (i, __j)));
            }
        }
        unreachable!()
    }
}

struct Solution;

fn main() {
    assert_eq!(Solution::swim_in_water(vec![vec![0, 2], vec![1, 3]]), 3);
    assert_eq!(
        Solution::swim_in_water(vec![
            vec![0, 1, 2, 3, 4],
            vec![24, 23, 22, 21, 5],
            vec![12, 13, 14, 15, 16],
            vec![11, 17, 18, 19, 20],
            vec![10, 9, 8, 7, 6]
        ]),
        16
    );
    assert_eq!(Solution::swim_in_water(vec![vec![3, 2], vec![0, 1]]), 3);
    assert_eq!(
        Solution::swim_in_water(vec![
            vec![10, 12, 4, 6],
            vec![9, 11, 3, 5],
            vec![1, 7, 13, 8],
            vec![2, 0, 15, 14]
        ]),
        14
    );
}
