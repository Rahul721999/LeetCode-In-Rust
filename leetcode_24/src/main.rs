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

mod test;
fn main() {
    let test_case = helper(vec![1, 2, 3, 4]);
    let expected_result = helper(vec![2, 1, 4, 3]);
    assert_eq!(Solution::swap_pairs(test_case.clone()), expected_result);
    assert_eq!(Solution::recursive_swap(test_case), expected_result);
}

pub struct Solution;
impl Solution {
    pub fn swap_pairs(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        if head.is_none() || head.as_ref()?.next.is_none() {
            return head;
        }
        // initialize a dummy head
        let mut dummy = ListNode::new(0);
        dummy.next = head;
        let mut prev = &mut dummy;

        //
        while let Some(mut first) = prev.next.take() {
            if let Some(mut second) = first.next.take() {
                // temp var to hold the value for the node after second one
                let third = second.next.take();

                // perform swap
                first.next = third;
                second.next = Some(first);
                prev.next = Some(second);

                // point to the node.next.next now
                prev = prev.next.as_mut().unwrap().next.as_mut().unwrap();
            } else {
                prev.next = Some(first);
                break;
            }
        }

        // return the actual head which is:
        dummy.next
    }
}

// node.next == node.next.next
// node.next.next == node
impl Solution {
    pub fn recursive_swap(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        fn swap(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
            match head {
                Some(mut first) => {
                    if let Some(mut second) = first.next.take() {
                        let third = second.next.take();
                        first.next = swap(third);
                        second.next = Some(first);
                        Some(second)
                    } else {
                        Some(first)
                    }
                }
                None => None,
            }
        }
        swap(head)
    }
}
