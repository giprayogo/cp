use std::collections::{HashMap, HashSet};

impl Solution {
    pub fn largest_path_value(colors: String, edges: Vec<Vec<i32>>) -> i32 {
        let colors: Vec<char> = colors.chars().collect();
        let mut adj_list = HashMap::new();
        let mut s: HashSet<i32> = (0..colors.len() as i32).collect();
        if edges.is_empty() {
            return 1;
        }
        for edge in edges {
            if let [from, to] = edge[..] {
                adj_list.entry(from).or_insert_with(Vec::new).push(to);
                adj_list.entry(to).or_insert_with(Vec::new);
                s.remove(&to);
            }
        }
        if s.is_empty() {
            return -1;
        }

        fn dfs(
            colors: &Vec<char>,
            adj_list: &HashMap<i32, Vec<i32>>,
            config: &mut HashMap<char, i32>,
            visited: &mut HashSet<i32>,
            node: i32,
        ) -> i32 {
            if visited.contains(&node) {
                return -1;
            }
            visited.insert(node);
            let mut value = 0;
            if adj_list[&node].is_empty() {
                for v in config.values() {
                    if *v > value {
                        value = *v;
                    }
                }
            } else {
                for next in adj_list[&node].iter() {
                    let color = colors[*next as usize];
                    *config.entry(color).or_default() += 1;
                    let _value = dfs(colors, adj_list, config, visited, *next);
                    if _value == -1 {
                        visited.remove(&node);
                        return -1;
                    }
                    config.entry(color).and_modify(|e| *e -= 1);
                    value = value.max(_value);
                }
            }
            visited.remove(&node);
            value
        }
        let mut value = 0;
        for _s in s {
            let mut config = HashMap::new();
            config.entry(colors[_s as usize]).or_insert(1);
            let _value = dfs(&colors, &adj_list, &mut config, &mut HashSet::new(), _s);
            if _value == -1 {
                return value;
            }
            value = value.max(_value);
        }
        value
    }
}

struct Solution;

fn main() {
    Solution::largest_path_value(
        "qddqqqddqqdqddddddqdqqddddqdqdqqdddqddqdqqdqqqqqddqddqqddqqqdqqqqdqdddddqdq".to_string(),
        vec![
            vec![0, 1],
            vec![1, 2],
            vec![2, 3],
            vec![3, 4],
            vec![3, 5],
            vec![4, 5],
            vec![3, 6],
            vec![5, 6],
            vec![6, 7],
            vec![5, 7],
            vec![3, 7],
            vec![6, 8],
            vec![5, 8],
            vec![4, 8],
            vec![8, 9],
            vec![9, 10],
            vec![10, 11],
            vec![9, 11],
            vec![9, 12],
            vec![11, 12],
            vec![6, 12],
            vec![11, 13],
            vec![9, 13],
            vec![13, 14],
            vec![12, 14],
            vec![10, 14],
            vec![11, 14],
            vec![13, 15],
            vec![14, 15],
            vec![12, 16],
            vec![9, 16],
            vec![7, 16],
            vec![15, 17],
            vec![13, 17],
            vec![17, 18],
            vec![11, 18],
            vec![17, 19],
            vec![18, 19],
            vec![13, 19],
            vec![17, 20],
            vec![18, 20],
            vec![19, 21],
            vec![17, 21],
            vec![12, 22],
            vec![21, 22],
            vec![16, 22],
            vec![22, 23],
            vec![21, 23],
            vec![16, 24],
            vec![22, 24],
            vec![15, 25],
            vec![24, 25],
            vec![20, 25],
            vec![12, 25],
            vec![23, 26],
            vec![26, 27],
            vec![13, 27],
            vec![27, 28],
            vec![21, 28],
            vec![26, 28],
            vec![28, 29],
            vec![15, 30],
            vec![27, 30],
            vec![24, 30],
            vec![21, 30],
            vec![27, 31],
            vec![30, 31],
            vec![25, 32],
            vec![29, 32],
            vec![17, 33],
            vec![31, 33],
            vec![32, 33],
            vec![25, 34],
            vec![33, 35],
            vec![31, 35],
            vec![34, 35],
            vec![30, 36],
            vec![35, 37],
            vec![36, 37],
            vec![26, 38],
            vec![36, 38],
            vec![34, 38],
            vec![37, 38],
            vec![38, 39],
            vec![22, 39],
            vec![39, 40],
            vec![40, 41],
            vec![38, 41],
            vec![20, 41],
            vec![41, 42],
            vec![37, 42],
            vec![40, 43],
            vec![42, 43],
            vec![43, 44],
            vec![41, 44],
            vec![32, 44],
            vec![38, 44],
            vec![39, 44],
            vec![43, 45],
            vec![44, 45],
            vec![44, 46],
            vec![45, 46],
            vec![45, 47],
            vec![42, 47],
            vec![43, 48],
            vec![45, 49],
            vec![45, 50],
            vec![48, 51],
            vec![30, 51],
            vec![46, 52],
            vec![48, 52],
            vec![38, 52],
            vec![51, 52],
            vec![47, 53],
            vec![45, 53],
            vec![53, 54],
            vec![48, 54],
            vec![30, 54],
            vec![50, 55],
            vec![30, 55],
            vec![36, 55],
            vec![55, 56],
            vec![39, 56],
            vec![54, 56],
            vec![50, 57],
            vec![56, 58],
            vec![32, 58],
            vec![57, 59],
            vec![49, 59],
            vec![38, 60],
            vec![60, 61],
            vec![35, 61],
            vec![54, 61],
            vec![53, 61],
            vec![54, 62],
            vec![58, 62],
            vec![62, 63],
            vec![40, 63],
            vec![58, 63],
            vec![49, 64],
            vec![63, 64],
            vec![47, 64],
            vec![39, 64],
            vec![45, 64],
            vec![62, 65],
            vec![64, 65],
            vec![54, 65],
            vec![52, 66],
            vec![61, 66],
            vec![60, 66],
            vec![55, 67],
            vec![65, 67],
            vec![45, 68],
            vec![56, 68],
            vec![36, 68],
            vec![67, 69],
            vec![66, 69],
            vec![27, 70],
            vec![60, 70],
            vec![67, 70],
            vec![48, 71],
            vec![70, 71],
            vec![53, 71],
            vec![62, 72],
            vec![72, 73],
            vec![73, 74],
        ],
    );
    assert_eq!(
        Solution::largest_path_value(
            "iivvvvv".to_string(),
            vec![
                vec![0, 1],
                vec![1, 2],
                vec![1, 3],
                vec![2, 3],
                vec![3, 4],
                vec![2, 4],
                vec![3, 5],
                vec![1, 5],
                vec![4, 5],
                vec![5, 6]
            ]
        ),
        5
    );
    assert_eq!(
        Solution::largest_path_value(
            "hhqhuqhqff".to_string(),
            vec![
                vec![0, 1],
                vec![0, 2],
                vec![2, 3],
                vec![3, 4],
                vec![3, 5],
                vec![5, 6],
                vec![2, 7],
                vec![6, 7],
                vec![7, 8],
                vec![3, 8],
                vec![5, 8],
                vec![8, 9],
                vec![3, 9],
                vec![6, 9]
            ]
        ),
        3
    );
    assert_eq!(
        Solution::largest_path_value(
            "abaca".to_string(),
            vec![vec![0, 1], vec![0, 2], vec![2, 3], vec![3, 4]],
        ),
        3
    );
    assert_eq!(
        Solution::largest_path_value("a".to_string(), vec![vec![0, 0]]),
        -1
    );
}
