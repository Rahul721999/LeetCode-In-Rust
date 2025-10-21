#![allow(unused)]
use crate::*;

#[test]
fn test1() {
    let input = vec![vec![0, 1], vec![0, 2], vec![0, 3], vec![1, 4]];
    let output = Solution::valid_graph(5, input);
    assert!(output);
}

#[test]
fn test2() {
    let input = vec![vec![0, 1], vec![1, 2], vec![2, 3], vec![1, 3], vec![1, 4]];
    let output = Solution::valid_graph(5, input);
    assert!(!output);
}

#[test]
fn test3() {
    let input = vec![vec![1,0], vec![2,0], vec![3,0]];
    let output = Solution::valid_graph(4, input);
    assert!(output);
}
