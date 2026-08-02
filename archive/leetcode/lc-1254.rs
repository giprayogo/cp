impl Solution {
    fn closed_island(mut grid: Vec<Vec<i32>>) -> i32 {
        fn dfs(grid: &mut Vec<Vec<i32>>, i: usize, j: usize, ni: usize, nj: usize) {
            if grid[i][j] != 0 {
                return;
            }
            grid[i][j] = 2;
            if i < ni - 1 {
                dfs(grid, i + 1, j, ni, nj);
            }
            if i > 0 {
                dfs(grid, i - 1, j, ni, nj);
            }
            if j < nj - 1 {
                dfs(grid, i, j + 1, ni, nj);
            }
            if j > 0 {
                dfs(grid, i, j - 1, ni, nj);
            }
        }
        let ni = grid.len();
        let nj = grid.first().unwrap().len();
        for i in 0..ni {
            dfs(&mut grid, i, 0, ni, nj);
            dfs(&mut grid, i, nj - 1, ni, nj);
        }
        for j in 1..(nj - 1) {
            dfs(&mut grid, 0, j, ni, nj);
            dfs(&mut grid, ni - 1, j, ni, nj);
        }

        let mut count = 0;
        for i in 1..(ni - 1) {
            for j in 1..(nj - 1) {
                if grid[i][j] == 0 {
                    count += 1;
                    dfs(&mut grid, i, j, ni, nj);
                }
            }
        }
        count
    }
}

struct Solution;

fn main() {
    assert_eq!(Solution::closed_island(vec![vec![0]]), 0);
    assert_eq!(Solution::closed_island(vec![vec![1]]), 0);
    assert_eq!(
        Solution::closed_island(vec![
            vec![1, 1, 1, 1, 1, 1, 1, 0],
            vec![1, 0, 0, 0, 0, 1, 1, 0],
            vec![1, 0, 1, 0, 1, 1, 1, 0],
            vec![1, 0, 0, 0, 0, 1, 0, 1],
            vec![1, 1, 1, 1, 1, 1, 1, 0]
        ]),
        2
    );
    assert_eq!(
        Solution::closed_island(vec![
            vec![0, 0, 1, 0, 0],
            vec![0, 1, 0, 1, 0],
            vec![0, 1, 1, 1, 0]
        ]),
        1
    );
}
