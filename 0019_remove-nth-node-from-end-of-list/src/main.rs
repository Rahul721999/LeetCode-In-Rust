fn main() {
    assert_eq!(
        Solution::remove_nth_from_end(helper(vec![1,2,3,4,5]), 2),
        helper(vec![1, 2, 3, 5])
    );
}

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

// helper fn to create LinkedList
fn helper(v: Vec<i32>) -> Option<Box<ListNode>> {
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

struct Solution;
impl Solution {
    pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
    // two pointer approach..
        // create a dummy head node
        let mut dummy_head = Box::new(ListNode{val: 0, next: head});
        let mut fast = dummy_head.clone();

        for _ in 0..n{
            fast = fast.next.unwrap();
        }
        
        let mut slow = dummy_head.as_mut();
        while fast.next.is_some(){
            fast = fast.next.unwrap();
            slow = slow.next.as_mut().unwrap();
        }

        slow.next = slow.next.as_ref().unwrap().next.clone();
        dummy_head.next
    }
}
