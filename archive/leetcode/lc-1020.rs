impl Solution {
    fn num_enclaves(mut grid: Vec<Vec<i32>>) -> i32 {
        let ni = grid.len();
        let nj = grid.first().unwrap().len();
        let mut count = 0;

        fn dfs(
            grid: &mut Vec<Vec<i32>>,
            count: &mut i32,
            i: usize,
            j: usize,
            ni: usize,
            nj: usize,
        ) {
            if grid[i][j] != 1 {
                return;
            }

            *count += 1;
            grid[i][j] = 2;
            if i > 0 {
                dfs(grid, count, i - 1, j, ni, nj);
            }
            if i < ni - 1 {
                dfs(grid, count, i + 1, j, ni, nj);
            }
            if j > 0 {
                dfs(grid, count, i, j - 1, ni, nj);
            }
            if j < nj - 1 {
                dfs(grid, count, i, j + 1, ni, nj);
            }
        }
        for i in 0..ni {
            if grid[i][0] == 1 {
                dfs(&mut grid, &mut 0, i, 0, ni, nj);
            }
            if grid[i][nj - 1] == 1 {
                dfs(&mut grid, &mut 0, i, nj - 1, ni, nj);
            }
        }
        for j in 0..nj {
            if grid[0][j] == 1 {
                dfs(&mut grid, &mut 0, 0, j, ni, nj);
            }
            if grid[ni - 1][j] == 1 {
                dfs(&mut grid, &mut 0, ni - 1, j, ni, nj);
            }
        }
        for i in 0..ni {
            for j in 0..nj {
                if grid[i][j] == 1 {
                    let mut _count = 0;
                    dfs(&mut grid, &mut _count, i, j, ni, nj);
                    count += _count;
                }
            }
        }
        count
    }
}

struct Solution;

fn main() {
    assert_eq!(
        Solution::num_enclaves(vec![
            vec![0, 0, 0, 0],
            vec![1, 0, 1, 0],
            vec![0, 1, 1, 0],
            vec![0, 0, 0, 0]
        ]),
        3
    );
    assert_eq!(
        Solution::num_enclaves(vec![
            vec![0, 1, 1, 0],
            vec![0, 0, 1, 0],
            vec![0, 0, 1, 0],
            vec![0, 0, 0, 0]
        ]),
        0
    )
}
