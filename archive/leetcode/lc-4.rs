impl Solution {
    // O((m+n) log(m+n)) solution (0ms lol)
    fn find_median_sorted_arrays(mut nums1: Vec<i32>, mut nums2: Vec<i32>) -> f64 {
        nums1.append(&mut nums2);
        nums1.sort();
        let mid = nums1.len() / 2;
        if nums1.len() % 2 == 1 {
            nums1[mid] as f64
        } else {
            (nums1[mid] + nums1[mid - 1]) as f64 / 2.0
        }
    }

    // Proper O(log (m+n)) solution?
    // fn find_median_sorted_arrays_2(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {}
}

struct Solution;

fn main() {
    assert_eq!(
        Solution::find_median_sorted_arrays(vec![1, 3], vec![2]),
        2.0
    );
    assert_eq!(
        Solution::find_median_sorted_arrays(vec![1, 2], vec![3, 4]),
        2.5
    );
}
