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

        let chars = ['A', 'B'];
        for line in lines.iter().skip(1) {
            if let Ok(n) = line.parse::<usize>() {
                if n % 2 == 1 {
                    writeln!(self.writer, "NO")?;
                } else {
                    writeln!(self.writer, "YES")?;
                    for i in 0..(n / 2) {
                        let c = chars[i % 2];
                        write!(self.writer, "{}{}", c, c)?;
                    }
                    writeln!(self.writer)?;
                }
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
    let input = ["3", "6", "1", "2"].join("\n");

    let mut output = Vec::new();
    let mut solver = Solver::new(input.as_bytes(), &mut output);
    assert!(solver.solve().is_ok());
    let output_string = String::from_utf8(output).unwrap();
    let output: Vec<&str> = output_string.trim().split('\n').collect();

    assert_eq!(output, ["YES", "AABBAA", "NO", "YES", "AA"]);
}
