mod tests;
use helper::linked_list::*;
fn main() {
    let list = create_linked_list(vec![1, 2, 3, 4, 5]);
    assert!(!Solution::is_palindrome(list));

    let list = create_linked_list(vec![1, 2, 2, 1]);
    assert!(Solution::is_palindrome(list));
}

pub struct Solution;
impl Solution {
    pub fn is_palindrome(head: Option<Box<ListNode>>) -> bool {
        
        // 1. find mid node
        let mut fast = &head;
        let mut slow = &head;
        while fast.is_some() && fast.as_ref().unwrap().next.is_some() {
            fast = &fast.as_ref().unwrap().next.as_ref().unwrap().next;
            slow = &slow.as_ref().unwrap().next;
        }

        // 2. reverse the second list
        let second_list = Self::reverse_list(None, slow.clone());

        // 3. traverse and compare each node.
        let mut first = &head;
        let mut second = &second_list;
        while let Some(sec_node) = &second {
            if let Some(first_node) = &first {
                if sec_node.val != first_node.val {
                    return false;
                }
                second = &sec_node.next;
                first = &first_node.next;
            } else {
                return false;
            }
        }
        true
    }

    fn reverse_list(
        prev: Option<Box<ListNode>>,
        mut curr: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        if let Some(curr_node) = curr.as_mut() {
            let next = curr_node.next.take();
            curr_node.next = prev;
            return Self::reverse_list(curr, next);
        }
        return prev;
    }
}
