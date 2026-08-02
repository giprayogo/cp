use std::io;

fn main() -> Result<(), Box<dyn std::error::Error>>{
    // TODO: this pattern is macroable?
    let mut buffer = String::new();
    let mut output_buffer = vec![];
    let stdin = io::stdin();
    stdin.read_line(&mut buffer)?;

    let n_cases = buffer.trim().parse().expect("invalid n cases");
    for i in 0..n_cases {
        buffer.clear();
        stdin.read_line(&mut buffer)?;
        let speed: u32 = buffer.split_whitespace()
            .skip(1)
            .map(|x| x.parse().unwrap())
            .max()
            .unwrap_or(0);
        output_buffer.push(format!("Case {}: {}", i+1, speed));
    }

    for output_line in output_buffer {
        println!("{}", output_line);
    }

    Ok(())
}