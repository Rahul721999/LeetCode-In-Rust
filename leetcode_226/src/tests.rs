#![allow(unused)]
use crate::{Solution, create_b_tree};

// A generic function to test the invert_tree function
fn test_invert_tree(input_vec: Vec<i32>, expected_output_vec: Vec<i32>) {
    let input_len = input_vec.len();
    let input_tree = create_b_tree(input_vec.clone(), 0, input_len);

    let expected_output_len = expected_output_vec.len();
    let expected_output_tree = create_b_tree(expected_output_vec, 0, expected_output_len);

    assert_eq!(expected_output_tree, Solution::invert_tree(input_tree));
}

#[test]
fn test_with_empty_tree() {
    test_invert_tree(vec![], vec![]);
}

#[test]
fn test1() {
    let input_vec = vec![2,1,3];
    let expected_output_vec = vec![2,3,1];
    test_invert_tree(input_vec, expected_output_vec);
}
#[test]
fn test2() {
    let input_vec = vec![4, 2, 7, 1, 3, 6, 9];
    let expected_output_vec = vec![4, 7, 2, 9, 6, 3, 1];
    test_invert_tree(input_vec, expected_output_vec);
}