#![allow(unused)]
#[cfg(test)]
use crate::Solution;
use crate::ListNode;


// helper fn to create LinkedList
pub fn helper(v : Vec<i32>) ->  Option<Box<ListNode>>{
     // create a head first
     let mut head = Box::new(ListNode::new(0));
     // apoint head as current node
     let mut curr_node: &mut Box<ListNode> = &mut head;
   
     for i in v{
         curr_node.next = Some(Box::new(ListNode::new(i)));
         curr_node = curr_node.next.as_mut().unwrap();
     }
     head.next
   
}


#[test]
fn test(){
     assert_eq!(Solution::merge_k_lists(vec![helper(vec![1,4,5]),helper(vec![1,3,4]),helper(vec![2,6])]), helper(vec![1,1,2,3,4,4,5,6]));
}

/// Test merging two sorted linked lists.
#[test]
fn test_merge_two_lists() {
    let list1 = helper(vec![1, 4, 5]);
    let list2 = helper(vec![1, 3, 4]);

    assert_eq!(
        Solution::merge_k_lists(vec![list1.clone(), list2.clone()]),
        helper(vec![1, 1, 3, 4, 4, 5])
    );
}

/// Test merging three sorted linked lists.
#[test]
fn test_merge_three_lists() {
    let list1 = helper(vec![1, 4, 5]);
    let list2 = helper(vec![1, 3, 4]);
    let list3 = helper(vec![2, 6]);

    assert_eq!(
        Solution::merge_k_lists(vec![list1.clone(), list2.clone(), list3.clone()]),
        helper(vec![1, 1, 2, 3, 4, 4, 5, 6])
    );
}

/// Test merging with an empty linked list.
#[test]
fn test_merge_with_empty_list() {
    let list1 = helper(vec![1, 4, 5]);
    let list2 = helper(vec![1, 3, 4]);

    assert_eq!(
        Solution::merge_k_lists(vec![list1.clone(), list2.clone(), None]),
        helper(vec![1, 1, 3, 4, 4, 5])
    );
}

/// Test merging with an empty vector of linked lists.
#[test]
fn test_merge_with_empty_vector() {
    assert_eq!(
        Solution::merge_k_lists(Vec::new()),
        None
    );
}