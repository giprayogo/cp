use std::collections::BinaryHeap;
use std::io;
use std::str::FromStr;

fn main() {
    let lines = io::stdin().lines();
    let mut sum = 0;
    let mut max_sum = 0;
    let mut heap = BinaryHeap::new();
    for line in lines {
        match i64::from_str(line.unwrap().trim()) {
            Ok(v) => sum += v,
            Err(_) => {
                println!("total calories: {sum}");
                max_sum = max_sum.max(sum);
                heap.push(sum);
                sum = 0;
            }
        }
    }
    println!("Most: {max_sum}");
    println!("Top three");
    let v = vec![
        heap.pop().unwrap(),
        heap.pop().unwrap(),
        heap.pop().unwrap(),
    ];
    println!("{:?}", v);
    println!("{}", v.iter().sum::<i64>());
}
