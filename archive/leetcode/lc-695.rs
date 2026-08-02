struct Solution;

impl Solution {
    pub fn max_area_of_island(mut grid: Vec<Vec<i32>>) -> i32 {
        // easy simple DFS
        let mut max_area = 0;
        let ni = grid.len();
        let nj = grid.first().unwrap().len();

        fn dfs(grid: &mut [Vec<i32>], i: usize, j: usize) -> i32 {
            if grid[i][j] == 0 {
                return 0;
            }
            // increase area by 1?
            grid[i][j] = 0;
            let mut area = 1;
            let _i = i.checked_sub(1);
            let _j = j.checked_sub(1);
            let __i = (i + 1 < grid.len()).then_some(i + 1);
            let __j = (j + 1 < grid.first().unwrap().len()).then_some(j + 1);
            if let Some(i) = _i {
                area += dfs(grid, i, j);
            }
            if let Some(i) = __i {
                area += dfs(grid, i, j);
            }
            if let Some(j) = _j {
                area += dfs(grid, i, j);
            }
            if let Some(j) = __j {
                area += dfs(grid, i, j);
            }
            area
        }

        for i in 0..ni {
            for j in 0..nj {
                // note: Can optimize unnecessary function calls here
                let area = dfs(&mut grid, i, j);
                max_area = max_area.max(area);
            }
        }

        max_area
    }
}

fn main() {
    assert_eq!(
        Solution::max_area_of_island(vec![
            vec![0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0],
            vec![0, 1, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0],
            vec![0, 1, 0, 0, 1, 1, 0, 0, 1, 0, 1, 0, 0],
            vec![0, 1, 0, 0, 1, 1, 0, 0, 1, 1, 1, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0]
        ]),
        6
    );
    assert_eq!(
        Solution::max_area_of_island(vec![vec![0, 0, 0, 0, 0, 0, 0, 0]]),
        0
    )
}
