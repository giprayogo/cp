use std::collections::HashSet;

struct Solution;

impl Solution {
    // Let's make something similar but climbing so I don't have to iterate from every grid
    // Also realization from neetcode: I don't need to construct matrix, that the water
    // can flow there is -enough-; make a set and join the results!
    // of course probably can be even more optimized here and there but... even this one is much better already!
    pub fn pacific_atlantic(heights: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let ni = heights.len();
        let nj = heights.first().unwrap().len();
        let mut pacific = HashSet::new();
        let mut atlantic = HashSet::new();

        fn dfs(
            heights: &Vec<Vec<i32>>,
            visited: &mut HashSet<Vec<usize>>,
            i: Option<usize>,
            j: Option<usize>,
            c: i32,
        ) {
            let i = match i {
                Some(v) => v,
                None => return,
            };
            let j = match j {
                Some(v) => v,
                None => return,
            };
            if heights[i][j] < c || visited.contains(&vec![i, j]) {
                return;
            }

            visited.insert(vec![i, j]);

            let c = heights[i][j];
            let _i = i.checked_sub(1);
            let _j = j.checked_sub(1);
            let __i = (i + 1 < heights.len()).then_some(i + 1);
            let __j = (j + 1 < heights.first().unwrap().len()).then_some(j + 1);
            dfs(heights, visited, _i, Some(j), c);
            dfs(heights, visited, Some(i), _j, c);
            dfs(heights, visited, __i, Some(j), c);
            dfs(heights, visited, Some(i), __j, c);
        }
        for j in 0..nj {
            dfs(&heights, &mut pacific, Some(0), Some(j), heights[0][j]);
        }
        for i in 0..ni {
            dfs(&heights, &mut pacific, Some(i), Some(0), heights[i][0]);
        }
        for j in 0..nj {
            dfs(
                &heights,
                &mut atlantic,
                Some(ni - 1),
                Some(j),
                heights[ni - 1][j],
            );
        }
        for i in 0..ni {
            dfs(
                &heights,
                &mut atlantic,
                Some(i),
                Some(nj - 1),
                heights[i][nj - 1],
            );
        }

        pacific
            .intersection(&atlantic)
            .map(|x| x.iter().map(|x| *x as i32).collect())
            .collect()
    }

    // Correct but slow (not TLE though);
    /*     #[allow(clippy::needless_range_loop)]
    pub fn pacific_atlantic(heights: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let ni = heights.len();
        let nj = heights.first().unwrap().len();
        let mut pacific = vec![vec![false; nj]; ni];
        let mut atlantic = vec![vec![false; nj]; ni];
        for j in 0..nj {
            pacific[0][j] = true;
        }
        for i in 0..ni {
            pacific[i][0] = true;
        }
        for j in 0..nj {
            atlantic[ni - 1][j] = true;
        }
        for i in 0..ni {
            atlantic[i][nj - 1] = true;
        }

        fn dfs(
            heights: &Vec<Vec<i32>>,
            can: &mut Vec<Vec<bool>>,
            visited: &mut Vec<Vec<bool>>,
            i: Option<usize>,
            j: Option<usize>,
            c: i32,
        ) -> bool {
            let i = match i {
                Some(v) => v,
                None => return false,
            };
            let j = match j {
                Some(v) => v,
                None => return false,
            };

            if heights[i][j] > c {
                return false;
            }
            if visited[i][j] {
                return can[i][j];
            }
            visited[i][j] = true;
            if can[i][j] {
                return true;
            }

            let _i = i.checked_sub(1);
            let _j = j.checked_sub(1);
            let __i = (i + 1 < heights.len()).then_some(i + 1);
            let __j = (j + 1 < heights.first().unwrap().len()).then_some(j + 1);
            let a = dfs(heights, can, visited, _i, Some(j), heights[i][j]);
            let b = dfs(heights, can, visited, Some(i), _j, heights[i][j]);
            let c = dfs(heights, can, visited, __i, Some(j), heights[i][j]);
            let d = dfs(heights, can, visited, Some(i), __j, heights[i][j]);
            can[i][j] = can[i][j] || a || b || c || d;
            can[i][j]
        }

        for i in 0..ni {
            for j in 0..nj {
                dfs(
                    &heights,
                    &mut pacific,
                    &mut vec![vec![false; nj]; ni],
                    Some(i),
                    Some(j),
                    heights[i][j],
                );
                dfs(
                    &heights,
                    &mut atlantic,
                    &mut vec![vec![false; nj]; ni],
                    Some(i),
                    Some(j),
                    heights[i][j],
                );
            }
        }

        let mut result = Vec::new();
        for i in 0..ni {
            for j in 0..nj {
                if pacific[i][j] && atlantic[i][j] {
                    result.push(vec![i as i32, j as i32]);
                }
            }
        }
        result
    } */
}

fn main() {
    assert_eq!(
        Solution::pacific_atlantic(vec![
            vec![1, 2, 2, 3, 5],
            vec![3, 2, 3, 4, 4],
            vec![2, 4, 5, 3, 1],
            vec![6, 7, 1, 4, 5],
            vec![5, 1, 1, 2, 4]
        ])
        .iter()
        .collect::<HashSet<_>>(),
        vec![
            vec![0, 4],
            vec![1, 3],
            vec![1, 4],
            vec![2, 2],
            vec![3, 0],
            vec![3, 1],
            vec![4, 0]
        ]
        .iter()
        .collect::<HashSet<_>>()
    );
    assert_eq!(
        Solution::pacific_atlantic(vec![vec![1]])
            .iter()
            .collect::<HashSet<_>>(),
        vec![vec![0, 0]].iter().collect::<HashSet<_>>()
    );
    Solution::pacific_atlantic(vec![vec![1, 1], vec![1, 1], vec![1, 1]]);
    Solution::pacific_atlantic(vec![
        vec![3, 3, 3, 3, 3, 3],
        vec![3, 0, 3, 3, 0, 3],
        vec![3, 3, 3, 3, 3, 3],
    ]);
}
