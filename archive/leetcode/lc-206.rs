fn main() {}

#[allow(unused)]
struct Solution;

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

#[allow(unused)]
impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

#[allow(unused)]
impl Solution {
    pub fn reverse_list(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        // becomes complicated because I didn't know I can mut the head!
        let mut prev = None;
        while let Some(node) = head {
            let tmp = ListNode {
                val: node.val,
                next: prev,
            };
            match node.next {
                Some(_node) => {
                    head = Some(_node);
                    prev = Some(Box::new(tmp));
                },
                None => {
                    return Some(Box::new(tmp))
                }
            }
        }
        head
    }
}

// impl From<Vec<i32>> for ListNode {
//     fn from(value: Vec<i32>) -> Self {
//         for e in value.iter().rev() {
            
//         }
//     }
// }

#[test]
fn test_solution() {
    // anyway just try directly
    // Need to be a loop??
    // assert_eq!(
    //     Solution::reverse_list(Some(vec![1, 2, 3, 4, 5].into())),
    //     [5, 4, 3, 2, 1].into()
    // );
    // assert_eq!(Solution::reverse_list([1, 2].into()), [2, 1].into());
}
