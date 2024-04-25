use std::{
    error,
    io::{self, BufRead, Write},
};

type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

struct Solver<R, W> {
    reader: R,
    writer: W,
}

impl<R, W> Solver<R, W>
where
    R: BufRead,
    W: Write,
{
    fn new(reader: R, writer: W) -> Self {
        Solver { reader, writer }
    }

    fn solve(&mut self) -> io::Result<()> {
        let mut buffer = String::new();
        let mut lines = Vec::new();
        while let Ok(v) = self.reader.read_line(&mut buffer) {
            if v == 0 {
                break;
            }
            lines.push(buffer.trim().to_string());
            buffer.clear();
        }

        for line in lines {
            if let Ok(v) = line.parse::<i32>() {
                write!(self.writer, "{}", v)?;
                self.writer.flush()?;
            }
        }
        Ok(())
    }
}

fn main() -> Result<()> {
    let input = io::stdin().lock();
    let output = io::stdout();

    let mut solver = Solver::new(input, output);
    solver.solve()?;
    Ok(())
}

#[test]
fn test() {
    let input = ["THIS", "IS", "a", "generic", "555"].join("\n");

    let mut output = Vec::new();
    let mut solver = Solver::new(input.as_bytes(), &mut output);
    assert!(solver.solve().is_ok());
    let output_string = String::from_utf8(output).unwrap();
    let output: Vec<&str> = output_string.trim().split('\n').collect();

    assert_eq!(output, ["555"]);
}

/*
Thanks to:

- https://stackoverflow.com/a/28370712,
- https://users.rust-lang.org/t/how-to-make-a-unit-test-in-rust-for-a-function-that-uses-console-input-io-stdin/89204/2,

and others!
Written here for easier copy+paste.
*/