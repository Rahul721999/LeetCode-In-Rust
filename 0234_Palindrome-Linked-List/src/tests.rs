#![allow(unused)]
use crate::*;

#[test]
fn test1() {
    let list = create_linked_list(vec![1, 2, 3, 4, 5]);
    assert!(!Solution::is_palindrome(list));
}

#[test]
fn test2() {
    let list = create_linked_list(vec![1, 2, 2, 1]);
    assert!(Solution::is_palindrome(list));
}

#[test]
fn test3() {
    let list = create_linked_list(vec![1]);
    assert!(Solution::is_palindrome(list));
}
#[test]
fn test4() {
    let list = create_linked_list(vec![1]);
    assert!(Solution::is_palindrome(list));
}
