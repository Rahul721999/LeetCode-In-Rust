#![allow(unused)]
use std::collections::BinaryHeap;
mod tests;
use tests::helper;
fn main() {
  Solution::merge_k_lists(vec![helper(vec![1,4,5]),helper(vec![1,3,4]),helper(vec![2,6])]);
}
  
  
  // Definition for singly-linked list.
  #[derive(PartialEq, Eq, Clone, Debug)]
  pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>
  }
  
  impl ListNode {
    #[inline]
    pub fn new(val: i32) -> Self {
      ListNode {
        next: None,
        val
      }
    }
  }
  
  use std::cmp::Reverse;
  
  struct Solution;
  impl Solution {
    pub fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>)
     -> Option<Box<ListNode>> 
    {
        let mut heap: BinaryHeap<Reverse<i32>> = BinaryHeap::new();
        
        // traverse through the vector of lists:
        for i in 0..lists.len(){
          heapify(&lists[i], &mut heap)
        }

        // insert the node's values from each list in a Heap:
        fn heapify(list: &Option<Box<ListNode>>, heap: &mut BinaryHeap<Reverse<i32>>){
          let mut head = list;
          while let Some( node) = head{
            heap.push(Reverse(node.val));
            head = &node.next;
          }
        }

        // try to pop in ascending order:
        let mut res: Box<ListNode>= Box::new(ListNode::new(-1));
        let mut curr_node = &mut res;
        while let Some(element) = heap.pop(){
          curr_node.next = Some(Box::new(ListNode::new(element.0)));
          curr_node = curr_node.next.as_mut().unwrap();
        }
        return res.next
    }
  }