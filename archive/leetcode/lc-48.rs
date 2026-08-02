impl Solution {
    pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
        let n = matrix.len();
        for i in 0..(n / 2) {
            for j in i..(n - i - 1) {
                let mut _i = i;
                let mut _j = j;
                let mut b = matrix[_i][_j];
                for _ in 0..4 {
                    std::mem::swap(&mut _i, &mut _j);
                    _j = n - 1 - _j;
                    std::mem::swap(&mut matrix[_i][_j], &mut b);
                }
            }
        }
    }
    // neetcode: can be more simply as reverse one axis + transpose!
}

struct Solution;

fn main() {
    let mut a = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
    Solution::rotate(&mut a);
    assert_eq!(a, [[7, 4, 1], [8, 5, 2], [9, 6, 3]]);
    let mut a = vec![
        vec![5, 1, 9, 11],
        vec![2, 4, 8, 10],
        vec![13, 3, 6, 7],
        vec![15, 14, 12, 16],
    ];
    Solution::rotate(&mut a);
    assert_eq!(
        a,
        [
            [15, 13, 2, 5],
            [14, 3, 4, 1],
            [12, 6, 8, 9],
            [16, 7, 10, 11]
        ]
    );
}
