#![allow(unused)]
use super::*;
use crate::*;
use std::cell::RefCell;
use std::rc::Rc;

#[test]
fn test1() {
    // Test case 1: Basic case with multiple nodes
    let preorder = vec![3, 9, 20, 15, 7];
    let inorder = vec![9, 3, 15, 20, 7];
    let tree = Solution::build_tree(preorder, inorder);

    // Expected tree structure as Vec<Option<i32>>
    let expected_vec = vec![Some(3), Some(9), Some(20), None, None, Some(15), Some(7)];

    let len = expected_vec.len();
    let expected_tree = create_b_tree(expected_vec, 0, len);

    assert_eq!(expected_tree, tree);
}

#[test]
fn test2() {
    // Test case 2: Single node tree
    let preorder = vec![-1];
    let inorder = vec![-1];
    let tree = Solution::build_tree(preorder, inorder);

    let expected_vec = vec![Some(-1)];
    let len = expected_vec.len();
    let expected_tree = create_b_tree(expected_vec, 0, len);

    assert_eq!(expected_tree, tree);
}

#[test]
fn test3_empty_tree() {
    // Test case 3: Empty tree
    let preorder: Vec<i32> = vec![];
    let inorder: Vec<i32> = vec![];
    let tree = Solution::build_tree(preorder, inorder);

    let expected_vec: Vec<Option<i32>> = vec![]; // Expected is an empty tree
    let len = expected_vec.len();
    let expected_tree = create_b_tree(expected_vec, 0, len);

    assert_eq!(expected_tree, tree);
}

#[test]
fn test4_deep_tree() {
    // Test case 4: Deep tree
    let preorder = vec![1, 2, 4, 5, 3];
    let inorder = vec![4, 2, 5, 1, 3];
    let tree = Solution::build_tree(preorder, inorder);

    let expected_vec = vec![Some(1), Some(2), Some(3), Some(4), Some(5)];

    let len = expected_vec.len();
    let expected_tree = create_b_tree(expected_vec, 0, len);

    assert_eq!(expected_tree, tree);
}

#[test]
fn test5_left_skewed_tree() {
    // Test case 5: Left-skewed tree (only left children)
    let preorder = vec![5, 4, 3, 2, 1];
    let inorder = vec![1, 2, 3, 4, 5];
    let tree = Solution::build_tree(preorder, inorder);

    let expected_vec = vec![
        Some(5),
        Some(4),
        None,
        Some(3),
        None,
        Some(2),
        None,
        Some(1),
    ];

    let len = expected_vec.len();
    let expected_tree = create_b_tree(expected_vec, 0, len);

    assert_eq!(expected_tree, tree);
}

#[test]
fn test6_right_skewed_tree() {
    // Test case 6: Right-skewed tree (only right children)
    let preorder = vec![1, 2, 3, 4, 5];
    let inorder = vec![1, 2, 3, 4, 5];
    let tree = Solution::build_tree(preorder, inorder);

    let expected_vec = vec![
        Some(1),
        None,
        Some(2),
        None,
        Some(3),
        None,
        Some(4),
        None,
        Some(5),
    ];

    let len = expected_vec.len();
    let expected_tree = create_b_tree(expected_vec, 0, len);

    assert_eq!(expected_tree, tree);
}
