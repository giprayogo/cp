impl Solution {
    fn find_min(nums: Vec<i32>) -> i32 {
        let is_left = |x| x > *nums.last().unwrap();
        let mut l = 0;
        let mut r = nums.len();
        while l < r {
            let m = l + (r - l) / 2;
            if is_left(nums[m]) {
                l = m + 1;
            } else {
                r = m;
            }
        }
        nums[r]
    }
}

// That the problem asked for a log(n) solution means it will be a variation of binary search.
//
// Rotation is defined below, if an array is rotation one time, it rolls one index to the right, e.g.
// [1,2,3,4,5] -> [5,1,2,3,4] (one rotation to the right)
//
// you can think of it as two arrays concatenated with the properties such that
// [ ] [   ]
// - monotonically increasing
// - no overlap in interval between two arrays
// - left most (smallest) element of the left > right most (largest) of the right
//
// The smallest element in the whole array is the left most element of
// the right subarray, so what we need to do is find
// the smallest element that belongs to the right array.
// We need a predicate that can distinguish between left and right array.
// Simple: check if the element is larger than the last element in the whole array.
//
// if an array is rotated n-times, it return to the original array
// so effectively the same as array rotated x / n times

struct Solution;

fn main() {
    assert_eq!(Solution::find_min(vec![3, 4, 5, 1, 2]), 1);
    assert_eq!(Solution::find_min(vec![4, 5, 6, 7, 0, 1, 2]), 0);
    assert_eq!(Solution::find_min(vec![11, 13, 15, 17]), 11);
}
