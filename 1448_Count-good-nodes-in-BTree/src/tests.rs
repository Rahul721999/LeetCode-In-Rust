#![allow(unused)]
use crate::*;

#[test]
fn test1() {
    let input = vec![Some(3), Some(1), Some(4), Some(3), None, Some(1), Some(5)];
    let len = input.len();
    let b_tree = create_b_tree(input, 0, len);

    let output = Solution::good_nodes(b_tree);
    assert_eq!(output, 4);
}
#[test]
fn test2() {
    let input = vec![Some(1)];
    let len = input.len();
    let b_tree = create_b_tree(input, 0, len);

    let output = Solution::good_nodes(b_tree);
    assert_eq!(output, 1);
}
#[test]
fn test3() {
    let input = vec![Some(3),  Some(3), None, Some(4), Some(2)];
    let len = input.len();
    let b_tree = create_b_tree(input, 0, len);

    let output = Solution::good_nodes(b_tree);
    assert_eq!(output, 3);
}
