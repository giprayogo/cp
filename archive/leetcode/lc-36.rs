/// I went overboard and wanted an engineered solution!
use std::collections::HashSet;

#[allow(unused)]
struct Solution;

#[allow(unused)]
impl Solution {
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        for row in board.iter() {
            match Solution::row_nodups(row.iter()) {
                true => continue,
                false => return false,
            }
        }
        for j in 0..9 {
            let it = SudokuColumnIterator::new(&board, j);
            match Solution::row_nodups(it) {
                true => continue,
                false => return false,
            }
        }
        for n in 0..9 {
            let it = SudokuSquareIterator::new(&board, n);
            match Solution::row_nodups(it) {
                true => continue,
                false => return false,
            }
        }
        true
    }

    fn row_nodups<'a>(row: impl Iterator<Item = &'a char>) -> bool {
        let mut rowset = HashSet::new();
        for e in row {
            match e {
                '.' => continue,
                _ => {
                    let e = e.to_digit(10).unwrap();
                    if rowset.contains(&e) {
                        return false;
                    } else {
                        rowset.insert(e);
                    }
                }
            }
        }
        true
    }
}

struct SudokuColumnIterator<'a> {
    board: &'a Vec<Vec<char>>,
    i: usize,
    j: usize,
}

impl<'a> SudokuColumnIterator<'a> {
    fn new(board: &'a Vec<Vec<char>>, j: usize) -> Self {
        Self { board, i: 0, j }
    }
}

impl<'a> Iterator for SudokuColumnIterator<'a> {
    type Item = &'a char;
    fn next(&mut self) -> Option<Self::Item> {
        if self.i < 9 {
            let ret = &self.board[self.i][self.j];
            self.i += 1;
            Some(ret)
        } else {
            None
        }
    }
}

struct SudokuSquareIterator<'a> {
    board: &'a Vec<Vec<char>>,
    i: usize,
    j: usize,
    di: usize,
    dj: usize,
}

impl<'a> SudokuSquareIterator<'a> {
    fn new(board: &'a Vec<Vec<char>>, n: usize) -> Self {
        let di = n / 3 * 3;
        let dj = (n % 3) * 3;
        Self {
            board,
            i: 0,
            j: 0,
            di,
            dj,
        }
    }
}

impl<'a> Iterator for SudokuSquareIterator<'a> {
    type Item = &'a char;
    fn next(&mut self) -> Option<Self::Item> {
        if self.i < 3 {
            let ret = &self.board[self.i + self.di][self.j + self.dj];
            self.j = (self.j + 1) % 3;
            if self.j == 0 {
                self.i += 1;
            }
            Some(ret)
        } else {
            None
        }
    }
}

fn main() {}

#[test]
fn test_solution() {
    let board = [
        ['5', '3', '.', '.', '7', '.', '.', '.', '.'],
        ['6', '.', '.', '1', '9', '5', '.', '.', '.'],
        ['.', '9', '8', '.', '.', '.', '.', '6', '.'],
        ['8', '.', '.', '.', '6', '.', '.', '.', '3'],
        ['4', '.', '.', '8', '.', '3', '.', '.', '1'],
        ['7', '.', '.', '.', '2', '.', '.', '.', '6'],
        ['.', '6', '.', '.', '.', '.', '2', '8', '.'],
        ['.', '.', '.', '4', '1', '9', '.', '.', '5'],
        ['.', '.', '.', '.', '8', '.', '.', '7', '9'],
    ];
    assert!(Solution::is_valid_sudoku(board.map(|x| x.into()).into()),);
    let board = [
        ['8', '3', '.', '.', '7', '.', '.', '.', '.'],
        ['6', '.', '.', '1', '9', '5', '.', '.', '.'],
        ['.', '9', '8', '.', '.', '.', '.', '6', '.'],
        ['8', '.', '.', '.', '6', '.', '.', '.', '3'],
        ['4', '.', '.', '8', '.', '3', '.', '.', '1'],
        ['7', '.', '.', '.', '2', '.', '.', '.', '6'],
        ['.', '6', '.', '.', '.', '.', '2', '8', '.'],
        ['.', '.', '.', '4', '1', '9', '.', '.', '5'],
        ['.', '.', '.', '.', '8', '.', '.', '7', '9'],
    ];
    assert!(!Solution::is_valid_sudoku(board.map(|x| x.into()).into()),);
}
