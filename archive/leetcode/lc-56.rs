impl Solution {
    pub fn merge(intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut s = Vec::new();
        for v in intervals {
            if let [start, stop] = v[..] {
                s.push((start, 1));
                s.push((stop, -1));
            }
        }
        s.sort();
        let mut c = 0;
        let mut intervals = Vec::new();
        let mut interval = Vec::new();
        let mut _i = -1;
        for (i, _c) in s {
            if c == 0 {
                if i == _i {
                    interval.pop();
                } else if !interval.is_empty() {
                    intervals.push(interval);
                    interval = Vec::new();
                    interval.push(i);
                } else {
                    interval.push(i);
                }
            }
            c += _c;
            if c == 0 {
                interval.push(i);
                _i = i;
            }
        }
        if !interval.is_empty() {
            intervals.push(interval);
        }
        intervals
    }

    // inspiration from other leetcode solutions: sum-less way (not exactly the same though)
    pub fn merge2(mut intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        intervals.sort();
        let mut _intervals = Vec::new();
        // neetcode: or I can just modify the _intervals directly too!
        let (mut b, mut e) = (intervals[0][0], intervals[0][1]);
        for interval in intervals[1..].iter() {
            if let [start, stop] = interval[..] {
                if start <= e {
                    e = e.max(stop);
                } else {
                    _intervals.push(vec![b, e]);
                    b = start;
                    e = stop;
                }
            }
        }
        _intervals.push(vec![b, e]);
        _intervals
    }
}

struct Solution;

fn main() {
    for f in [Solution::merge, Solution::merge2] {
        assert_eq!(
            f(vec![vec![1, 3], vec![2, 6], vec![8, 10], vec![15, 18]]),
            [[1, 6], [8, 10], [15, 18]]
        );
        assert_eq!(f(vec![vec![1, 4], vec![4, 5]]), [[1, 5]]);
        assert_eq!(f(vec![vec![1, 4], vec![1, 5]]), [[1, 5]]);
        assert_eq!(f(vec![vec![1, 4]]), [[1, 4]]);
    }
}
