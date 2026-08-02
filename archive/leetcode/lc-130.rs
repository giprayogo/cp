// use std::collections::HashSet;
struct Solution;

impl Solution {
    // Try other's different way of marking
    // Lesson: try to find rules to -not- check things.
    // Looping several times is preferrable to executing unnecessary DFS!
    // Also if it is ok to mutate the array, do it rather than creating other structures
    pub fn solve(board: &mut Vec<Vec<char>>) {
        let ni = board.len();
        let nj = board.first().unwrap().len();

        fn dfs(board: &mut Vec<Vec<char>>, i: Option<usize>, j: Option<usize>) {
            let i = match i {
                Some(i) => i,
                None => return,
            };
            let j = match j {
                Some(j) => j,
                None => return,
            };
            if board[i][j] != 'O' {
                return;
            }
            board[i][j] = 'T';

            let _i = i.checked_sub(1);
            let _j = j.checked_sub(1);
            let __i = (i + 1 < board.len()).then_some(i + 1);
            let __j = (j + 1 < board.first().unwrap().len()).then_some(j + 1);
            dfs(board, _i, Some(j));
            dfs(board, __i, Some(j));
            dfs(board, Some(i), _j);
            dfs(board, Some(i), __j);
        }

        for i in [0, ni - 1] {
            for j in 0..nj {
                dfs(board, Some(i), Some(j));
            }
        }
        for i in 0..ni {
            for j in [0, nj - 1] {
                dfs(board, Some(i), Some(j));
            }
        }
        for i in 0..ni {
            for j in 0..nj {
                if board[i][j] == 'O' {
                    board[i][j] = 'X';
                } else if board[i][j] == 'T' {
                    board[i][j] = 'O';
                }
            }
        }
    }
    // Accepted but (relatively) slow: did I missed something?
    // Other's solution are similar! Just different way of marking.
    // pub fn solve(board: &mut Vec<Vec<char>>) {
    //     let ni = board.len();
    //     let nj = board.first().unwrap().len();
    //     let mut nodes = Vec::new();
    //     let mut visited = HashSet::new();

    //     fn dfs(
    //         board: &Vec<Vec<char>>,
    //         nodes: &mut Vec<(usize, usize)>,
    //         i: Option<usize>,
    //         j: Option<usize>,
    //     ) -> bool {
    //         let i = match i {
    //             Some(i) => i,
    //             None => return true,
    //         };
    //         let j = match j {
    //             Some(j) => j,
    //             None => return true,
    //         };
    //         let coordinate = (i, j);
    //         if board[i][j] == 'X' {
    //             return false;
    //         }
    //         if nodes.contains(&coordinate) {
    //             return false;
    //         }
    //         nodes.push(coordinate);

    //         let _i = i.checked_sub(1);
    //         let _j = j.checked_sub(1);
    //         let __i = (i + 1 < board.len()).then_some(i + 1);
    //         let __j = (j + 1 < board.first().unwrap().len()).then_some(j + 1);
    //         let down = dfs(board, nodes, _i, Some(j));
    //         let up = dfs(board, nodes, __i, Some(j));
    //         let left = dfs(board, nodes, Some(i), _j);
    //         let right = dfs(board, nodes, Some(i), __j);
    //         down || up || left || right
    //     }

    //     for i in 0..ni {
    //         for j in 0..nj {
    //             if board[i][j] == 'O' && !visited.contains(&(i, j)) {
    //                 let dont_flip = dfs(board, &mut nodes, Some(i), Some(j));
    //                 if !dont_flip {
    //                     for (_i, _j) in nodes.drain(..) {
    //                         board[_i][_j] = 'X';
    //                         visited.insert((_i, _j));
    //                     }
    //                 } else {
    //                     visited.extend(nodes.drain(..));
    //                 }
    //             }
    //         }
    //     }
    // }
}

fn main() {
    let mut board = vec![
        vec!['X', 'X', 'X', 'X'],
        vec!['X', 'O', 'O', 'X'],
        vec!['X', 'X', 'O', 'X'],
        vec!['X', 'O', 'X', 'X'],
    ];
    Solution::solve(&mut board);
    assert_eq!(
        board,
        vec![
            vec!['X', 'X', 'X', 'X'],
            vec!['X', 'X', 'X', 'X'],
            vec!['X', 'X', 'X', 'X'],
            vec!['X', 'O', 'X', 'X']
        ]
    );
    let mut board = vec![
        vec!['O', 'X', 'X', 'X'],
        vec!['O', 'X', 'O', 'X'],
        vec!['O', 'O', 'X', 'X'],
        vec!['O', 'O', 'X', 'X'],
    ];
    Solution::solve(&mut board);
    assert_eq!(
        board,
        vec![
            vec!['O', 'X', 'X', 'X'],
            vec!['O', 'X', 'X', 'X'],
            vec!['O', 'O', 'X', 'X'],
            vec!['O', 'O', 'X', 'X']
        ]
    );
    board = vec![vec!['X']];
    Solution::solve(&mut board);
    assert_eq!(board, vec![vec!['X']]);
    board = vec![vec!['O']];
    Solution::solve(&mut board);
    assert_eq!(board, vec![vec!['O']]);
}
