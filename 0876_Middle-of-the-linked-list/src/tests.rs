#![allow(unused)]
use crate::{ListNode, helper, Solution};

#[test]
fn test1(){
    let input = helper(vec![1, 2, 3, 4]);
    assert_eq!(Solution::middle_node(input).unwrap().val, 3);
}
#[test]
fn test2(){
    let input = helper(vec![1, 2, 3, 4, 5]);
    assert_eq!(Solution::middle_node(input).unwrap().val, 3);
}
#[test]
fn test3(){
    let input = helper(vec![1]);
    assert_eq!(Solution::middle_node(input).unwrap().val, 1);
}
#[test]
fn test4(){
    let input = helper(vec![1, 2]);
    assert_eq!(Solution::middle_node(input).unwrap().val, 2);
}