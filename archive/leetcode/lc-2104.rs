impl Solution {
    // Accepted but not fast: better solution?
    fn sub_array_ranges(nums: Vec<i32>) -> i64 {
        if nums.len() == 1 {
            return 0;
        }
        let mut sum = 0;
        let mut mmax = Vec::with_capacity(nums.len() - 1);
        for i in 0..nums.len() - 1 {
            let left = nums[i] as i64;
            let right = nums[i + 1] as i64;
            if left < right {
                mmax.push((left, right));
                sum += right - left;
            } else {
                mmax.push((right, left));
                sum += left - right;
            }
        }

        let mut size = 2;
        while size < nums.len() {
            let _skip = mmax.len();
            let mut _mmax = Vec::with_capacity(nums.len() - size);
            for i in 0..(mmax.len() - 1) {
                let left = mmax[i];
                let right = mmax[i + 1];
                let _left = left.0.min(right.0);
                let _right = left.1.max(right.1);
                sum += _right - _left;
                _mmax.push((_left, _right));
            }
            mmax = _mmax;
            size += 1;
        }
        sum
    }
}
struct Solution;
fn main() {
    assert_eq!(Solution::sub_array_ranges(vec![1, 2, 3]), 4);
    assert_eq!(Solution::sub_array_ranges(vec![1, 3, 3]), 4);
    assert_eq!(Solution::sub_array_ranges(vec![4, -2, -3, 4, 1]), 59);
}
