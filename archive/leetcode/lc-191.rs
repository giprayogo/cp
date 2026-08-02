impl Solution {
    pub fn hamming_weight(n: i32) -> i32 {
        n.count_ones() as i32
    }
    pub fn hamming_weight_2(mut n: i32) -> i32 {
        let mut count = 0;
        while n != 0 {
            n = (n - 1) & n;
            count += 1;
        }
        count
    }
}

struct Solution;

fn main() {
    for f in [Solution::hamming_weight, Solution::hamming_weight_2] {
        assert_eq!(f(11), 3);
        assert_eq!(f(128), 1);
        // assert_eq!(f(2147483645), 3);
    }
}
