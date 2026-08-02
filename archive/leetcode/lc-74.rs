use std::cmp::Ordering::{Equal, Greater, Less};

fn main() {}

#[allow(unused)]
struct Solution;

#[allow(unused)]
impl Solution {
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        let nrow = matrix.len();
        let ncol = matrix[0].len();
        let len = nrow * ncol;

        let mut l = 0;
        let mut r = len;
        while l < r {
            let m = (l + r) / 2;
            let i = m / ncol;
            let j = m % ncol;
            match target.cmp(&matrix[i][j]) {
                Equal => {
                    return true;
                }
                Greater => {
                    l = m + 1;
                }
                Less => {
                    r = m;
                }
            }
        }
        false
    }
}

#[test]
fn test_solution() {
    assert!(Solution::search_matrix(
        vec![vec![1, 3, 5, 7], vec![10, 11, 16, 20], vec![23, 30, 34, 60]],
        3
    ));
    assert!(!Solution::search_matrix(
        vec![vec![1, 3, 5, 7], vec![10, 11, 16, 20], vec![23, 30, 34, 60]],
        13
    ));
}
