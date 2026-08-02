use std::cmp::Ordering;

#[allow(unused)]
struct Solution;

#[allow(unused)]
impl Solution {
    // heuristic was wrong; anyway just use normal indexing.
    pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
        let mut i = 0;
        let mut j = 1;
        'search: while j < numbers.len() {
            let l = numbers[i];
            let r = numbers[j];
            match (r + l).cmp(&target) {
                Ordering::Less => {
                    while i < j - 1 && (r + l) < target {
                        i += 1;
                        let l = numbers[i];
                        if r + l == target {
                            break 'search;
                        }
                    }
                }
                Ordering::Greater => {
                    while i > 0 && (r + l) > target {
                        i -= 1;
                        let l = numbers[i];
                        if r + l == target {
                            break 'search;
                        }
                    }
                }
                Ordering::Equal => break 'search,
            }
            j += 1;
        }
        vec![(i + 1) as i32, (j + 1) as i32]
    }
    // wtf some old rust compiler on leetcode
    // pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
    //     let mut li = numbers.iter().enumerate();
    //     let mut jl = li.next().unwrap();
    //     for (i, r) in numbers.iter().enumerate().skip(1) {
    //         let mut j = jl.0;
    //         let mut l = jl.1;
    //         println!("{j}, {i} = {l}-{r}");
    //         loop {
    //             if r + l == target {
    //                 return vec![j as i32 + 1, i as i32 + 1];
    //             }
    //             if j == (i - 1) {
    //                 break;
    //             }
    //             jl = li.next().unwrap();
    //             j = jl.0;
    //             l = jl.1;
    //             println!("{j}, {i} = {l}-{r}");
    //         }
    //     }
    //     unreachable!()
    // }
    // no need left right!
    // pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
    //     let mut li = numbers.iter().enumerate();
    //     let (mut j, mut l) = li.next().unwrap();
    //     for (i, r) in numbers.iter().enumerate().skip(1) {
    //         loop {
    //             if r + l == target {
    //                 return vec![j as i32 + 1, i as i32 + 1];
    //             }
    //             if j == (i - 1) {
    //                 break;
    //             }
    //             (j, l) = li.next().unwrap();
    //         }
    //     }
    //     unreachable!()
    // }
    // rough n2
    // pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
    //     let (mut i, mut j) = (0, 0);
    //     for (i, r) in numbers.iter().enumerate() {
    //         for (j, l) in numbers.iter().enumerate() {
    //             if j == i {
    //                 break;
    //             }
    //             if r + l == target {
    //                 return vec![j as i32 + 1, i as i32 + 1];
    //             }
    //         }
    //     }
    //     unreachable!()
    // }
}

fn main() {}

#[test]
fn test_solution() {
    let numbers = vec![2, 7, 11, 15];
    assert_eq!(Solution::two_sum(numbers, 9), vec![1, 2]);
    let numbers = vec![2, 3, 4];
    assert_eq!(Solution::two_sum(numbers, 6), vec![1, 3]);
    let numbers = vec![-1, 0];
    assert_eq!(Solution::two_sum(numbers, -1), vec![1, 2]);
    let numbers = vec![3, 24, 50, 79, 88, 150, 345];
    assert_eq!(Solution::two_sum(numbers, 200), vec![3, 6]);
}
