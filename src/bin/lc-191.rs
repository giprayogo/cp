impl Solution {
    pub fn hamming_weight(n: i32) -> i32 {
        n.count_ones() as i32
    }
}

struct Solution;

fn main() {
    assert_eq!(Solution::hamming_weight(11), 3);
    assert_eq!(Solution::hamming_weight(128), 1);
    assert_eq!(Solution::hamming_weight(2147483645), 3);
}
