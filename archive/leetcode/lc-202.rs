use std::collections::HashSet;

impl Solution {
    pub fn is_happy(mut n: i32) -> bool {
        let mut visited = HashSet::new();
        loop {
            let mut _n = 0;
            while n != 0 {
                _n += (n % 10).pow(2);
                n /= 10;
            }
            if _n == 1 {
                return true;
            } else if visited.contains(&_n) {
                return false;
            } else {
                visited.insert(_n);
                n = _n;
            }
        }
    }

    // neetcode: slow-fast pointer can be used here! remember!
    pub fn is_happy_2(n: i32) -> bool {
        fn next(n: i32) -> i32 {
            let mut n = n;
            let mut _n = 0;
            while n != 0 {
                _n += (n % 10).pow(2);
                n /= 10;
            }
            _n
        }

        let mut slow = n;
        let mut fast = next(n);
        loop {
            slow = next(slow);
            fast = next(next(fast));
            if fast == 1 {
                return true;
            } else if slow == fast {
                return false;
            }
        }
    }
}

// NOTE: check bounds with largest digits: does it increases forever?
// if not then we can safely assume that it will loop at some point!
// no magic number theory thing blah2

struct Solution;

fn main() {
    for f in [Solution::is_happy, Solution::is_happy_2] {
        assert!(f(19));
        assert!(!f(2));
    }
}
