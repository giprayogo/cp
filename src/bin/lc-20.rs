#[allow(unused)]
struct Solution;
enum BT {
    P,
    S,
    C,
}

#[allow(unused)]
impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut stack = vec![];
        for c in s.chars() {
            match c {
                '(' => stack.push(BT::P),
                '[' => stack.push(BT::S),
                '{' => stack.push(BT::C),
                ')' => match stack.pop() {
                    Some(BT::P) => {},
                    _ => return false, 
                }
                ']' => match stack.pop() {
                    Some(BT::S) => {},
                    _ => return false, 
                }
                '}' => match stack.pop() {
                    Some(BT::C) => {},
                    _ => return false, 
                }
                _ => unreachable!(),
            }
        }
        stack.is_empty()
    }
    // Wrong heuristic (too eager to -not- use stack)
    // pub fn is_valid(s: String) -> bool {
    //     let mut pc = 0;
    //     let mut sc = 0;
    //     let mut cc = 0;
    //     let mut bt = None;
    //     for c in s.chars() {
    //         match c {
    //             '(' => {
    //                 pc += 1;
    //                 bt = Some(BT::P)
    //             }
    //             ')' => match bt {
    //                 Some(BT::P) => pc -= 1,
    //                 _ => return false,
    //             },
    //             '[' => {
    //                 sc += 1;
    //                 bt = Some(BT::S)
    //             }
    //             ']' => match bt {
    //                 Some(BT::S) => sc -= 1,
    //                 _ => return false,
    //             },
    //             '{' => {
    //                 cc += 1;
    //                 bt = Some(BT::C)
    //             }
    //             '}' => match bt {
    //                 Some(BT::C) => cc -= 1,
    //                 _ => return false,
    //             },
    //             _ => unreachable!(),
    //         }
    //         if pc < 0 || sc < 0 || cc < 0 {
    //             return false;
    //         }
    //     }
    //     true
    // }
}

fn main() {}

#[test]
fn test_solution() {
    assert!(Solution::is_valid("()".into()));
    assert!(Solution::is_valid("()[]{}".into()));
    assert!(!Solution::is_valid("(]".into()));
    assert!(!Solution::is_valid("([)]".into()));
    assert!(Solution::is_valid("{[]}".into()));
    assert!(!Solution::is_valid("[".into()));
}
