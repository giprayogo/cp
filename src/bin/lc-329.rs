struct Solution;

impl Solution {
    // NOT TLE but not the best
    pub fn longest_increasing_path(matrix: Vec<Vec<i32>>) -> i32 {
        let m = matrix.len();
        let n = matrix.first().unwrap().len();
        let mut pathlen = vec![vec![-1; n]; m];

        fn dfs(
            matrix: &Vec<Vec<i32>>,
            pathlen: &mut Vec<Vec<i32>>,
            visited: &mut Vec<Vec<bool>>,
            i: Option<usize>,
            j: Option<usize>,
            prev_depth: i32,
        ) -> i32 {
            let i = match i {
                Some(i) => i,
                None => return 0,
            };
            let j = match j {
                Some(j) => j,
                None => return 0,
            };

            if visited[i][j] {
                return 0;
            }
            if prev_depth >= matrix[i][j] {
                return 0;
            }
            if pathlen[i][j] != -1 {
                return pathlen[i][j];
            }

            let _i = i.checked_sub(1);
            let _j = j.checked_sub(1);
            let __i = (i + 1 < matrix.len()).then_some(i + 1);
            let __j = (j + 1 < matrix.first().unwrap().len()).then_some(j + 1);
            let cur_depth = matrix[i][j];
            let cur_pathlen = dfs(matrix, pathlen, visited, _i, Some(j), cur_depth)
                .max(dfs(matrix, pathlen, visited, __i, Some(j), cur_depth))
                .max(dfs(matrix, pathlen, visited, Some(i), _j, cur_depth))
                .max(dfs(matrix, pathlen, visited, Some(i), __j, cur_depth))
                + 1;
            pathlen[i][j] = cur_pathlen;
            cur_pathlen
        }

        let mut max = 1;
        for i in 0..m {
            for j in 0..n {
                max = max.max(dfs(
                    &matrix,
                    &mut pathlen,
                    &mut vec![vec![false; n]; m],
                    Some(i),
                    Some(j),
                    -1,
                ));
            }
        }
        max
    }
}

fn main() {
    assert_eq!(
        Solution::longest_increasing_path(vec![vec![9, 9, 4], vec![6, 6, 8], vec![2, 1, 1]]),
        4
    );
    assert_eq!(
        Solution::longest_increasing_path(vec![vec![3, 4, 5], vec![3, 2, 6], vec![2, 2, 1]],),
        4
    );
    assert_eq!(Solution::longest_increasing_path(vec![vec![1]]), 1);
}
