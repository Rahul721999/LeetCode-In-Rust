#![allow(unused)]
use crate::*;

#[test]
fn test1() {
    let input = vec![Some(2), Some(1), Some(3)];
    let len = input.len();
    let tree = create_b_tree(input, 0, len);
    let output = Solution::is_valid_bst(tree);
    assert!(output);
}
#[test]
fn test2() {
    let input = vec![Some(0), Some(-1)];
    let len = input.len();
    let tree = create_b_tree(input, 0, len);
    let output = Solution::is_valid_bst(tree);
    assert!(output);
}
#[test]
fn test3() {
    let input = vec![Some(5), Some(1), Some(4), None, None, Some(3), Some(6)];
    let len = input.len();
    let tree = create_b_tree(input, 0, len);
    let output = Solution::is_valid_bst(tree);
    assert!(!output);
}
