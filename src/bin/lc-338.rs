impl Solution {
    pub fn count_bits(n: i32) -> Vec<i32> {
        let mut v = Vec::with_capacity(n as usize + 1);
        for i in 0..=n {
            v.push(i.count_ones() as i32);
        }
        v
    }
    // neetcode: basically notice the pattern on the bits
    // 2*n + 2*n-1 + ...
    pub fn count_bits_2(n: i32) -> Vec<i32> {
        let n = n as usize;
        let mut v = vec![0; n + 1];
        let mut offset: usize = 1;
        for i in 1..=n {
            if i == offset * 2 {
                offset = i;
            }
            v[i] = 1 + v[i - offset];
        }
        v
    }
}

struct Solution;

fn main() {
    for f in [Solution::count_bits, Solution::count_bits_2] {
        assert_eq!(f(2), [0, 1, 1]);
        assert_eq!(f(5), [0, 1, 1, 2, 1, 2]);
    }
}
