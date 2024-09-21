mod tests;
mod listnode;
use listnode::*;
fn main() {
    let input = helper(vec![1, 2, 3, 4, 5]);
    let expected_output = helper(vec![4,5,1,2,3]);
    let output = Solution::rotate_right(input, 2);
    assert_eq!(output, expected_output);
}

pub struct Solution;
impl Solution {
    pub fn rotate_right(mut head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        if head.is_none() {
            return None;
        }

        // 1. Calculate the length of the list
        let mut len = 0;
        let mut current = head.as_ref(); // Immutable borrow for length calculation
        while let Some(node) = current {
            len += 1;
            current = node.next.as_ref();
        }

        // 2. position to split the list
        let k = k % len;
        if k == 0 {
            return head;
        }

        let split_pos = len - k;
        let mut temp_head = &mut head;
        
        // Traverse the list to the node before the split point
        for _ in 0..split_pos - 1 {
            if let Some(node) = temp_head {
                temp_head = &mut node.next;
            }
        }

        // `temp_head` now points to the node before the split
        let mut new_head = temp_head.as_mut()?.next.take();
        let mut tail = &mut new_head;

        // traverse to the end of the second list...
        while let Some(node) = tail.as_mut(){
            if node.next.is_none(){
                node.next = head;
                break;
            };
            tail = &mut node.next;
        }
        return new_head
    }
}
