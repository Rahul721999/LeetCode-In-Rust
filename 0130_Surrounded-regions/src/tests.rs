#![allow(unused)]
use crate::*;

#[test]
fn test1() {
    let mut input = vec![
        vec!['X', 'X', 'X', 'X'],
        vec!['X', 'O', 'O', 'X'],
        vec!['X', 'X', 'O', 'X'],
        vec!['X', 'O', 'X', 'X'],
    ];

    let expected = vec![
        vec!['X', 'X', 'X', 'X'],
        vec!['X', 'X', 'X', 'X'],
        vec!['X', 'X', 'X', 'X'],
        vec!['X', 'O', 'X', 'X'],
    ];

    Solution::solve(&mut input);

    assert_eq!(expected, input);
}

#[test]
fn test2() {
    // Add test here
}

#[test]
fn test3() {
    // Add test here
}
