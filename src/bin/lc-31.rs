impl Solution {
    fn search(nums: Vec<i32>, target: i32) -> i32 {
        let mut l = 0;
        let mut r = nums.len() - 1;
        let largest_small = *nums.last().unwrap();
        let smallest_large = *nums.first().unwrap();
        let mut shift = 0;

        let search_range = 'sr: {
            if smallest_large < largest_small {
                0..nums.len()
            } else {
                while l != r {
                    let m = (l + r).div_ceil(2);
                    match nums.get(m) {
                        Some(v) => {
                            if *v > smallest_large {
                                l = m;
                            } else {
                                r = m - 1;
                            }
                        }
                        None => break 'sr 0..r,
                    }
                }
                l = if nums[l] >= smallest_large { l + 1 } else { l };
                if target >= smallest_large {
                    0..l
                } else {
                    shift += l;
                    l..nums.len()
                }
            }
        };
        match nums[search_range].binary_search(&target) {
            Ok(i) => (i + shift) as i32,
            Err(_) => -1,
        }
    }
}
struct Solution;
fn main() {
    Solution::search(vec![7, 8, 1, 2, 3, 4, 5, 6], 2);
    assert_eq!(Solution::search(vec![3, 1], 3), 0);
    assert_eq!(Solution::search(vec![1, 3], 3), 1);
    assert_eq!(Solution::search(vec![3, 5, 1], 0), -1);
    assert_eq!(Solution::search(vec![3, 1], 1), 1);
    assert_eq!(Solution::search(vec![3, 1], 0), -1);
    assert_eq!(Solution::search(vec![4, 5, 6, 7, 0, 1, 2], 0), 4);
    assert_eq!(Solution::search(vec![4, 5, 6, 7, 0, 1, 2], 3), -1);
    assert_eq!(Solution::search(vec![1], 0), -1);
}
