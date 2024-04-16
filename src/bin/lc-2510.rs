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

    // More clever math solution (thanks nskybytskyi!): if there's path to (i,j)
    // of length A and B, A < B, then there's path of length C for every A < C < B
    // simple image
    // 1 0
    // 1 1
    // going right first or down first, at most differ by 1
    #[allow(clippy::needless_range_loop)]
    fn is_there_a_path_2(grid: Vec<Vec<i32>>) -> bool {
        let ni = grid.len();
        let nj = grid.first().unwrap().len();
        let distance = ni + nj - 1;
        if distance % 2 == 1 {
            return false;
        }
        let target = distance / 2;
        let mut row = vec![(0, 0); nj];
        for i in 0..ni {
            let mut _row = vec![(0, 0); nj];
            for j in 0..nj {
                let left = match j.checked_sub(1) {
                    Some(_j) => _row[_j],
                    None => (0, 0),
                };
                let up = row[j];
                _row[j] = (
                    left.0.min(up.0) + grid[i][j] as usize,
                    left.1.max(up.1) + grid[i][j] as usize,
                );
            }
            row = _row;
        }
        let ans = row.last().unwrap();
        ans.0 <= target && target <= ans.1
    }
}

struct Solution;

// Lesson: overthinking. Indeed it was DP 3-D (or 2-D with some math!)

fn main() {
    for f in [Solution::is_there_a_path, Solution::is_there_a_path_2] {
        assert!(!f(vec![vec![1, 0], vec![0, 1],]));
        assert!(f(vec![vec![1, 0, 0], vec![0, 1, 0]]));
        assert!(f(vec![
            vec![0, 1, 0, 0],
            vec![0, 1, 0, 0],
            vec![1, 0, 1, 0]
        ]));
        assert!(!f(vec![vec![1, 1, 0], vec![0, 0, 1], vec![1, 0, 0]]))
    }
}
