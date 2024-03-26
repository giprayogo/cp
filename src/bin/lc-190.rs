impl Solution {
    pub fn reverse_bits(mut x: u32) -> u32 {
        let mut _x = 0;
        _x |= x & 1;
        for _ in 0..31 {
            _x <<= 1;
            x >>= 1;
            _x |= x & 1;
        }
        _x
    }
    // neetcode: more compact!, kind of...
    pub fn reverse_bits_2(x: u32) -> u32 {
        let mut _x = 0;
        for i in 0..32 {
            let b = (x >> i) & 1;
            _x += b << (31 - i);
        }
        _x
    }
    // or builtin solution
    pub fn reverse_bits_3(x: u32) -> u32 {
        x.reverse_bits()
    }
}

struct Solution;

fn main() {
    for f in [
        Solution::reverse_bits,
        Solution::reverse_bits_2,
        Solution::reverse_bits_3,
    ] {
        assert_eq!(f(0b00000010100101000001111010011100), 964176192);
        assert_eq!(f(0b11111111111111111111111111111101), 3221225471);
    }
}
