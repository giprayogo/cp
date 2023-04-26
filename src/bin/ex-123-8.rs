use itertools::Itertools;

fn main() {
    for sub in (0..20).powerset() {
        println!("{}", sub.iter().map(|x| x.to_string()).join(" ") );
    }
}