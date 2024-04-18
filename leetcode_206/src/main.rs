// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}
// helper fn to create LinkedList
pub fn helper(v: Vec<i32>) -> Option<Box<ListNode>> {
    // create a head first
    let mut head = Box::new(ListNode::new(0));
    // apoint head as current node
    let mut curr_node = &mut head;

    for i in v {
        curr_node.next = Some(Box::new(ListNode::new(i)));
        curr_node = curr_node.next.as_mut().unwrap();
    }
    head.next
}

mod tests;

fn main() {
    let test_case = helper(vec![1, 2, 3, 4]);
    let expected_result = helper(vec![4, 3, 2, 1]);
    assert_eq!(Solution::reverse_list(test_case.clone()), expected_result);
    // assert_eq!(Solution::reverse_list(test_case), expected_result);
}

pub struct Solution;
impl Solution {
    pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        fn recursive_list(
            node: Option<Box<ListNode>>,
            prev: Option<Box<ListNode>>,
        ) -> Option<Box<ListNode>> {
            match node {
                Some(mut curr_node) => {
                    // println!("{}", curr_node.val);
                    // println!("{:?}", prev.clone());
                    let next_node = curr_node.next.take();
                    curr_node.next = prev;
                    recursive_list(next_node, Some(curr_node))
                }
                None => prev,
            }
        }
        recursive_list(head, None)
    }
}
