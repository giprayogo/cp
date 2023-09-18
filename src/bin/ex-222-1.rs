use std::error;
use std::io;

fn main() -> Result<(), Box<dyn error::Error>> {
    let mut buf = String::new();
    let stdin = io::stdin();

    loop {
        buf.clear();
        stdin.read_line(&mut buf)?;
        let s = match buf.trim().parse() {
            Ok(v) => v,
            Err(_) => break,
        };

        println!("S = {}", s);
        let mut n = 1;
        while s >= n {
            println!("Modulo of {} by {} is {}", s, n, bitwise_modulo(s, n));
            n <<= 1;
        }
        println!("Is {} power of two? {}", s, is_power_of_two(s));
        println!(
            "Turn off last bit of {}: {:#b} -> {:#b}",
            s,
            s,
            turn_off_last(s)
        );
        println!(
            "Turn on last zero of {}: {:#b} -> {:#b}",
            s,
            s,
            turn_on_last(s)
        );
        println!(
            "Turn off last consecutive ones of {}: {:#b} -> {:#b}",
            s,
            s,
            turn_off_last_consecutive(s)
        );
        println!(
            "Turn on last consecutive zeros of {}: {:#b} -> {:#b}",
            s,
            s,
            turn_on_last_consecutive(s)
        );
    }

    Ok(())
}

/// Using bitwise operation to calculate modulo of something
/// divided by power of two
fn bitwise_modulo(s: i32, n: i32) -> i32 {
    s & !(n | -n) // Two's complement is considered bitwise in the book
}

/// Determine if S is power of 2 only by bitwise operation
fn is_power_of_two(s: i32) -> bool {
    match !(s & -s) & s {
        0 => true,
        _ => false,
    }
}

/// Turn off last bit in s
fn turn_off_last(s: i32) -> i32 {
    !(s & -s) & s
}

/// Turn on last zero in s
fn turn_on_last(s: i32) -> i32 {
    (!s & -!s) | s
}

// Turn off last consecutive run of ones in s
fn turn_off_last_consecutive(s: i32) -> i32 {
    let k = s | !(s | -s);
    s & (!k | -!k)
}

// Turn on last consecutive run of zeros in s
fn turn_on_last_consecutive(s: i32) -> i32 {
    !turn_off_last_consecutive(!s)
}