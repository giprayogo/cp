use std::error;
use std::fmt;
use std::io;

#[path = "../utils.rs"]
mod utils;

#[derive(Debug, Clone, Copy)]
enum Wire {
    X,
    MinX,
    Y,
    MinY,
    Z,
    MinZ,
}

#[derive(Debug, Clone, Copy)]
enum Bend {
    PlusZ,
    MinZ,
    PlusY,
    MinY,
    No,
}

impl Wire {
    fn bend(&mut self, b: Bend) {
        use Wire::*;

        match self {
            X => match b {
                Bend::PlusY => *self = Y,
                Bend::MinY => *self = MinY,
                Bend::PlusZ => *self = Z,
                Bend::MinZ => *self = MinZ,
                _ => {}
            },
            MinX => match b {
                Bend::PlusY => *self = MinY,
                Bend::MinY => *self = Y,
                Bend::PlusZ => *self = MinZ,
                Bend::MinZ => *self = Z,
                _ => {}
            },
            Y => match b {
                Bend::PlusY => *self = MinX,
                Bend::MinY => *self = X,
                _ => {}
            },
            MinY => match b {
                Bend::PlusY => *self = X,
                Bend::MinY => *self = MinX,
                _ => {}
            },
            Z => match b {
                Bend::PlusZ => *self = MinX,
                Bend::MinZ => *self = X,
                _ => {}
            },
            MinZ => match b {
                Bend::PlusZ => *self = X,
                Bend::MinZ => *self = MinX,
                _ => {}
            },
        }
    }
}

impl fmt::Display for Wire {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Wire::*;
        let s = match *self {
            X => "+x",
            MinX => "-x",
            Y => "+y",
            MinY => "-y",
            Z => "+z",
            MinZ => "-z",
        };
        write!(f, "{}", s)
    }
}

fn main() -> Result<(), Box<dyn error::Error>> {
    let mut buf = String::new();
    let stdin = io::stdin();

    let mut output_buffer = Vec::new();
    loop {
        let length: u32 = stdin!(buf, stdin);
        if length == 0 {
            break;
        }

        let mut wire = Wire::X;
        for instruction in stdins!(buf, stdin) {
            let bend = match instruction {
                "+z" => Bend::PlusZ,
                "-z" => Bend::MinZ,
                "+y" => Bend::PlusY,
                "-y" => Bend::MinY,
                "No" => Bend::No,
                _ => panic!("Invalid bend instruction: {}", instruction),
            };
            wire.bend(bend);
        }
        output_buffer.push(format!("{}", wire));
    }
    for text in output_buffer {
        println!("{}", text);
    }

    Ok(())
}
