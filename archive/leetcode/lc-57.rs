impl Solution {
    // not so memory efficient but 100% on speed! Are there better heuristic though?
    pub fn insert1(mut intervals: Vec<Vec<i32>>, new_interval: Vec<i32>) -> Vec<Vec<i32>> {
        let mut h = Vec::new();
        intervals.push(new_interval);
        for i in intervals.into_iter() {
            if let [start, stop] = i[..] {
                h.push((start, 1));
                h.push((stop, -1));
            }
        }
        h.sort();
        let mut c = 0;
        let mut v = Vec::new();
        let mut intervals = Vec::new();
        for (i, _c) in h {
            if c == 0 {
                if let Some(&_i) = v.last() {
                    if _i == i {
                        v.pop();
                    } else {
                        intervals.push(v);
                        v = Vec::new();
                        v.push(i);
                    }
                } else {
                    v.push(i);
                }
            }
            c += _c;
            if c == 0 {
                v.push(i);
            }
        }
        if !v.is_empty() {
            intervals.push(v);
        }
        intervals
    }

    // neetcode: stupid: just check where the new interval lies w.r.t. prev intervals
    // anywya there are multiple other heuristics
    pub fn insert2(intervals: Vec<Vec<i32>>, mut new_interval: Vec<i32>) -> Vec<Vec<i32>> {
        let mut _intervals = Vec::new();
        for (i, interval) in intervals.iter().enumerate() {
            if new_interval[1] < interval[0] {
                // left of current, right of previous
                _intervals.push(new_interval);
                _intervals.extend_from_slice(&intervals[i..]);
                return _intervals;
            } else if new_interval[0] > interval[1] {
                // right of current
                _intervals.push(interval.clone());
            } else {
                // overlap: meld as may cover the next ones too
                new_interval = vec![
                    interval[0].min(new_interval[0]),
                    interval[1].max(new_interval[1]),
                ];
            }
        }
        _intervals.push(new_interval);
        _intervals
    }
}

struct Solution;

fn main() {
    for f in [Solution::insert1, Solution::insert2] {
        assert_eq!(
            f(vec![vec![1, 3], vec![6, 9]], vec![2, 5]),
            [[1, 5], [6, 9]]
        );
        assert_eq!(
            f(
                vec![
                    vec![1, 2],
                    vec![3, 5],
                    vec![6, 7],
                    vec![8, 10],
                    vec![12, 16]
                ],
                vec![4, 8]
            ),
            [[1, 2], [3, 10], [12, 16]]
        );
        assert_eq!(f(vec![vec![1, 5]], vec![0, 0]), [[0, 0], [1, 5]]);
        assert_eq!(f(vec![vec![1, 5]], vec![1, 7]), [[1, 7]]);
    }
}
