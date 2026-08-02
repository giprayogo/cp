use std::cmp::min;

#[allow(unused)]
struct Solution;

#[allow(unused)]
impl Solution {
    // use iterator!
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut ma = 0;
        let mut li = height.iter().enumerate();
        let mut ri = height.iter().enumerate().rev();

        let (mut l, mut hl) = li.next().unwrap();
        let (mut r, mut hr) = ri.next().unwrap();
        while l < r {
            let area = (r as i32 - l as i32) * min(hl, hr);
            if area > ma {
                ma = area;
            }
            if hl < hr {
                (l, hl) = li.next().unwrap();
            } else {
                (r, hr) = ri.next().unwrap();
            }
        }
        ma
    }
    // thanks discussion!
    // pub fn max_area(height: Vec<i32>) -> i32 {
    //     let mut ma = 0;
    //     let mut l = 0;
    //     let mut r = height.len() - 1;

    //     while l < r {
    //         let hl = height[l];
    //         let hr = height[r];
    //         let area = (r as i32 - l as i32) * min(hl, hr);
    //         if area > ma {
    //             ma = area;
    //         }
    //         if hl < hr {
    //             l += 1;
    //         } else {
    //             r -= 1;
    //         }
    //     }
    //     ma
    // }
    // naive n2 (TLE indeed!)
    // pub fn max_area(height: Vec<i32>) -> i32 {
    //     let mut h = BinaryHeap::new();
    //     for l in 0..height.len() - 1 {
    //         for r in l + 1..height.len() {
    //             let area = (r as i32 - l as i32) * min(height[l], height[r]);
    //             h.push(area);
    //         }
    //     }
    //     h.pop().unwrap()
    // }
}

fn main() {}

#[test]
fn test_solution() {
    assert_eq!(Solution::max_area(vec![1, 8, 6, 2, 5, 4, 8, 3, 7]), 49);
    assert_eq!(Solution::max_area(vec![1, 1]), 1);
}
