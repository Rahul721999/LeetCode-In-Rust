#![allow(unused)]
use crate::*;

#[test]
fn test1() {
    let matrix = vec![vec![1, 3, 5, 7], vec![10, 11, 16, 20], vec![23, 30, 34, 60]];
    let output = Solution::search_matrix(matrix, 34);
    assert!(output);
}

#[test]
fn test2() {
    let matrix = vec![vec![1,3,5,7],vec![10,11,16,20],vec![23,30,34,50]];
    let output = Solution::search_matrix(matrix, 11);
    assert!(output);
}

#[test]
fn test3() {
    let matrix = vec![vec![1, 3, 5, 7], vec![10, 11, 16, 20], vec![23, 30, 34, 60]];
    let output = Solution::search_matrix(matrix, 21);
    assert!(!output);
}
