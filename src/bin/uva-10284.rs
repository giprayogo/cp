use core::panic;
// use std::cmp;
use std::error;
// use std::fmt::write;
use std::fmt::Display;
use std::io;
use std::ops::{AddAssign, Index, IndexMut};

use itertools::Itertools;

#[path = "../utils.rs"]
mod utils;

type Direction = (isize, isize);

#[derive(Debug, Copy, Clone)]
struct AttackBoard {
    board: [[bool; 8]; 8],
}

// TODO: should check position OOB
impl Index<Position> for AttackBoard {
    type Output = bool;
    fn index(&self, position: Position) -> &Self::Output {
        &self.board[position.row][position.column]
    }
}

// TODO: should check position OOB
impl IndexMut<Position> for AttackBoard {
    fn index_mut(&mut self, position: Position) -> &mut Self::Output {
        &mut self.board[position.row][position.column]
    }
}

impl Display for AttackBoard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut row_strings = vec![];
        for row in self.board {
            row_strings.push(row.iter().map(|x| {
                if *x == true {
                    "■"
                } else {
                    "□"
                }
            }).join(" "));
        }
        write!(f, "{}", row_strings.join("\n"))
    }
}

impl AttackBoard {
    fn new() -> Self {
        Self {
            board: [[false; 8]; 8],
        }
    }
}

// TODO: Macroable trait implementation
struct ChessBoard {
    board: [[Option<char>; 8]; 8]
}

// TODO: should check position OOB
impl Index<Position> for ChessBoard {
    type Output = Option<char>;
    fn index(&self, position: Position) -> &Self::Output {
        &self.board[position.row][position.column]
    }
}

// TODO: should check position OOB
impl IndexMut<Position> for ChessBoard {
    fn index_mut(&mut self, position: Position) -> &mut Self::Output {
        &mut self.board[position.row][position.column]
    }
}

impl Display for ChessBoard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut row_strings = vec![];
        for row in self.board {
            row_strings.push(row.iter().map(|x| {
                match *x {
                    Some(v) => v,
                    None => '.',
                }
            }).join(" "));
        }
        write!(f, "{}", row_strings.join("\n"))
    }
}

impl ChessBoard {
    fn new() -> Self {
        Self {
            board: [[None; 8]; 8],
        }
    }
}

#[derive(Debug, Copy, Clone)]
struct Position {
    row: usize,
    column: usize,
}

impl Position {
    fn new(row: usize, column: usize) -> Self {
        Position { row, column }
    }

    fn oob(&self) -> bool {
        if !((0..8).contains(&self.row) && (0..8).contains(&self.column)) {
            true
        } else {
            false
        }
    }
}

impl AddAssign<Direction> for Position {
    fn add_assign(&mut self, rhs: Direction) {
        // println!("{}, {} ({}, {})", self.row, self.column, rhs.0, rhs.1);
        *self = Self {
            row: (self.row as isize + rhs.0) as usize,
            column: (self.column as isize + rhs.1) as usize,
        }
    }
}

/// Scan continuously and update on dr and dc until obstructed or OOB
fn scan_attack(
    attack: &mut AttackBoard,
    occ: &mut ChessBoard,
    direction: (isize, isize), // (row, column)
    position: Position,
    length: Option<usize>,
) {
    // Copy as mutable locals
    let mut position = position;

    // Unlimited attack by default
    let length = match length {
        Some(v) => v,
        None => usize::MAX,
    };

    for _ in 0..length {
        position += direction;
        // println!("eval {:?}", position);
        // TODO: A bit dangerous this is
        if position.oob() || (occ[position].is_some()) {
            break;
        } else {
            attack[position] = true;
        }
    }
}

fn king_space(attack: &mut AttackBoard, occ: &mut ChessBoard, position: Position) {
    // println!("king at {}, {}", position.row, position.column);
    let attack_vectors = [
        (1, 0),
        (-1, 0),
        (0, 1),
        (0, -1),
        (1, 1),
        (1, -1),
        (-1, 1),
        (-1, -1),
    ];
    for direction in attack_vectors {
        scan_attack(attack, occ, direction, position, Some(1));
    }
}

