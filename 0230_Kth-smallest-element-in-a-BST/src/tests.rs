#![allow(unused)]
use crate::*;

#[test]
fn test1() {
    let input = vec![Some(3), Some(1), Some(4), None, Some(2)];
    let len = input.len();
    let b_tree = create_b_tree(input, 0, len);

    let output = Solution::kth_smallest(b_tree, 1);
    assert_eq!(output, 1);
}
#[test]
fn test2() {
    let input = vec![
        Some(5),
        Some(3),
        Some(6),
        Some(2),
        Some(4),
        None,
        None,
        Some(1),
    ];
    let len = input.len();
    let b_tree = create_b_tree(input, 0, len);

    let output = Solution::kth_smallest(b_tree, 3);
    assert_eq!(output, 3);
}
#[test]
fn test3() {
    let input = vec![Some(1)];
    let len = input.len();
    let b_tree = create_b_tree(input, 0, len);

    let output = Solution::kth_smallest(b_tree, 1);
    assert_eq!(output, 1);
}
