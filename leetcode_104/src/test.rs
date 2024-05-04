#![allow(unused)]
use crate::{create_b_tree, Solution};

// A generic function to test the invert_tree function
fn test_max_depth_of_tree(input_vec: Vec<Option<i32>>, expected_depth: i32) {
    let input_len = input_vec.len();
    let input_tree = create_b_tree(input_vec, 0, input_len);

    assert_eq!(expected_depth, Solution::max_depth(input_tree));
}

#[test]
fn test_with_empty_tree() {
    test_max_depth_of_tree(vec![], 0);
}

#[test]
fn test1() {
    let input_vec = vec![Some(2),Some(1),Some(3)];
    test_max_depth_of_tree(input_vec, 2);
}
#[test]
fn test2() {
    let input_vec = vec![Some(1), None, Some(2)];
    test_max_depth_of_tree(input_vec, 2);
}