#![allow(unused)]
use crate::{create_b_tree, find_node, Solution};
use std::cell::RefCell;
use std::rc::Rc;

#[test]
fn test1() {
    let tree = vec![
        Some(6),
        Some(2),
        Some(8),
        Some(0),
        Some(4),
        Some(7),
        Some(9),
        None,
        None,
        Some(3),
        Some(5),
    ];
    let len = tree.len();
    let root = create_b_tree(tree, 0, len);
    let p = find_node(root.clone(), 2);
    let q = find_node(root.clone(), 8);

    let output = Solution::lowest_common_ancestor(root.clone(), p, q);

    assert_eq!(output, root);
}
#[test]
fn test2() {
    let tree = vec![
        Some(6),
        Some(2),
        Some(8),
        Some(0),
        Some(4),
        Some(7),
        Some(9),
        None,
        None,
        Some(3),
        Some(5),
    ];
    let len = tree.len();
    let root = create_b_tree(tree, 0, len);
    let p = find_node(root.clone(), 2);
    let q = find_node(root.clone(), 4);

    let output = Solution::lowest_common_ancestor(root.clone(), p, q);

    let expected_output = find_node(root.clone(), 2);
    assert_eq!(output, expected_output);
}
#[test]
fn test3() {
    let tree = vec![Some(2), Some(1)];
    let len = tree.len();
    let root = create_b_tree(tree, 0, len);
    let p = find_node(root.clone(), 2);
    let q = find_node(root.clone(), 1);

    let output = Solution::lowest_common_ancestor(root.clone(), p, q);

    let expected_output = find_node(root.clone(), 2);
    assert_eq!(output, expected_output);
}
