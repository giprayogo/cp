use std::cmp::Ordering::{Equal, Greater, Less};

fn main() {}

#[allow(unused)]
struct Solution;

#[allow(unused)]
impl Solution {
    pub fn min_eating_speed(piles: Vec<i32>, h: i32) -> i32 {
        let mut l = 1;
        let mut r = 10i32.pow(9) + 1;  // can use max too; but that's another O(n)
        // let mut answers = Vec::new();
        while l < r {
            let m = (l + r) / 2;
            let eathr = piles.iter().map(|x| x/m + (x%m != 0) as i32).sum();
            match h.cmp(&eathr) {
                // Equal | Greater => { r = m; answers.push(m) },
                Equal | Greater => { r = m },
                Less => { l = m+1 },
            }
        }
        r
        // answers.into_iter().min().unwrap()
    }
}

#[test]
fn test_solution() {
    assert_eq!(Solution::min_eating_speed(vec![3, 6, 7, 11], 8), 4);
    assert_eq!(Solution::min_eating_speed(vec![30, 11, 23, 4, 20], 5), 30);
    assert_eq!(Solution::min_eating_speed(vec![30, 11, 23, 4, 20], 6), 23);
    assert_eq!(Solution::min_eating_speed(vec![1000000000,1000000000], 3), 1000000000);
}
