impl Solution {
    pub fn reverse(mut x: i32) -> i32 {
        let mut m = 1;
        let mut set = false;
        while x / m != 0 {
            m = match m.checked_mul(10) {
                Some(v) => v,
                None => {
                    set = true;
                    break;
                }
            }
        }
        if !set {
            m /= 10;
        }
        let mut _m = 1;
        let mut r: i32 = 0;
        while m != 0 {
            let d = x / m;
            let s = match d.checked_mul(_m) {
                Some(v) => v,
                None => return 0,
            };
            r = match r.checked_add(s) {
                Some(v) => v,
                None => return 0,
            };
            x -= d * m;
            m /= 10;
            _m = match _m.checked_mul(10) {
                Some(v) => v,
                None => return r,
            };
        }
        r
    }

    // neetcode: things can be simpler if I do things in one loop you know
    // also there is -remainder- operator
    pub fn reverse_2(mut x: i32) -> i32 {
        let mut res: i32 = 0;
        while x != 0 {
            res = match res.checked_mul(10) {
                Some(v) => v,
                None => return 0,
            };
            let d = x % 10;
            x /= 10;
            res = match res.checked_add(d) {
                Some(v) => v,
                None => return 0,
            };
        }
        res
    }

    // also some people use string: clever!
}

struct Solution;

fn main() {
    for f in [Solution::reverse, Solution::reverse_2] {
        assert_eq!(f(123), 321);
        assert_eq!(f(-123), -321);
        assert_eq!(f(120), 21);
        assert_eq!(f(1534236469), 0);
        assert_eq!(f(-2147483412), -2143847412);
    }
}