fn queen_space(attack: &mut AttackBoard, occ: &mut ChessBoard, position: Position) {
    let attack_vectors = [
        (1, 0),
        (-1, 0),
        (0, 1),
        (0, -1),
        (1, 1),
        (1, -1),
        (-1, 1),
        (-1, -1),
    ];
    for direction in attack_vectors {
        scan_attack(attack, occ, direction, position, None);
    }
}

fn rook_space(attack: &mut AttackBoard, occ: &mut ChessBoard, position: Position) {
    let attack_vectors = [(1, 0), (-1, 0), (0, 1), (0, -1)];
    for direction in attack_vectors {
        scan_attack(attack, occ, direction, position, None);
    }
}

fn bishop_space(attack: &mut AttackBoard, occ: &mut ChessBoard, position: Position) {
    let attack_vectors = [(1, 1), (1, -1), (-1, 1), (-1, -1)];
    for direction in attack_vectors {
        scan_attack(attack, occ, direction, position, None);
    }
}

fn p_space(attack: &mut AttackBoard, occ: &mut ChessBoard, position: Position, up:bool) {
    let attack_vectors = match up {
        true => [(-1, 0), (-1, 1), (-1, -1)],
        false => [(1, 0), (1, 1), (1, -1)],
    };
    for direction in attack_vectors {
        scan_attack(attack, occ, direction, position, Some(1));
    }
}

fn horse_space(attack: &mut AttackBoard, occ: &mut ChessBoard, position: Position) {
    let attack_vectors = [
        (-1, -2),
        (-2, -1),
        (-2, 1),
        (-1, 2),
        (1, -2),
        (2, -1),
        (2, 1),
        (1, 2),
    ];
    for direction in attack_vectors {
        scan_attack(attack, occ, direction, position, Some(1));
    }
}

fn main() -> Result<(), Box<dyn error::Error>> {
    let mut buf = String::new();
    let stdin = io::stdin();

    'fens: loop {
        let fen: String = stdin!(buf, stdin);
        if fen.is_empty() {
            break 'fens;
        }
        println!("{}", fen);

        let mut occ = ChessBoard::new();
        let mut attack = AttackBoard::new();
        let mut position = Position::new(0, 0);

        // Fill the board first
        'afen: for char in fen.chars() {
            // println!("match {}", char);
            match char {
                '/' => {
                    position.row += 1;
                    position.column = 0;
                    // println!("ENTER");
                    continue;
                },
                c if c.is_numeric() => {
                    position.column += c.to_digit(10).unwrap() as usize;
                    continue;
                },
                v  => occ[position] = Some(v),
            }

            // println!("{}, {}", position.row, position.column);
            position.column += 1;
            if position.row >= 8 {
                break 'afen;
            }
        }

        // Then fill up the attack squares
        let mut position = Position::new(0, 0);
        'afen: for char in fen.chars() {
            // println!("match {}", char);
            match char {
                'k' | 'K' => king_space(&mut attack, &mut occ, position),
                'q' | 'Q' => queen_space(&mut attack, &mut occ, position),
                'b' | 'B' => bishop_space(&mut attack, &mut occ, position),
                'n' | 'N' => horse_space(&mut attack, &mut occ, position),
                'r' | 'R' => rook_space(&mut attack, &mut occ, position),
                'p' => p_space(&mut attack, &mut occ, position, false),
                'P' => p_space(&mut attack, &mut occ, position, true),
                '/' => {
                    position.row += 1;
                    position.column = 0;
                    // println!("ENTER");
                    continue;
                }
                c if c.is_numeric() => {
                    position.column += c.to_digit(10).unwrap() as usize;
                    continue;
                }
                _ => panic!("Invalid character: {}", char),
            }

            // println!("{}, {}", position.row, position.column);
            position.column += 1;
            if position.row >= 8 {
                break 'afen;
            }
        }

        println!("Occupation matrix");
        println!("{}", occ);
        println!("Attacked squares");
        println!("{}", attack);

        // Count (stupid but works)
        let mut count = 0;
        for r in 0..8 {
            for c in 0..8 {
                if (attack.board[r][c] == false) && (occ.board[r][c].is_none()) {
                    count += 1;
                }
            }
        }
        println!("{}", count);

    }

    Ok(())
}
