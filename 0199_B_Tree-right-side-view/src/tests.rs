#![allow(unused)]
use crate::{create_b_tree, Solution};


#[test]
fn test1() {
    let input = vec![Some(3), Some(9), Some(20), None, None, Some(15), Some(7)];
    let len = input.len();
    let b_tree = create_b_tree(input, 0, len);
    let expected_output = vec![3, 20, 7];

    let output = Solution::right_side_view(b_tree);
    assert_eq!(output, expected_output);
}
#[test]
fn test2() {
    let input = vec![Some(1), Some(2)];
    let len = input.len();
    let b_tree = create_b_tree(input, 0, len);
    let expected_output = vec![1, 2];

    let output = Solution::right_side_view(b_tree);
    assert_eq!(output, expected_output);
}
#[test]
fn test3() {
    let input = Vec::new();
    let len = input.len();
    let b_tree = create_b_tree(input, 0, len);
    let expected_output = Vec::new();

    let output = Solution::right_side_view(b_tree);
    assert_eq!(output, expected_output);
}