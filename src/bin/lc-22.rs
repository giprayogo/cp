#[allow(unused)]
use std::{collections::HashSet, vec};

fn main() {}

#[allow(unused)]
struct Solution {}

#[allow(unused)]
#[derive(Clone)]
struct Frame {
    string: String,
    open: i32,
    n: i32,
}

#[allow(unused)]
impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        let mut frame_stack = vec![Frame {
            string: "".to_string(),
            open: 0,
            n,
        }];
        let mut result_stack = vec![];
        while let Some(mut frame) = frame_stack.pop() {
            if frame.n > 0 {
                let mut _frame = frame.clone();
                _frame.n -= 1;
                _frame.string += "(";
                _frame.open += 1;
                frame_stack.push(_frame);
                if frame.open > 0 {
                    let mut _frame = frame.clone();
                    _frame.string += ")";
                    _frame.open -= 1;
                    frame_stack.push(_frame);
                }
                let mut _frame = frame.clone();
            } else {
                while frame.open > 0 {
                    frame.string += ")";
                    frame.open -= 1;
                }
                result_stack.push(frame);
            }
        }
        result_stack.into_iter().map(|x| x.string).collect()
    }
}

#[test]
fn test_solution() {
    assert_eq!(
        Solution::generate_parenthesis(3)
            .into_iter()
            .collect::<HashSet<_>>(),
        vec!["((()))", "(()())", "(())()", "()(())", "()()()"]
            .iter()
            .map(|x| x.to_string())
            .collect::<HashSet<_>>()
    );
    assert_eq!(Solution::generate_parenthesis(1), vec!["()"]);
}
