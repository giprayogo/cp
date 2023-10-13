#[allow(unused)]
struct Solution;

#[allow(unused)]
impl Solution {
    // neetcode solution
    // what I missed: if right-most is larger than left-most, I can simply use one pointer from the left
    // because it is the one limiting the volume!
    // if equal just choose one whatever
    // it is guaranteed to be correct!
    pub fn trap(height: Vec<i32>) -> i32 {
        let mut l = 0;
        let mut r = height.len() - 1;
        let mut hl = height[l];
        let mut hr = height[r];
        let mut vol = 0;
        while l < r {
            if hl <= hr {
                l += 1;
                hl = hl.max(height[l]);
                vol += hl - height[l];
            } else {
                r -= 1;
                hr = hr.max(height[r]);
                vol += hr - height[r];
            }
        }
        vol
    }
    // pub fn trap(height: Vec<i32>) -> i32 {
    //     let n = height.len();
    //     if n <= 2 {
    //         return 0;
    //     }

    //     let mut left = 0;
    //     let mut vol = 0;

    //     while left < n - 2 {
    //         let right = Solution::scan(&height, left);
    //         vol += Solution::accumulate(&height, left, right);
    //         left = right;
    //     }
    //     vol
    // }
    // /// accumulate volume of body of water between two peaks
    // fn accumulate(height: &[i32], i: usize, j: usize) -> i32 {
    //     let hi = height[i];
    //     let hj = height[j];

    //     let wl = hi.min(hj);
    //     let mut vol = 0;

    //     let forward = hi < hj;
    //     let mut k = if hi < hj { i } else { j };
    //     loop {
    //         let hk = height[k];
    //         vol += wl - hk;
    //         if forward {
    //             k += 1;
    //         } else {
    //             k -= 1;
    //         }
    //         if height[k] >= wl {
    //             break;
    //         }
    //     }
    //     vol
    // }

    // /// Return index where height is higher than h[i]
    // /// or the maximum local maxima
    // fn scan(height: &[i32], i: usize) -> usize {
    //     // let mut j = i + 1;
    //     // let mut jr = i + 1;
    //     let mut jr = height.len() - 1;
    //     let hi = height[i];
    //     let mut hpeak = 0;

    //     for j in i + 1..height.len() {
    //         let hj = height[j];
    //         if hj >= hi {
    //             return j;
    //         }
    //         if Solution::is_peak(height, j) && hj > hpeak {
    //             hpeak = hj;
    //             jr = j;
    //         }
    //     }
    //     jr
    // }
    // fn is_peak(height: &[i32], i: usize) -> bool {
    //     // i == height.len() - 1
    //     //     || (height[i+1] <= height[i]) && (height[i - 1] < height[i])
    //     (height[i - 1] < height[i]) && (height[i+1] <= height[i])
    // }
    // probably wrong heuristic (don't get trapped by it!)
    // pub fn trap(height: Vec<i32>) -> i32 {
    //     let n = height.len();
    //     if n <= 2 {
    //         return 0;
    //     }

    //     let mut left = 0;
    //     let mut right = 1;

    //     let mut vol = 0;
    //     let mut _vol = 0;

    //     while left < n - 2 {
    //         if right == n {
    //             left += 1;
    //             right = left + 1;
    //             _vol = 0;
    //         }

    //         let hl = height[left];
    //         let hr = height[right];
    //         if hl > hr {
    //             _vol += hl - hr;
    //             println!("{}|{} ({},{}) -> {}", hl, hr, left, right, _vol);
    //             right += 1;
    //         } else {
    //             left = right;
    //             right = left + 1;
    //             vol += _vol;
    //             println!("+{} -> {} | {},{}", _vol, vol, left, right);
    //             _vol = 0;
    //         }
    //     }
    //     vol
    // }
}

fn main() {}

#[test]
fn test_solution() {
    assert_eq!(Solution::trap(vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1]), 6);
    assert_eq!(Solution::trap(vec![4, 2, 0, 3, 2, 5]), 9);
    assert_eq!(Solution::trap(vec![4, 2, 3]), 1);
    assert_eq!(
        Solution::trap(vec![
            0, 1, 2, 0, 3, 0, 1, 2, 0, 0, 4, 2, 1, 2, 5, 0, 1, 2, 0, 2
        ]),
        26
    );
}
