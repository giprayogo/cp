impl Solution {
    pub fn get_sum(mut a: i32, mut b: i32) -> i32 {
        let mut c = 0;
        let mut s = 0;
        for i in 0..32 {
            let __a = 1 & a;
            let __b = 1 & b;
            s += (__a ^ __b ^ c) << i;
            c = (__a & __b) | (__a & c) | (__b & c);
            a >>= 1;
            b >>= 1;
        }
        s
    }
    // actually my thought of "repeated curry" works; just that I killed the process early (and wrongly sum a and b)
    pub fn get_sum_2(mut a: i32, mut b: i32) -> i32 {
        while b != 0 {
            let c = (a & b) << 1;
            a ^= b;
            b = c;
        }
        a
    }
}

struct Solution;

fn main() {
    for f in [Solution::get_sum, Solution::get_sum_2] {
        assert_eq!(f(1, 2), 3);
        assert_eq!(f(2, 3), 5);
        assert_eq!(f(0, 3), 3);
    }
}
