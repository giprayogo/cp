impl Solution {
    #[allow(clippy::needless_range_loop)]
    pub fn set_zeroes(matrix: &mut Vec<Vec<i32>>) {
        // mark
        let ni = matrix.len();
        let nj = matrix.first().unwrap().len();
        let mut row_0 = false;
        let mut col_0 = false;
        for i in 0..ni {
            if matrix[i][0] == 0 {
                col_0 = true;
            };
        }
        for j in 0..nj {
            if matrix[0][j] == 0 {
                row_0 = true;
            }
        }

        for i in 1..ni {
            for j in 1..nj {
                if matrix[i][j] == 0 {
                    matrix[0][j] = 0;
                    matrix[i][0] = 0;
                }
            }
        }
        for j in 1..nj {
            if matrix[0][j] == 0 {
                for i in 0..ni {
                    matrix[i][j] = 0;
                }
            }
        }
        for i in 1..ni {
            if matrix[i][0] == 0 {
                for j in 0..nj {
                    matrix[i][j] = 0;
                }
            }
        }
        if row_0 {
            for j in 0..nj {
                matrix[0][j] = 0;
            }
        }
        if col_0 {
            for i in 0..ni {
                matrix[i][0] = 0;
            }
        }
    }
    // neetcode: I really only need to check for the row but.. I guess this way is more symmetric?
}

struct Solution;

fn main() {
    let mut a = vec![vec![1, 1, 1], vec![1, 0, 1], vec![1, 1, 1]];
    Solution::set_zeroes(&mut a);
    assert_eq!(a, [[1, 0, 1], [0, 0, 0], [1, 0, 1]]);
    a = vec![vec![0, 1, 2, 0], vec![3, 4, 5, 2], vec![1, 3, 1, 5]];
    Solution::set_zeroes(&mut a);
    assert_eq!(a, [[0, 0, 0, 0], [0, 4, 5, 0], [0, 3, 1, 0]]);
    a = vec![
        vec![1, 2, 3, 4],
        vec![5, 0, 7, 8],
        vec![0, 10, 11, 12],
        vec![13, 14, 15, 0],
    ];
    Solution::set_zeroes(&mut a);
    assert_eq!(a, [[0, 0, 3, 0], [0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0]])
}
