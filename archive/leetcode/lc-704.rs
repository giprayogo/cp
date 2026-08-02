use std::cmp::Ordering;

fn main() {}

#[allow(unused)]
struct Solution;

#[allow(unused)]
impl Solution {
    // more standard implementation
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let mut l = 0;
        let mut r = nums.len();
        while l < r {
            let m = (l + r) / 2;
            println!("{l},{r}");
            match nums[m].cmp(&target) {
                Ordering::Equal => { return m as i32 },
                Ordering::Greater => { r = m },
                Ordering::Less => { l = m + 1 },
            }
        }
        -1
    }
    // my shitty first implementation
    // pub fn search(nums: Vec<i32>, target: i32) -> i32 {
    //     let mut stack = vec![];
    //     stack.push((0usize, nums.len()));
    //     while let Some(e) = stack.pop() {
    //         let l = e.0;
    //         let r = e.1;
    //         println!("{l},{r}");

    //         let m = (l + r) / 2;
    //         match nums[m].cmp(&target) {
    //             Ordering::Equal => { return m as i32 },
    //             Ordering::Greater => { if r-l > 1 {stack.push((l, m))} },
    //             Ordering::Less => { if r-l > 1 {stack.push((m, r))} },
    //         }
    //     }
    //     -1
    // }
}

#[test]
fn test_solution() {
    assert_eq!(Solution::search(vec![-1, 0, 3, 5, 9, 12], 9), 4);
    assert_eq!(Solution::search(vec![-1,0,3,5,9,12], 2), -1);
}
