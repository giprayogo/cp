use std::collections::HashMap;

struct Solution;

impl Solution {
    // Naive (a form of DP): still accepted but really slow!
    pub fn can_jump_1(nums: Vec<i32>) -> bool {
        let mut can = vec![false; nums.len()];
        match can.last_mut() {
            Some(v) => *v = true,
            None => return true,
        }
        for i in (0..(nums.len() - 1)).rev() {
            for n in (1..=nums[i]).rev() {
                if let Some(v) = can.get(i + n as usize) {
                    if *v {
                        can[i] = true;
                        break;
                    }
                }
            }
        }
        match can.first() {
            Some(v) => *v,
            None => true,
        }
    }

    // NOTE: If overflow definitely can jump duh...
    // Much faster but faaar from the fastest
    pub fn can_jump_1b(nums: Vec<i32>) -> bool {
        let mut can = vec![false; nums.len()];
        match can.last_mut() {
            Some(v) => *v = true,
            None => return true,
        }
        for i in (0..(nums.len() - 1)).rev() {
            for n in (1..=nums[i]).rev() {
                match can.get(i + n as usize) {
                    Some(v) => {
                        if *v {
                            can[i] = true;
                            break;
                        }
                    }
                    None => {
                        can[i] = true;
                        break;
                    }
                }
            }
        }
        match can.first() {
            Some(v) => *v,
            None => true,
        }
    }

    // Backtracking with preference to jump as far as possible
    // NOT TLE but no go...not to mention memory
    pub fn can_jump_2(nums: Vec<i32>) -> bool {
        let mut memo = HashMap::new();
        fn dfs(memo: &mut HashMap<usize, bool>, nums: &Vec<i32>, i: usize) -> bool {
            match memo.get(&i) {
                Some(v) => *v,
                None => {
                    if i == nums.len() - 1 {
                        return true;
                    }
                    let n = match nums.get(i) {
                        Some(v) => *v,
                        None => return false,
                    };
                    for _i in (1..=n).rev() {
                        if dfs(memo, nums, i + _i as usize) {
                            memo.insert(i, true);
                            return true;
                        };
                    }
                    memo.insert(i, false);
                    false
                }
            }
        }
        dfs(&mut memo, &nums, 0)
    }

    // same insight as 1b
    pub fn can_jump_2b(nums: Vec<i32>) -> bool {
        let mut memo = HashMap::new();
        fn dfs(memo: &mut HashMap<usize, bool>, nums: &Vec<i32>, i: usize) -> bool {
            match memo.get(&i) {
                Some(v) => *v,
                None => {
                    if i == nums.len() - 1 {
                        memo.insert(i, true);
                        return true;
                    }
                    let n = match nums.get(i) {
                        Some(v) => *v,
                        None => {
                            memo.insert(i, true);
                            return true;
                        }
                    };
                    for _i in (1..=n).rev() {
                        if dfs(memo, nums, i + _i as usize) {
                            memo.insert(i, true);
                            return true;
                        };
                    }
                    memo.insert(i, false);
                    false
                }
            }
        }
        dfs(&mut memo, &nums, 0)
    }

    // I just need to check the content: i + nums[i] becomes the max visited index;
    // less than that definitely can!
    // return true whenever it becomes >= last element; false if it never become such
    // YAAASHH
    // neetcode do it in reverse but... roughly the same really
    pub fn can_jump_3(nums: Vec<i32>) -> bool {
        let mut max_index = 0;
        for i in 0..nums.len() {
            max_index = max_index.max(nums[i] as usize + i);
            if max_index >= nums.len() - 1 {
                return true;
            }
            if i >= max_index {
                return false;
            }
        }
        unreachable!()
    }
}

fn main() {
    for f in [
        Solution::can_jump_1,
        Solution::can_jump_1b,
        Solution::can_jump_2,
        Solution::can_jump_2b,
        Solution::can_jump_3,
    ] {
        println!("======");
        assert!(f(vec![2]));
        assert!(f(vec![2, 3, 1, 1, 4]));
        assert!(!f(vec![3, 2, 1, 0, 4]));
        assert!(f(vec![2, 0]));
        assert!(f(vec![0]));
    }
}
