#![allow(unused)]
use crate::*;

#[test]
fn test1() {
    let n = 6;
    let edges = vec![vec![0, 1], vec![1, 2], vec![2, 3], vec![4, 5]];
    let result = Solution::count_component(n, edges);

    assert_eq!(result, 2);
}

#[test]
fn test2() {
    let n = 4;
    let edges = vec![vec![0, 1], vec![1, 2], vec![2, 3]];
    let result = Solution::count_component(n, edges);

    assert_eq!(result, 1);
}

#[test]
fn test3() {
    let n = 6;
    let edges = vec![vec![0, 1], vec![1, 2], vec![3,4]];
    let result = Solution::count_component(n, edges);

    assert_eq!(result, 3);
}
