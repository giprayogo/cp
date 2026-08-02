impl Solution {
    // Ugly but works -> is there a simpler solution?
    pub fn spiral_order(mut matrix: Vec<Vec<i32>>) -> Vec<i32> {
        let ni = matrix.len() as isize;
        let nj = matrix[0].len() as isize;
        let size = ni * nj;
        let mut res = Vec::with_capacity(size as usize);
        let d = vec![(0, 1), (1, 0), (0, -1), (-1, 0)];
        let mut d_i = 0;
        let mut i = 0;
        let mut j = 0;
        while res.len() < (size - 1) as usize {
            let (di, dj) = d[d_i];
            if i + di >= ni || j + dj >= nj || i + di < 0 || j + dj < 0 {
                d_i = (d_i + 1) % 4;
                continue;
            }
            if matrix[(i + di) as usize][(j + dj) as usize] == i32::MIN {
                d_i = (d_i + 1) % 4;
                continue;
            }
            res.push(matrix[i as usize][j as usize]);
            matrix[i as usize][j as usize] = i32::MIN;
            i += di;
            j += dj;
        }
        res.push(matrix[i as usize][j as usize]);
        res
    }
    // simpler code by neetcode (neat!)
    #[allow(clippy::needless_range_loop)]
    pub fn spiral_order_2(matrix: Vec<Vec<i32>>) -> Vec<i32> {
        let ni = matrix.len();
        let nj = matrix[0].len();
        let mut res = Vec::with_capacity(ni * nj);
        let mut left = 0;
        let mut right = nj;
        let mut top = 0;
        let mut bottom = ni;
        while left < right && top < bottom {
            for j in left..right {
                res.push(matrix[top][j]);
            }
            top += 1;
            for i in top..bottom {
                res.push(matrix[i][right - 1]);
            }
            right -= 1;
            if !(left < right && top < bottom) {
                break;
            }
            for j in (left..right).rev() {
                res.push(matrix[bottom - 1][j]);
            }
            bottom -= 1;
            for i in (top..bottom).rev() {
                res.push(matrix[i][left]);
            }
            left += 1;
        }
        res
    }
}

struct Solution;

fn main() {
    for f in [Solution::spiral_order, Solution::spiral_order_2] {
        assert_eq!(
            f(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]]),
            [1, 2, 3, 6, 9, 8, 7, 4, 5]
        );
        assert_eq!(
            f(vec![
                vec![1, 2, 3, 4],
                vec![5, 6, 7, 8],
                vec![9, 10, 11, 12]
            ]),
            [1, 2, 3, 4, 8, 12, 11, 10, 9, 5, 6, 7]
        );
        assert_eq!(f(vec![vec![1], vec![2]]), [1, 2]);
    }
}
