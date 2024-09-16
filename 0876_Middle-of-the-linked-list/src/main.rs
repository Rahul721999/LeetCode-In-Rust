mod listnode;
use listnode::*;
fn main() {
    let input = helper(vec![1, 2, 3, 4]);
    assert_eq!(Solution::middle_node(input).unwrap().val, 3);
}

pub struct Solution;
impl Solution {
    pub fn middle_node(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        if head.is_none() || head.as_ref().unwrap().next.is_none() {
            return head;
        }
        let mut slow = head.as_ref();
        let mut fast = head.as_ref();

        while fast.is_some(){
            fast = fast.as_ref()?.next.as_ref();
            if fast.is_none() {break}
            fast = fast.as_ref()?.next.as_ref();
            slow = slow.as_ref()?.next.as_ref();
        }
        Some(slow.unwrap().clone())
    }
}
