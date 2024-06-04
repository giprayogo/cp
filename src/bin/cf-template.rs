use std::{
    error,
    io::{self, BufRead, Write},
};

type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

struct Solver<R, W> {
    reader: R,
    writer: W,
    buffer: String,
}

impl<R, W> Solver<R, W>
where
    R: BufRead,
    W: Write,
{
    fn new(reader: R, writer: W) -> Self {
        Solver {
            reader,
            writer,
            buffer: String::new(),
        }
    }
    /// C++ cin or Python input -like API that takes one line from stdin.
    fn input(&mut self) -> Option<String> {
        match self.reader.read_line(&mut self.buffer) {
            Ok(v) => {
                // Check documentation here: https://doc.rust-lang.org/std/io/trait.BufRead.html#method.read_line
                if v == 0 {
                    None
                } else {
                    let line = self.buffer.trim().to_string();
                    self.buffer.clear();
                    Some(line)
                }
            }
            Err(e) => panic!("Unexpected BufRead error: {e}"),
        }
    }

    // * Implement here *
    fn solve(&mut self) -> io::Result<()> {
        while let Some(line) = self.input() {
            if let Ok(v) = line.parse::<i32>() {
                write!(self.writer, "{}", v)?; // TODO: Another helper for this
            }
        }
        Ok(())
    }
}

// * Don't touch *
fn main() -> Result<()> {
    let input = io::stdin().lock();
    let output = io::stdout();
    let mut solver = Solver::new(input, output);
    solver.solve()?;
    Ok(())
}

// * Copy from here up *
//
#[test]
fn test() {
    let input = ["THIS", "IS", "a", "generic", "555"].join("\n");
    assert_eq!(test_helper(&input), ["555"]);
}

#[allow(unused)]
fn test_helper(input: &str) -> Vec<String> {
    let mut output = Vec::new();
    let mut solver = Solver::new(input.as_bytes(), &mut output);
    assert!(solver.solve().is_ok());
    let output_string = String::from_utf8(output).unwrap();
    let output: Vec<String> = output_string
        .trim()
        .split('\n')
        .map(str::to_string)
        .collect();
    output
}

/*
Thanks to:

- https://stackoverflow.com/a/28370712,
- https://users.rust-lang.org/t/how-to-make-a-unit-test-in-rust-for-a-function-that-uses-console-input-io-stdin/89204/2,

and others!
Written here for easier copy+paste.
*/
