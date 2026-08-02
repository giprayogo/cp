use std::str::FromStr;

#[allow(unused)]
struct Solution;

#[allow(unused)]
/// a bit more efficient solution
impl Solution {
    pub fn eval_rpn(tokens: Vec<String>) -> i32 {
        let mut stack = Vec::new();
        for token in tokens {
            match token.as_str() {
                "+" => {
                    let rhs = stack.pop().unwrap();
                    let lhs = stack.pop().unwrap();
                    stack.push(lhs + rhs);
                },
                "-" => {
                    let rhs = stack.pop().unwrap();
                    let lhs = stack.pop().unwrap();
                    stack.push(lhs - rhs);
                },
                "*" => {
                    let rhs = stack.pop().unwrap();
                    let lhs = stack.pop().unwrap();
                    stack.push(lhs * rhs);
                },
                "/" => {
                    let rhs = stack.pop().unwrap();
                    let lhs = stack.pop().unwrap();
                    stack.push(lhs / rhs);
                },
                v => {
                    stack.push(i32::from_str(v).unwrap());
                }
            }
        }
        *stack.last().unwrap()
    }
}
// impl Solution {
//     pub fn eval_rpn(tokens: Vec<String>) -> i32 {
//         let mut stack = Vec::new();
//         for token in tokens {
//             match token.as_str() {
//                 "+" => {
//                     let rhs = stack.pop().unwrap();
//                     let lhs = stack.pop().unwrap();
//                     stack.push(lhs + rhs);
//                 },
//                 "-" => {
//                     let rhs = stack.pop().unwrap();
//                     let lhs = stack.pop().unwrap();
//                     stack.push(lhs - rhs);
//                 },
//                 "*" => {
//                     let rhs = stack.pop().unwrap();
//                     let lhs = stack.pop().unwrap();
//                     stack.push(lhs * rhs);
//                 },
//                 "/" => {
//                     let rhs = stack.pop().unwrap();
//                     let lhs = stack.pop().unwrap();
//                     stack.push(lhs / rhs);
//                 },
//                 v => {
//                     stack.push(i32::from_str(v).unwrap());
//                 }
//             }
//         }
//         *stack.last().unwrap()
//     }
// }

fn main() {}

/// I don't like the way leetcode
/// make their solution's interface
#[allow(unused)]
fn to_vs<const N: usize>(slice: [&str; N]) -> Vec<String> {
    slice.iter().map(|x| x.to_string()).collect()
}

#[test]
fn test_solution() {
    assert_eq!(Solution::eval_rpn(to_vs(["2", "1", "+", "3", "*"])), 9);
    assert_eq!(Solution::eval_rpn(to_vs(["4","13","5","/","+"])), 6);
    assert_eq!(Solution::eval_rpn(to_vs(["10","6","9","3","+","-11","*","/","*","17","+","5","+"])), 22);
}
