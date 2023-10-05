use std::cmp::Ordering;
use std::collections::HashSet;

#[allow(unused)]
struct Solution;

#[allow(unused)]
impl Solution {
    /// neetcode nice (ugh! but still learn something by implementing this from memory)
    /// nums.len 3..=3000
    /// nums[i] -10^5..10^5
    pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        nums.sort_unstable();

        let mut po = 0;
        if nums[po] > 0 {
            return vec![];
        }

        let mut result = HashSet::new();
        for p in 0..nums.len() - 2 {
            let mut lo = p + 1;
            let mut ro = nums.len() - 1;

            loop {
                let mut l = lo;
                let mut r = ro;

                if l == r {
                    break;
                }

                let s = nums[p] + nums[l] + nums[r];
                match s.cmp(&0) {
                    Ordering::Equal => {
                        result.insert(vec![nums[p], nums[l], nums[r]]);
                        while nums[l] == nums[lo] && l < r {
                            l += 1;
                        }
                        lo = l;
                    }
                    Ordering::Greater => {
                        while nums[r] == nums[ro] && l < r {
                            r -= 1;
                        }
                        ro = r;
                    }
                    Ordering::Less => {
                        while nums[l] == nums[lo] && l < r{
                            l += 1;
                        }
                        lo = l;
                    }
                }
            }
        }
        result.into_iter().collect()
    }

    // naive cubic solution
    // pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
    //     let nl = nums.len();
    //     if nums.len() == 3 {
    //         if nums.iter().sum::<i32>() == 0 {
    //             return vec![nums];
    //         } else {
    //             return vec![];
    //         }
    //     }
    //     let mut result = HashSet::new();
    //     let mut nums = nums;
    //     nums.sort();
    //     for left in 0..(nl - 2) {
    //         for right in (left + 2)..nl {
    //             for j in (left + 1)..right {
    //                 let v = vec![nums[left], nums[j], nums[right]];
    //                 if v.iter().sum::<i32>() == 0 {
    //                     result.insert(v);
    //                 }
    //             }
    //         }
    //     }
    //     result.into_iter().collect()
    // }
}

fn main() {}

#[test]
fn test_solution() {
    // a bit annoying to test so let's just print
    println!("{:?}", Solution::three_sum(vec![-1, 0, 1, 2, -1, 4]));
    println!("{:?}", Solution::three_sum(vec![0, 1, 1]));
    println!("{:?}", Solution::three_sum(vec![0, 0, 0]));
    println!("{:?}", Solution::three_sum(vec![0, 0, 0, 0]));
    println!("{:?}", Solution::three_sum(vec![-2,0,1,1,2]));
}
