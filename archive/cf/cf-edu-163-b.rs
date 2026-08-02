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

        'a: for line in lines.iter().skip(2).step_by(2) {
            let array: Vec<i8> = line
                .split_ascii_whitespace()
                .map(|x| {
                    x.parse()
                        .unwrap_or_else(|_| panic!("can't parse {} as int", x))
                })
                .collect();
            let mut last = i8::MAX;
            for num in array.into_iter().rev() {
                if num > last {
                    let (l, r) = (num / 10, num % 10);
                    if l <= r && r <= last {
                        last = l;
                    } else {
                        writeln!(self.writer, "NO")?;
                        continue 'a;
                    }
                } else {
                    last = num;
                }
            }
            writeln!(self.writer, "YES")?;
            self.writer.flush()?;
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
    let input = ["3", "4", "12 3 45 67", "3", "12 28 5", "2", "0 0"].join("\n");

    let mut output = Vec::new();
    let mut solver = Solver::new(input.as_bytes(), &mut output);
    assert!(solver.solve().is_ok());
    let output_string = String::from_utf8(output).unwrap();
    let output: Vec<&str> = output_string.trim().split('\n').collect();

    assert_eq!(output, ["YES", "NO", "YES"]);
}
