impl Solution {
    #[allow(clippy::needless_range_loop)]
    fn is_there_a_path(grid: Vec<Vec<i32>>) -> bool {
        let ni = grid.len();
        let nj = grid.first().unwrap().len();
        let distance = ni + nj - 1;
        if distance % 2 == 1 {
            return false;
        }
        let nk = distance + 1;
        let target = distance / 2;

        let mut row = vec![vec![false; nk]; nj];
        row[0][0] = true;
        for i in 0..ni {
            let mut _row = vec![vec![false; nk]; nj];
            for j in 0..nj {
                for k in 0..=(i + j + 1) {
                    let _k = if grid[i][j] == 0 {
                        k
                    } else if let Some(_k) = k.checked_sub(1) {
                        _k
                    } else {
                        continue;
                    };
                    let left = match j.checked_sub(1) {
                        Some(_j) => _row[_j][_k],
                        None => false,
                    };
                    let up = row[j][_k];
                    _row[j][k] = left || up;
                }
            }
            row = _row;
        }
        row.last().unwrap()[target]
    }
}

struct Solution;

// Lesson: overthinking. Indeed it was DP 3-D

fn main() {
    assert!(!Solution::is_there_a_path(vec![vec![1, 0], vec![0, 1],]));
    assert!(Solution::is_there_a_path(vec![
        vec![1, 0, 0],
        vec![0, 1, 0]
    ]));
    assert!(Solution::is_there_a_path(vec![
        vec![0, 1, 0, 0],
        vec![0, 1, 0, 0],
        vec![1, 0, 1, 0]
    ]));
    assert!(!Solution::is_there_a_path(vec![
        vec![1, 1, 0],
        vec![0, 0, 1],
        vec![1, 0, 0]
    ]))
}
