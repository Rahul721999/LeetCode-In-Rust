#![allow(unused)]
mod tests;
mod listnode;
use listnode::{helper, ListNode};

fn main() {
    let mut input = helper(vec![1, 2, 3, 4]);
    let expected_output = helper(vec![1, 4, 2, 3]);

    println!("INPUT: {:?}", input);

    // Reorder list
    Solution::reorder_list(&mut input);
    println!("OUTPUT: {:?}", input);

    // Test
    assert_eq!(expected_output, input);
}

pub struct Solution;

impl Solution {
    /* Three steps to solve the problem:
        Given a list: l1 -> l2 -> l3 -> ... -> ln-2 -> ln-1 -> ln

        1. Split the list in half: (l1 -> l2 -> l3) and (ln-2 -> ln-1 -> ln)
        2. Reverse the second half: (ln -> ln-1 -> ln-2 -> None)
        3. Merge the two halves in alternating order: (l1 -> ln -> l2 -> ln-1 -> l3 -> ln-2)
    */

    pub fn reorder_list(head: &mut Option<Box<ListNode>>) {
        if head.is_none() || head.as_deref().unwrap().next.is_none() {
            return;
        }

        // 1. Split the list into two halves
        let mut second_list = Solution::mid_node(head);

        // 2. Reverse the second half
        second_list = Self::reverse_list(second_list);

        // 3. Merge the two halves
        Solution::merge_lists(head, second_list);
    }

    // Splits the list into two halves and returns the head of the second half
    pub fn mid_node(head: &mut Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        // Handle case where the list has fewer than 2 nodes
        if head.is_none() || head.as_ref().unwrap().next.is_none() {
            return None;
        }

        // First pass: Count the total number of nodes
        let mut count = 0;
        let mut temp = head.as_ref();
        while let Some(node) = temp {
            count += 1;
            temp = node.next.as_ref();
        }

        // Second pass: Traverse to the node before the middle
        let prev_of_mid = (count / 2) - 1;
        let mut temp = head.as_mut();
        for _ in 0..prev_of_mid {
            if let Some(node) = temp {
                temp = node.next.as_mut();
            }
        }

        // Split the list into two halves
        let res = temp.as_mut()?.next.take(); // Get the second half
        temp.as_mut()?.next = None; // Terminate the first half
        res
    }

    // Reverses a linked list
    pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        fn recursive_fn(
            curr: Option<Box<ListNode>>,
            prev: Option<Box<ListNode>>,
        ) -> Option<Box<ListNode>> {
            match curr {
                Some(mut curr_node) => {
                    let next_node = curr_node.next.take();
                    curr_node.next = prev;
                    recursive_fn(next_node, Some(curr_node))
                }
                None => prev,
            }
        }
        recursive_fn(head, None)
    }

    // Merges two linked lists: first and list2 in alternating order
    pub fn merge_lists(mut head: &mut Option<Box<ListNode>>, mut list2: Option<Box<ListNode>>) {
        let mut p1 = head.take(); // Take the first list
        let mut p2 = list2; // Second list is taken as is

        while p1.is_some() && p2.is_some() {
            let mut temp1 = p1.as_mut().unwrap().next.take(); // Store the next node of p1
            let mut temp2 = p2.as_mut().unwrap().next.take(); // Store the next node of p2

            *head = p1.take(); // Add p1 node to the merged list
            head = &mut head.as_mut().unwrap().next; // Move to the next in the merged list
            *head = p2.take(); // Add p2 node to the merged list
            head = &mut head.as_mut().unwrap().next; // Move to the next in the merged list

            // Update p1 and p2 for the next iteration
            p1 = temp1;
            p2 = temp2;
        }

        // If any nodes remain in p1 or p2, append them to the merged list
        if p1.is_some() {
            *head = p1;
        } else if p2.is_some() {
            *head = p2;
        }
    }
}
