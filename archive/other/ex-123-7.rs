use itertools::Itertools;

fn main () {
    for i in 0..10 {
        for perm in ('A'..='J').permutations(i) {
            println!("{}", perm.iter().map(|x| x.to_string()).join(" "));
        }
    }
}