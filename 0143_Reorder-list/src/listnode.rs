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
    let mut curr_node: &mut Box<ListNode> = &mut head;

    for i in v {
        curr_node.next = Some(Box::new(ListNode::new(i)));
        curr_node = curr_node.next.as_mut().unwrap();
    }
    head.next
}
