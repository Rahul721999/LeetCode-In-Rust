#![allow(unused)]
use crate::*;

#[test]
fn test1() {
    let mut input = vec![
        vec![2147483647, -1, 0, 2147483647],
        vec![2147483647, 2147483647, 2147483647, -1],
        vec![2147483647, -1, 2147483647, -1],
        vec![0, -1, 2147483647, 2147483647],
    ];
    let expected = vec![
        vec![3, -1, 0, 1],
        vec![2, 2, 1, -1],
        vec![1, -1, 2, -1],
        vec![0, -1, 3, 4],
    ];
    Solution::walls_and_gates(&mut input);
    assert_eq!(expected, input);
}

#[test]
fn test2() {
    let mut input = vec![vec![0, -1], vec![2147483647, 2147483647]];
    let expected = vec![vec![0, -1], vec![1, 2]];
    Solution::walls_and_gates(&mut input);
    assert_eq!(expected, input);
}

#[test]
fn test3() {
    // Add test here
}
