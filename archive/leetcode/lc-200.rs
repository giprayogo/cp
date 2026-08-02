struct Solution;

impl Solution {
    pub fn num_islands(grid: Vec<Vec<char>>) -> i32 {
        let ni = grid.len();
        let nj = grid.first().unwrap().len();
        // NOTE: neetcode I don't need this, just replace visited place with '0'!
        let mut visited = vec![vec![false; nj]; ni];

        // NOTE: I don't technically need to insist i, j to be usize, but this feels more 'correct'
        fn dfs(visited: &mut [Vec<bool>], grid: &[Vec<char>], i: usize, j: usize) {
            // TODO: I don't like double checking!!!!!
            // NOTE: Traditionally everything is checked here instead of in multiple places like before
            if visited[i][j] || grid[i][j] == '0' {
                return;
            }
            visited[i][j] = true;

            // bound checks
            let _i = i.checked_sub(1);
            let _j = j.checked_sub(1);
            let __i = (i + 1 < grid.len()).then_some(i + 1);
            let __j = (j + 1 < grid.first().unwrap().len()).then_some(j + 1);
            if let Some(i) = _i {
                dfs(visited, grid, i, j)
            };
            if let Some(i) = __i {
                dfs(visited, grid, i, j)
            };
            if let Some(j) = _j {
                dfs(visited, grid, i, j)
            };
            if let Some(j) = __j {
                dfs(visited, grid, i, j)
            };
        }

        let mut count = 0;
        for i in 0..ni {
            for j in 0..nj {
                if !visited[i][j] && grid[i][j] == '1' {
                    dfs(&mut visited, &grid, i, j);
                    count += 1;
                }
            }
        }
        count
    }
}

fn main() {
    assert_eq!(
        Solution::num_islands(vec![
            vec!['1', '1', '1', '1', '0'],
            vec!['1', '1', '0', '1', '0'],
            vec!['1', '1', '0', '0', '0'],
            vec!['0', '0', '0', '0', '0']
        ]),
        1
    );
    assert_eq!(
        Solution::num_islands(vec![
            vec!['1', '1', '0', '0', '0'],
            vec!['1', '1', '0', '0', '0'],
            vec!['0', '0', '1', '0', '0'],
            vec!['0', '0', '0', '1', '1']
        ]),
        3
    );
}
