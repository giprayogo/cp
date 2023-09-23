use std::error;
use std::fmt::Display;
use std::io;

use itertools::Itertools;

#[derive(Debug, PartialEq)]
struct Square(Vec<Vec<char>>);

impl Square {
    fn new(n: usize) -> Self {
        Self(vec![vec![char::default(); n]; n])
    }

    fn fill_square(&mut self, buf: &mut String, stdin: &io::Stdin) -> Result<(), io::Error> {
        for row in self.0.iter_mut() {
            buf.clear();
            stdin.read_line(buf)?;
            for (c, col) in buf.trim().chars().zip(row) {
                *col = c;
            }
        }
        Ok(())
    }

    /// not very efficient but anyway
    fn view(&self, anchor: (usize, usize), n: usize) -> Option<Square> {
        if n > self.0.len() || anchor.0 + n > self.0.len() || anchor.1 + n > self.0.len() {
            return None;
        }

        let mut vec = vec![];
        for row in &self.0[anchor.0..anchor.0 + n] {
            let col_view = row[anchor.1..anchor.1 + n].to_owned();
            vec.push(col_view);
        }
        Some(Square(vec))
    }

    fn includes_count(&self, other: &Square) -> usize {
        let n = self.0.len();
        let m = other.0.len();
        let mut count = 0;
        if m > n {
            return count;
        }

        // find anchor
        for i in 0..(n - m + 1) {
            for j in 0..(n - m + 1) {
                // match
                let view = match self.view((i, j), m) {
                    Some(v) => v,
                    None => panic!("Invalid anchor {:?}/{} for square of size {}", (i, j), m, n),
                };
                if view == *other {
                    count += 1;
                }
            }
        }
        count
    }

    fn rotate_90(&self) -> Square {
        let n = self.0.len();
        let mut square = Square::new(n);
        for i in 0..n {
            for j in 0..n {
                square.0[i][j] = self.0[n - j - 1][i];
            }
        }
        square
    }

    fn rotate_180(&self) -> Square {
        Square(
            self.0
                .iter()
                .rev()
                .map(|row| row.iter().rev().map(|c| *c).collect_vec())
                .collect_vec(),
        )
    }

    fn rotate_270(&self) -> Square {
        let n = self.0.len();
        let mut square = Square::new(n);
        for i in 0..n {
            for j in 0..n {
                square.0[i][j] = self.0[j][n - i - 1];
            }
        }
        square
    }
}

impl Display for Square {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.iter().map(|x| x.iter().join("")).join("\n"))
    }
}

fn main() -> Result<(), Box<dyn error::Error>> {
    let mut buf = String::new();
    let stdin = io::stdin();
    let mut square_pairs = vec![];

    loop {
        buf.clear();
        stdin.read_line(&mut buf)?;
        let (n1, n2) = match buf.trim().split_whitespace().collect_tuple() {
            Some(("0", "0")) => break,
            Some((n1, n2)) => {
                let n1 = n1.parse()?;
                let n2 = n2.parse()?;
                (n1, n2)
            }
            None => panic!("Invalid Nn input: {}", buf),
        };

        let mut square_1 = Square::new(n1);
        let mut square_2 = Square::new(n2);
        square_1.fill_square(&mut buf, &stdin)?;
        square_2.fill_square(&mut buf, &stdin)?;
        square_pairs.push((square_1, square_2));
    }

    for (square_1, square_2) in square_pairs {
        println!(
            "{} {} {} {}",
            square_1.includes_count(&square_2),
            square_1.includes_count(&square_2.rotate_90()),
            square_1.includes_count(&square_2.rotate_180()),
            square_1.includes_count(&square_2.rotate_270()),
        );
    }
    Ok(())
}
