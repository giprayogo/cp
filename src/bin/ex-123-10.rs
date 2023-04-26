use regex::Regex;
use std::io;

fn main() {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();

    // let re = Regex::new(r"(\s|^)([a-z][0-9]{2})(\n|\s)").unwrap();
    let re = Regex::new(r"[a-z][0-9]{2}").unwrap();

    let before = buf.trim();

    let mut words: Vec<&str> = Vec::new();
    for word in buf.trim().split_whitespace() {
        if word.len() == 3 && re.is_match(&word) {
            words.push("***");
        } else {
            words.push(word);
        }
    }
    let after = words.join(" ");
    println!("{}", before);
    println!("{}", after);
}
