use std::convert::TryFrom;

fn main() {}

#[allow(unused)]
struct Solution;

// Definition for singly-linked list.
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

impl TryFrom<Vec<i32>> for ListNode {
    type Error = &'static str;

    fn try_from(value: Vec<i32>) -> Result<Self, Self::Error> {
        let mut vi = value.iter();
        let mut head = match vi.next() {
            Some(v) => ListNode::new(*v),
            None => return Err("List empty"),
        };
        let mut current = &mut head;
        for v in vi {
            let node = ListNode::new(*v);
            current.next = Some(Box::new(node));
            current = current.next.as_mut().unwrap();
        }
        Ok(head)
    }
}

impl From<ListNode> for Vec<i32> {
    fn from(mut value: ListNode) -> Self {
        let mut v = vec![value.val];
        while let Some(node) = value.next {
            v.push(node.val);
            value = *node;
        }
        v
    }
}

#[allow(unused)]
impl Solution {
    pub fn merge_two_lists(
        mut list1: Option<Box<ListNode>>,
        mut list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut head = ListNode::new(0);
        let mut cursor = &mut head;
        while let (Some(node1), Some(node2)) = (&mut list1, &mut list2) {
            if node1.val < node2.val {
                cursor.next = list1.take();
                list1 = cursor.next.as_mut().unwrap().next.take();
            } else {
                cursor.next = list2.take();
                list2 = cursor.next.as_mut().unwrap().next.take();
            }
            cursor = cursor.next.as_mut().unwrap(); 
        }
        if let Some(node) = list1 {
            cursor.next = Some(node);
        } else if let Some(node) = list2 {
            cursor.next = Some(node);
        }

        head.next
    }
}

#[test]
fn test_solution() {
    let list1: Vec<i32> = vec![1, 2, 4];
    let list1_node = ListNode::try_from(list1.clone()).unwrap();
    assert_eq!(Into::<Vec<i32>>::into(list1_node.clone()), list1);

    let list2 = vec![1, 3, 4];
    let list2_node = ListNode::try_from(list2.clone()).unwrap();
    let merged_list =
        Solution::merge_two_lists(Some(Box::new(list1_node)), Some(Box::new(list2_node))).unwrap();
    assert_eq!(Into::<Vec<i32>>::into(*merged_list), vec![1, 1, 2, 3, 4, 4]);
}
