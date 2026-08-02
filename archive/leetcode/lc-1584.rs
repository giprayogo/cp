// use std::{collections::HashSet, vec};

struct Solution;

// In Rust a little awkward because I can't do self reference
// I gave up implementing in Rust for this structure
// struct DisjointSet {
//     value: usize,
//     size: usize,
//     parent: Option<Box<DisjointSet>>,
// }

// impl DisjointSet {
//     fn new(value: usize) -> Self {
//         let mut d = DisjointSet {
//             value,
//             parent: None,
//             size: 1,
//         };
//         d.parent = Some(Box::new(d));
//         d
//     }

//     fn find(&mut self) -> Box<DisjointSet> {
//         let mut x = *self;
//         // while let Some(mut node) = x.parent {
//         //     let p = &mut node.parent;
//         //     // node.parent = match node.parent {
//         //     //     Some(_node) => _node.parent,
//         //     //     None => None,
//         //     // };
//         //     x = &mut node;
//         // }
//         Box::new(&self)
//     }

//     fn union(&mut self, other: &mut Self) {
//         let x = self.find();
//         let y = other.find();
//     }
// }

impl Solution {
    pub fn min_cost_connect_points_1(points: Vec<Vec<i32>>) -> i32 {
        let mut edges = Vec::new();
        for i in 0..points.len() {
            for j in 0..i {
                let cost =
                    points[i][0].abs_diff(points[j][0]) + points[i][1].abs_diff(points[j][1]);
                edges.push((i, j, cost))
            }
        }
        edges.sort_by_key(|k| k.2);
        let mut cost = 0;
        let mut p: Vec<_> = (0..points.len()).collect();
        for (i, j, _cost) in edges {
            let pi = p[i];
            let pj = p[j];
            if p[i] != p[j] {
                cost += _cost;
                let pij = p[i].min(p[j]);
                // Probably expensive; without disjoint set. not -that- bad tho
                for _p in p.iter_mut() {
                    if *_p == pi || *_p == pj {
                        *_p = pij;
                    }
                }
            }
        }
        cost as i32
    }
}

fn main() {
    for f in [Solution::min_cost_connect_points_1] {
        assert_eq!(
            f(vec![
                vec![0, 0],
                vec![2, 2],
                vec![3, 10],
                vec![5, 2],
                vec![7, 0]
            ]),
            20
        );
        assert_eq!(f(vec![vec![3, 12], vec![-2, 5], vec![-4, 1]]), 18);
        assert_eq!(
            f(vec![
                vec![2, -3],
                vec![-17, -8],
                vec![13, 8],
                vec![-17, -15]
            ]),
            53
        );
    }
}
