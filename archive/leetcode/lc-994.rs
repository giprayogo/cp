use std::collections::VecDeque;

struct Solution;

impl Solution {
    // NOTE: How much is this vs... say get all rotten apples
    // simulate by BFS, continuing if still any...
    // This way it is max depth from each rotten apple, clustered together
    #[allow(clippy::needless_range_loop)]
    pub fn oranges_rotting(mut grid: Vec<Vec<i32>>) -> i32 {
        let ni = grid.len();
        let nj = grid.first().unwrap().len();
        let mut stack = VecDeque::new();
        // need more thinking: is it really max depth?
        // max depth for different cluster
        // min depth for the same cluster!
        // how to mark if this cluster have been visited before?
        // how to mark the clusters shallowest tree?
        // (efficiently that is!)
        // let mut max_depths = HashMap::new();
        // let mut marker = 3;

        // mislead: it is not always the deepest tree from any rotten apple.
        // if they are on the same "cluster", it would be the shallowest tree within
        // that cluster
        /* for i in 0..ni {
            for j in 0..nj {
                if grid[i][j] == 2 {
                    stack.push_back((Some(i), Some(j)));
                    let mut depth = 0;
                    let mut visited = HashSet::new();
                    while !stack.is_empty() {
                        for _ in 0..stack.len() {
                            let (i, j) = stack.pop_front().unwrap();
                            let i = match i {
                                Some(i) => i,
                                None => continue,
                            };
                            let j = match j {
                                Some(j) => j,
                                None => continue,
                            };
                            let coordinate = (i, j);
                            if grid[i][j] == 0 || visited.contains(&coordinate) {
                                continue;
                            }
                            if grid[i][j] != 2 {
                                max_depths
                                    .entry(marker)
                                    .and_modify(|e| *e = depth.max(*e))
                                    .or_insert(depth);
                                grid[i][j] = marker;
                            }
                            visited.insert(coordinate);
                            let _i = i.checked_sub(1);
                            let _j = j.checked_sub(1);
                            let __i = (i + 1 < ni).then_some(i + 1);
                            let __j = (j + 1 < nj).then_some(j + 1);
                            stack.push_back((_i, Some(j)));
                            stack.push_back((__i, Some(j)));
                            stack.push_back((Some(i), _j));
                            stack.push_back((Some(i), __j));
                        }
                        depth += 1;
                    }
                    marker += 1;
                    println!("{grid:?}");
                }
            }
        } */
        // push the bad apples
        for i in 0..ni {
            for j in 0..nj {
                if grid[i][j] == 2 {
                    stack.push_back((Some(i), Some(j)));
                }
            }
        }

        let mut depth = -1;
        let mut _depth = depth;
        while !stack.is_empty() {
            depth += 1;
            for _ in 0..stack.len() {
                let (i, j) = stack.pop_front().unwrap();
                let i = match i {
                    Some(i) => i,
                    None => continue,
                };
                let j = match j {
                    Some(j) => j,
                    None => continue,
                };
                if grid[i][j] == 0 || grid[i][j] == 3 {
                    continue;
                }
                grid[i][j] = 3;
                let _i = i.checked_sub(1);
                let _j = j.checked_sub(1);
                let __i = (i + 1 < ni).then_some(i + 1);
                let __j = (j + 1 < nj).then_some(j + 1);
                stack.push_back((_i, Some(j)));
                stack.push_back((__i, Some(j)));
                stack.push_back((Some(i), _j));
                stack.push_back((Some(i), __j));
            }
        }
        depth -= 1;

        // I need to check if some oranges are never visited
        // note from neetcode: alternatively I can track number of fresh oranges during traversal
        for row in grid {
            for e in row {
                if e == 1 {
                    return -1;
                }
            }
        }
        depth.max(0)
    }
}

fn main() {
    assert_eq!(
        Solution::oranges_rotting(vec![vec![2, 1, 1], vec![1, 1, 0], vec![0, 1, 1]],),
        4
    );
    assert_eq!(
        Solution::oranges_rotting(vec![vec![2, 1, 1], vec![0, 1, 1], vec![1, 0, 1]]),
        -1
    );
    assert_eq!(
        Solution::oranges_rotting(vec![vec![2, 1, 1, 2, 1, 1, 1]]),
        3
    );
    assert_eq!(
        Solution::oranges_rotting(vec![vec![2, 1, 1], vec![0, 1, 0]]),
        2
    );
    assert_eq!(Solution::oranges_rotting(vec![vec![0, 2]]), 0);
    assert_eq!(Solution::oranges_rotting(vec![vec![0]]), 0);
    assert_eq!(Solution::oranges_rotting(vec![vec![0, 2, 2]]), 0);
}
