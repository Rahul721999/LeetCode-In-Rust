#![allow(unused)]
use crate::*;

#[test]
fn test1() {
    let vec = vec![
        Some(5),
        Some(4),
        Some(8),
        Some(11),
        None,
        Some(13),
        Some(4),
        Some(7),
        Some(2),
        None,
        None,
        None,
        Some(1),
    ];
    let len = vec.len();
    let tree = create_b_tree(vec, 0, len);
    let output = Solution::has_path_sum(tree, 22);
    assert!(output);
}

#[test]
fn test2() {
    let vec = vec![Some(1), Some(2), Some(3)];
    let len = vec.len();
    let tree = create_b_tree(vec, 0, len);
    let output = Solution::has_path_sum(tree, 5);
    assert!(!output);
}

#[test]
fn test3() {
    let vec = vec![];
    let len = vec.len();
    let tree = create_b_tree(vec, 0, len);
    let output = Solution::has_path_sum(tree, 0);
    assert!(!output);
}
#[test]
fn test4() {
    let vec = vec![Some(1), Some(2)];
    let len = vec.len();
    let tree = create_b_tree(vec, 0, len);
    let output = Solution::has_path_sum(tree, 1);
    assert!(!output);
}
