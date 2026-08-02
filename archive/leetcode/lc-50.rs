use std::collections::{BTreeSet, HashMap};

impl Solution {
    // funny question? am I supposed to implement on my own?
    // normal multiplication right, not one of those sum series...
    // pub fn my_pow(x: f64, n: i32) -> f64 {
    //     x.powf(n as f64)
    // }
    // Note: this naive thing can TLE on large n!
    // pub fn my_pow(x: f64, n: i32) -> f64 {
    //     match n.cmp(&0) {
    //         std::cmp::Ordering::Greater => {
    //             let mut _x = x;
    //             for _ in 1..n {
    //                 _x *= x;
    //             }
    //             _x
    //         }
    //         std::cmp::Ordering::Less => {
    //             let mut _x = 1.0;
    //             for _ in 0..-n {
    //                 _x /= x;
    //             }
    //             _x
    //         }
    //         std::cmp::Ordering::Equal => 1.0,
    //     }
    // }
    pub fn my_pow(x: f64, n: i32) -> f64 {
        let mut cache: HashMap<i32, f64> = HashMap::new();
        cache.insert(0, 1.0);
        cache.insert(1, x);
        let mut chain = BTreeSet::new();
        let mut overflow = false;
        let mut _n = if n < 1 {
            match n.checked_neg() {
                Some(v) => v,
                None => {
                    overflow = true;
                    -(n + 1)
                }
            }
        } else {
            n
        };

        while _n > 1 {
            chain.insert(_n);
            chain.insert(_n % 2);
            _n /= 2;
        }
        for c in chain {
            let half = cache[&(c / 2)];
            let rem = cache[&(c % 2)];
            cache.insert(c, half * half * rem);
        }
        if n < 0 {
            if overflow {
                1.0 / cache[&-(n + 1)] / x
            } else {
                1.0 / cache[&-n]
            }
        } else {
            cache[&n]
        }
    }
    // neetcode: more elegant formulation! remember residual of /2 is either 0 or 1...
    pub fn my_pow_2(x: f64, n: i32) -> f64 {
        fn rec(x: f64, n: i32) -> f64 {
            if n == 0 {
                return 1.0;
            } else if x == 0.0 {
                return 0.0;
            }
            let _x = rec(x * x, n / 2);
            if n % 2 == 0 {
                _x
            } else {
                x * _x
            }
        }
        if n < 0 {
            match n.checked_neg() {
                Some(_n) => 1.0 / rec(x, _n),
                None => 1.0 / rec(x, -(n + 1)) / x,
            }
        } else {
            rec(x, n)
        }
    }
}

struct Solution;

fn main() {
    for f in [Solution::my_pow, Solution::my_pow_2] {
        assert_eq!(f(2.0, 10), 1024.0);
        assert!((f(2.1, 3) - 9.261).abs() < 10e-10);
        assert_eq!(f(2.0, -2), 0.25);
        assert_eq!(f(1.0, -2147483648), 1.0);
    }
}
