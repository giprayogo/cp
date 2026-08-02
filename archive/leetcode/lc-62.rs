struct Solution;

impl Solution {
    pub fn unique_paths(m: i32, n: i32) -> i32 {
        let m = m as usize;
        let n = n as usize;
        let mut paths = vec![0; m * n];
        // NOTE: neetcode: I only need to keep the current and previous row, really
        // so 2*n instead of m*n should be ok
        paths[0] = 1;
        for x in 0..m {
            for y in 0..n {
                let up = if x != 0 { paths[(x - 1) * n + y] } else { 0 };
                let left = if y != 0 { paths[x * n + (y - 1)] } else { 0 };
                paths[x * n + y] += up + left;
            }
        }
        *paths.last().unwrap()
    }
}

fn main() {
    assert_eq!(Solution::unique_paths(3, 7), 28);
    assert_eq!(Solution::unique_paths(3, 2), 3);
}
