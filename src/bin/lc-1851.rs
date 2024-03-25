// use std::cmp::Reverse;
// use std::collections::{BTreeMap, BTreeSet, BinaryHeap};
use std::collections::{BTreeMap, BTreeSet};

impl Solution {
    // Simple but TLE! since this *should* be more efficient than filling intervals... did I miss other tricks?
    pub fn min_interval(mut intervals: Vec<Vec<i32>>, queries: Vec<i32>) -> Vec<i32> {
        intervals.sort();
        let mut sizes = Vec::with_capacity(intervals.len());
        let mut begins = Vec::with_capacity(intervals.len());
        let mut ends = Vec::with_capacity(intervals.len());
        for interval in intervals.iter() {
            if let [begin, end] = interval[..] {
                begins.push(begin);
                ends.push(end);
                sizes.push(end - begin + 1);
            }
        }
        let mut min_intervals = Vec::with_capacity(queries.len());
        for q in queries {
            let j = begins.partition_point(|x| x <= &q);
            let mut min_interval = i32::MAX;
            for i in 0..j {
                if ends[i] >= q {
                    min_interval = min_interval.min(sizes[i]);
                }
            }
            if min_interval == i32::MAX {
                min_interval = -1;
            }
            min_intervals.push(min_interval);
        }
        min_intervals
    }

    // Accepted but slooooow
    pub fn min_interval_2(intervals: Vec<Vec<i32>>, queries: Vec<i32>) -> Vec<i32> {
        let mut stack = Vec::with_capacity(intervals.len() * 2);
        for interval in intervals {
            if let [begin, end] = interval[..] {
                let size = end - begin + 1;
                stack.push((begin, begin, size));
                stack.push((end + 1, begin, size));
            }
        }
        stack.sort();
        let mut sizes = BTreeMap::new();
        let mut h = BTreeSet::new();
        for (i, j, size) in stack.into_iter() {
            if i == j {
                h.insert((size, i));
            } else {
                h.remove(&(size, j));
            }
            if let Some(v) = h.first() {
                sizes
                    .entry(i)
                    .and_modify(|e: &mut i32| *e = v.0)
                    .or_insert(v.0);
            } else {
                sizes
                    .entry(i)
                    .and_modify(|e: &mut i32| *e = -1) // is this correct??
                    .or_insert(-1);
            }
        }
        let starts: Vec<_> = sizes.keys().cloned().collect();
        let mut res = Vec::with_capacity(queries.len());
        for q in queries {
            let i = starts.partition_point(|x| x < &q);
            if starts[i] == q {
                res.push(sizes[&starts[i]]);
            } else {
                match i.checked_sub(1) {
                    Some(i) => res.push(sizes[&starts[i]]),
                    None => res.push(-1),
                }
            }
        }
        res
    }
}

struct Solution;

fn main() {
    for f in [Solution::min_interval, Solution::min_interval_2] {
        assert_eq!(
            f(
                vec![vec![1, 4], vec![2, 4], vec![3, 6], vec![4, 4]],
                vec![2, 3, 4, 5]
            ),
            [3, 3, 1, 4]
        );
        assert_eq!(
            f(
                vec![vec![2, 3], vec![2, 5], vec![1, 8], vec![20, 25]],
                vec![2, 19, 5, 22]
            ),
            [2, -1, 4, 6]
        );
        assert_eq!(
            f(
                vec![vec![4, 5], vec![5, 8], vec![1, 9], vec![8, 10], vec![1, 6]],
                vec![7, 9, 3, 9, 3]
            ),
            [4, 3, 6, 3, 6]
        );
        assert_eq!(
            f(
                vec![vec![9, 9], vec![6, 7], vec![5, 6], vec![2, 5], vec![3, 3]],
                vec![6, 1, 1, 1, 9],
            ),
            [2, -1, -1, -1, 1]
        );
    }
}
