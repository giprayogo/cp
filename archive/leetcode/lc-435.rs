// use std::collections::BTreeMap;
// use std::collections::HashMap;

impl Solution {
    // wrong heuristics
    // pub fn erase_overlap_intervals_1(intervals: Vec<Vec<i32>>) -> i32 {
    //     let mut h = BTreeMap::new();
    //     for interval in intervals {
    //         if let [begin, end] = interval[..] {
    //             h.entry(begin).and_modify(|e| *e += 1).or_insert(1);
    //             h.entry(end).and_modify(|e| *e -= 1).or_insert(-1);
    //         }
    //     }
    //     let mut _s = 0;
    //     let mut max_s = 0;
    //     for s in h.values() {
    //         _s += s;
    //         max_s = max_s.max(_s);
    //     }
    //     max_s - 1
    // }

    // DFS: correct (I hope) but TLE
    // pub fn erase_overlap_intervals_1(mut intervals: Vec<Vec<i32>>) -> i32 {
    //     let mut memo = HashMap::new();
    //     fn dfs(
    //         intervals: &Vec<Vec<i32>>,
    //         memo: &mut HashMap<(i32, usize, usize), i32>,
    //         erase: i32,
    //         i: usize,
    //         j: usize,
    //     ) -> i32 {
    //         println!("{i},{j} => {erase} | {}", memo.len());
    //         match memo.get(&(erase, i, j)) {
    //             Some(v) => *v,
    //             None => {
    //                 let left = intervals.get(i).unwrap();
    //                 let right = match intervals.get(j) {
    //                     Some(v) => v,
    //                     None => {
    //                         memo.insert((erase, i, j), erase);
    //                         return erase;
    //                     }
    //                 };
    //                 // NOTE; only works when sorted; NG?
    //                 // I want to make it to have sum of erase?
    //                 if left[1] > right[0] {
    //                     let erase = erase + 1;
    //                     let del_j = dfs(intervals, memo, erase, i, j + 1); // del j
    //                     let del_i = dfs(intervals, memo, erase, j, j + 1); // del i
    //                     let min_del = del_j.min(del_i);
    //                     memo.insert((erase, i, j), min_del);
    //                     min_del
    //                 } else {
    //                     let next = dfs(intervals, memo, erase, j, j + 1);
    //                     memo.insert((erase, i, j), next);
    //                     next
    //                 }
    //             }
    //         }
    //     }
    //     intervals.sort();
    //     dfs(&intervals, &mut memo, 0, 0, 1)
    // }

    // this is actually similar to that scheduling problem? just complemented??
    // so bodoh. overthink again
    pub fn erase_overlap_intervals_2(intervals: Vec<Vec<i32>>) -> i32 {
        let mut intervals: Vec<(i32, i32)> = intervals
            .into_iter()
            .map(|v| {
                if let [a, b] = v[..] {
                    return (b, a);
                }
                unreachable!()
            })
            .collect();
        intervals.sort();
        let n = intervals.len();
        let mut n_selected = 0;
        let mut prev_end = i32::MIN;
        for (end, begin) in intervals {
            if begin >= prev_end {
                n_selected += 1;
                prev_end = end;
            }
        }
        (n - n_selected) as i32
    }
}

struct Solution;

fn main() {
    for f in [Solution::erase_overlap_intervals_2] {
        assert_eq!(f(vec![vec![1, 2], vec![2, 3], vec![3, 4], vec![1, 3]]), 1);
        assert_eq!(f(vec![vec![1, 2], vec![1, 2], vec![1, 2]]), 2);
        assert_eq!(f(vec![vec![1, 2], vec![2, 3]]), 0);
        assert_eq!(
            f(vec![
                vec![0, 2],
                vec![1, 3],
                vec![2, 4],
                vec![3, 5],
                vec![4, 6]
            ]),
            2
        );
        assert_eq!(
            f(vec![vec![1, 100], vec![11, 22], vec![1, 11], vec![2, 12]]),
            2
        );
        assert_eq!(
            f(vec![
                vec![-24, 99],
                vec![96, 98],
                vec![71, 96],
                vec![88, 99],
                vec![-24, 4],
                vec![99, 100],
                vec![-69, -27],
                vec![18, 28],
                vec![25, 38],
                vec![8, 25],
                vec![-99, 33],
                vec![-85, -30],
                vec![56, 64],
                vec![-77, 98],
                vec![-38, 88],
                vec![-96, 6],
                vec![91, 92],
                vec![-39, 80],
                vec![-7, 97],
                vec![-82, 9],
                vec![-80, 3],
                vec![87, 94],
                vec![-96, 16],
                vec![-15, 40],
                vec![-40, 86],
                vec![31, 81],
                vec![97, 98],
                vec![69, 83],
                vec![-40, 47],
                vec![1, 82],
                vec![13, 44],
                vec![-92, -65],
                vec![51, 80],
            ]),
            24
        );
        f(vec![
            vec![-79, 65],
            vec![-12, 89],
            vec![80, 86],
            vec![-62, 31],
            vec![-83, -63],
            vec![40, 45],
            vec![-11, 70],
            vec![16, 19],
            vec![40, 92],
            vec![-16, 71],
            vec![-45, 21],
            vec![-38, 85],
            vec![-84, -58],
            vec![74, 83],
            vec![-52, -5],
            vec![-21, 84],
            vec![-10, -1],
            vec![-47, -40],
            vec![-45, 48],
            vec![-12, 68],
            vec![-65, -2],
            vec![-17, 43],
            vec![-56, -32],
            vec![44, 49],
            vec![8, 20],
            vec![59, 91],
            vec![44, 83],
            vec![38, 99],
            vec![78, 93],
            vec![84, 85],
            vec![-45, 38],
            vec![-77, 78],
            vec![-71, 64],
            vec![-69, 97],
            vec![21, 85],
            vec![-56, 28],
            vec![-12, -7],
            vec![54, 76],
            vec![-8, -4],
            vec![54, 98],
            vec![69, 76],
            vec![7, 64],
            vec![-62, 12],
            vec![77, 90],
            vec![-58, 34],
            vec![52, 80],
            vec![66, 67],
            vec![-58, 91],
            vec![-26, 97],
            vec![32, 57],
            vec![-95, -37],
            vec![-36, -30],
            vec![32, 91],
            vec![84, 92],
            vec![87, 97],
            vec![43, 86],
        ]);
    }
}
