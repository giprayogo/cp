struct Solution;

impl Solution {
    // Accepted, OK, but not the best
    pub fn jump_1(nums: Vec<i32>) -> i32 {
        let mut njump = vec![i32::MAX; nums.len() - 1];
        match njump.first_mut() {
            Some(v) => *v = 0,
            None => return 0,
        }
        let mut max_i = 0;
        for i in 0..(nums.len() - 1) {
            let n = nums[i];
            let nj = njump[i] + 1;
            for j in (max_i..=(i + n as usize)).rev() {
                match njump.get_mut(j) {
                    Some(v) => *v = (*v).min(nj),
                    None => return nj,
                };
            }
            max_i = max_i.max(i + n as usize);
        }
        unreachable!()
    }

    // neetcode: I really don't need the njump vector
    pub fn jump_2(mut nums: Vec<i32>) -> i32 {
        nums.pop();
        let mut n_jumps = 0;
        let mut l = 1;
        let mut r = match nums.first() {
            Some(v) => *v as usize,
            None => return 0,
        };
        loop {
            let mut _r = r;
            n_jumps += 1;
            for i in l..=r {
                if let Some(v) = nums.get(i) {
                    _r = _r.max(*v as usize + i);
                } else {
                    return n_jumps;
                };
            }
            l = r + 1;
            r = _r;
        }
    }
}

fn main() {
    for f in [Solution::jump_1, Solution::jump_2] {
        assert_eq!(f(vec![2, 3, 1, 1, 4]), 2);
        assert_eq!(f(vec![2, 3, 0, 1, 4]), 2);
        assert_eq!(f(vec![0]), 0);
    }
}
