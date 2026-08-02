use std::collections::HashSet;

impl Solution {
    // Conclusion: bad heuristic (if condition) can make or break things
    #[allow(clippy::comparison_chain)]
    pub fn visible_mountains(peaks: Vec<Vec<i32>>) -> i32 {
        let mut _peaks = Vec::with_capacity(peaks.len());
        let mut vp = Vec::with_capacity(peaks.len());
        for peak in peaks {
            if let [x, y] = peak[..] {
                _peaks.push((y, x));
            }
        }
        _peaks.sort();
        let mut dups = HashSet::new();
        vp.push(*(_peaks.last().unwrap()));
        'peak: for (y, x) in _peaks[..(_peaks.len() - 1)].iter().rev() {
            for &(my, mx) in vp.iter() {
                let dx = (x - mx).abs();
                let dy = my - y;
                if *y != my && dy >= dx {
                    continue 'peak;
                } else if *y == my {
                    if *x == mx {
                        dups.insert((*y, *x));
                        continue 'peak;
                    } else {
                        vp.push((*y, *x));
                        continue 'peak;
                    }
                }
            }
            vp.push((*y, *x));
        }
        (vp.len() - dups.len()) as i32
    }
}

struct Solution;

fn main() {
    assert_eq!(
        Solution::visible_mountains(vec![vec![2, 2], vec![2, 2], vec![3, 1]]),
        0
    );
    assert_eq!(
        Solution::visible_mountains(vec![vec![2, 2], vec![6, 3], vec![5, 4]]),
        2
    );
    assert_eq!(Solution::visible_mountains(vec![vec![1, 3], vec![1, 3]]), 0);
    assert_eq!(
        Solution::visible_mountains(vec![
            vec![12, 10],
            vec![37, 25],
            vec![8, 12],
            vec![8, 36],
            vec![39, 4],
            vec![22, 3],
            vec![36, 19],
            vec![3, 17],
            vec![10, 19],
            vec![19, 38],
            vec![6, 36],
            vec![27, 23],
            vec![4, 29],
            vec![36, 27],
            vec![21, 28],
            vec![9, 11],
            vec![24, 1],
            vec![16, 17],
            vec![16, 9],
            vec![22, 23],
            vec![37, 31],
            vec![34, 17],
            vec![19, 2],
            vec![33, 3],
            vec![12, 14],
            vec![23, 7],
            vec![20, 36],
            vec![7, 36],
            vec![16, 7],
            vec![24, 38]
        ]),
        6
    )
}
