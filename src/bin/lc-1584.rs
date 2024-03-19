// use std::{collections::HashSet, vec};

struct Solution;

// later use Disjoint-Set structure (a bit awkward in rust... or is it?)
// struct DJSNode<'a> {
//     parent: Option<&'a DJSNode<'a>>,
// }

// impl<'a> DJSNode<'a> {
//     fn new() -> Self {
//         let mut node = DJSNode { parent: None };
//         node.find();
//         node
//     }

//     fn find(&mut self) -> &'a DJSNode {
//         let parent = match self.parent {
//             Some(p) => p.find(),
//             None => self,
//         };
//         self.parent = Some(parent);
//         parent
//     }

//     fn union(&mut self) {}

//     fn size(&self) {}
// }

impl Solution {
    pub fn min_cost_connect_points(points: Vec<Vec<i32>>) -> i32 {
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

    pub fn min_cost_connect_points_2(points: Vec<Vec<i32>>) -> i32 {
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
    for f in [
        Solution::min_cost_connect_points,
        Solution::min_cost_connect_points_2,
    ] {
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
