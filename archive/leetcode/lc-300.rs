struct Solution;

impl Solution {
    // Greedy and binary search: I did see solution and neetcode for this
    pub fn length_of_lis(nums: Vec<i32>) -> i32 {
        let mut inc = Vec::new();
        for n in nums {
            match inc.last() {
                None => inc.push(n),
                Some(v) => {
                    if n > *v {
                        inc.push(n);
                    } else {
                        match inc.binary_search(&n) {
                            Ok(_) => {}
                            Err(i) => inc[i] = n,
                        };
                    }
                }
            }
        }
        inc.len() as i32
    }

    // Accepted but much slower than many; missed a trick? Indeed this way it is n^2
    // Most people including neetcode uses this solution though...
    // Probably this is the best with DP! Later revisit after learning segment tree, etc.
    // pub fn length_of_lis(nums: Vec<i32>) -> i32 {
    //     let mut inc = vec![1; nums.len()];
    //     let mut max_inc = 1;
    //     for i in 0..nums.len() {
    //         for j in 0..i {
    //             if nums[i] > nums[j] {
    //                 inc[i] = inc[i].max(inc[j] + 1);
    //             }
    //         }
    //         max_inc = max_inc.max(inc[i]);
    //     }
    //     max_inc
    // }
}

fn main() {
    assert_eq!(Solution::length_of_lis(vec![10, 9, 2, 5, 3, 7, 101, 18]), 4);
    assert_eq!(Solution::length_of_lis(vec![0, 1, 0, 3, 2, 3]), 4);
    assert_eq!(Solution::length_of_lis(vec![7, 7, 7, 7, 7, 7, 7]), 1);
}
