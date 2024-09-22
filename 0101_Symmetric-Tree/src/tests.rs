#![allow(unused)]
use crate::{create_b_tree, Solution};
#[test]
fn test1() {
    let input_vec = vec![
        Some(1),
        Some(2),
        Some(2),
        Some(3),
        Some(4),
        Some(4),
        Some(3),
    ];
    let len = input_vec.len();
    let input = create_b_tree(input_vec, 0, len);
    assert_eq!(Solution::is_symmetric(input), true);
}

#[test]
fn test2() {
    let input = vec![Some(1), Some(2), Some(2), None, Some(3), None, Some(3)];
    let len = input.len();
    let input = create_b_tree(input, 0, len);
    assert_eq!(Solution::is_symmetric(input), false);
}
#[test]
fn test3() {
    let input = vec![None];
    let len = input.len();
    let input = create_b_tree(input, 0, len);
    assert_eq!(Solution::is_symmetric(input), true);
}
#[test]
fn test4() {
    let input = vec![Some(1), None, None];
    let len = input.len();
    let input = create_b_tree(input, 0, len);
    assert_eq!(Solution::is_symmetric(input), true);
}
