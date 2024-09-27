#![allow(unused)]
use crate::{create_b_tree, Solution};

#[test]
fn test1() {
    let input = vec![Some(3), Some(9), Some(20), None, None, Some(15), Some(7)];
    let len = input.len();
    let b_tree = create_b_tree(input, 0, len);
    let expected_output = vec![vec![3], vec![9, 20], vec![15, 7]];

    let output = Solution::level_order(b_tree);
    assert_eq!(output, expected_output);
}

#[test]
fn test2() {

    let input = vec![Some(1)];
    let len = input.len();
    let b_tree = create_b_tree(input, 0, len);
    let expected_output = vec![vec![1]];

    let output = Solution::level_order(b_tree);
    assert_eq!(output, expected_output);
}
#[test]
fn test3() {
    let input = vec![];
    let len = input.len();
    let b_tree = create_b_tree(input, 0, len);
    let expected_output: Vec<Vec<i32>> = vec![];

    let output = Solution::level_order(b_tree);
    assert_eq!(output, expected_output);
}
