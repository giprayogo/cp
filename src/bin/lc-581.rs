impl Solution {
    fn find_unsorted_subarray(nums: Vec<i32>) -> i32 {
        let mut largest_uptonow = i32::MIN;
        let mut smallest_ooe = i32::MAX;
        let mut last_ooe_index = 0;
        for (i, e) in nums.iter().enumerate() {
            if *e < largest_uptonow {
                smallest_ooe = smallest_ooe.min(*e);
                last_ooe_index = i;
            }
            largest_uptonow = largest_uptonow.max(*e);
        }
        if smallest_ooe == i32::MAX {
            return 0;
        }
        let mut smallest_ooe_index = 0;
        for (i, e) in nums.iter().enumerate() {
            if *e > smallest_ooe {
                smallest_ooe_index = i;
                break;
            }
        }
        (last_ooe_index - smallest_ooe_index + 1) as i32
    }
}
struct Solution;
fn main() {
    assert_eq!(
        Solution::find_unsorted_subarray(vec![1, 2, 3, 7, 4, 5, 6]),
        4
    );
    assert_eq!(
        Solution::find_unsorted_subarray(vec![2, 6, 4, 8, 10, 9, 15]),
        5
    );
    assert_eq!(Solution::find_unsorted_subarray(vec![1, 2, 3, 4]), 0);
    assert_eq!(Solution::find_unsorted_subarray(vec![1]), 0);
}
