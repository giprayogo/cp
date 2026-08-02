use std::{collections::HashMap, error::Error, io::stdin};

#[derive(Debug)]
struct Contestant {
    // TODO: need rework because I misunderstood the scoring mechanism
    problems_solved: HashMap<i32, i32>,
    incorrect_penalty: HashMap<i32, i32>,
}

enum Status {
    C,
    I,
    R,
    U,
    E,
}

impl Status {
    fn new(s: &str) -> Self {
        use Status::*;
        match s {
            "C" => C,
            "I" => I,
            "R" => R,
            "U" => U,
            "E" => E,
            _ => panic!("Invalid status"), // I don't want to bother with proper error for now
        }
    }
}

impl Contestant {
    fn new() -> Self {
        Contestant {
            problems_solved: HashMap::new(),
            incorrect_penalty: HashMap::new(),
        }
    }

    fn add_submission(&mut self, p: i32, t: i32, l: Status) {
        use Status::*;
        match l {
            C => {
                self.problems_solved
                    .entry(p)
                    .and_modify(|e| {
                        if t < *e {
                            *e = t;
                        }
                    })
                    .or_insert(t);
                self.incorrect_penalty.entry(p).or_insert(0);
            }
            I => {
                self.incorrect_penalty
                    .entry(p)
                    .and_modify(|e| *e += 20)
                    .or_insert(20);
            }
            _ => {}
        };
    }

    fn total_problems_solved(&self) -> i32 {
        self.problems_solved.len() as i32
    }

    fn total_time(&self) -> i32 {
        let mut total = 0;
        for key in self.problems_solved.keys() {
            total += match self.problems_solved.get(key) {
                Some(&v) => {
                    v + match self.incorrect_penalty.get(key) {
                        Some(&v) => v,
                        None => 0,
                    }
                }
                None => 0,
            };
        }
        total
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut buf = String::new();
    let stdin = stdin();

    stdin.read_line(&mut buf)?;
    let n_cases = buf.trim().parse()?;
    // skip one blank line
    buf.clear();
    stdin.read_line(&mut buf)?;
    for _ in 0..n_cases {
        let mut contestants = HashMap::new();
        loop {
            buf.clear();
            stdin.read_line(&mut buf)?;
            // println!("{}", buf.trim());
            match buf.split_whitespace().collect::<Vec<_>>()[..] {
                [c, p, t, l] => {
                    let c: u8 = c.parse()?;
                    let p = p.parse()?;
                    let t = t.parse()?;
                    let l = Status::new(l);
                    if c == 49 {
                        println!("{}", buf);
                    }
                    // println!("C{}", c);
                    contestants.entry(c).or_insert(Contestant::new());
                    contestants
                        .entry(c)
                        .and_modify(|e| e.add_submission(p, t, l));
                }
                _ => break,
            }
        }

        // println!("{:?}", contestants);
        let mut vec = vec![];
        for (id, contestant) in contestants.iter() {
            vec.push((
                *id,
                contestant.total_problems_solved(),
                contestant.total_time(),
            ));
        }
        vec.sort_by_key(|a| a.0);
        vec.sort_by_key(|a| a.2);
        vec.sort_by_key(|a| -(a.1));

        for v in vec.iter() {
            println!("{} {} {}", v.0, v.1, v.2);
        }
    }
    Ok(())
}
