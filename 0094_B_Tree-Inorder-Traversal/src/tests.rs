#![allow(unused)]
use crate::*;

#[test]
fn test1() {
    let input = vec![Some(1), None, Some(2), None, None, Some(3)];
    let len = input.len();
    let tree = create_b_tree(input, 0, len);
    let output = Solution::inorder_traversal(tree);
    println!("{output:?}");
    let expeted = vec![1, 3, 2];

    assert_eq!(output, expeted);
}

#[test]
fn test2() {
    let input: Vec<Option<i32>> = vec![
        Some(1),
        Some(2),
        Some(3),
        Some(4),
        Some(5),
        None,
        Some(8),
        None,
        None,
        Some(6),
        Some(7),
        None, None,
        Some(9),
    ];
    let len = input.len();
    let tree = create_b_tree(input, 0, len);
    let output = Solution::inorder_traversal(tree);
    println!("{output:?}");
    let expeted = vec![4,2,6,5,7,1,3,9,8];

    assert_eq!(output, expeted);
}

#[test]
fn test3() {
    let input = vec![];
    let len = input.len();
    let tree = create_b_tree(input, 0, len);
    let output = Solution::inorder_traversal(tree);
    println!("{output:?}");
    let expeted = vec![];

    assert_eq!(output, expeted);
}
#[test]
fn test4() {
    let input = vec![Some(1)];
    let len = input.len();
    let tree = create_b_tree(input, 0, len);
    let output = Solution::inorder_traversal(tree);
    println!("{output:?}");
    let expeted = vec![1];

    assert_eq!(output, expeted);
}
