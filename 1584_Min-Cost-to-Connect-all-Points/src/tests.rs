#![allow(unused)]
use crate::*;

#[test]
fn test1() {
    let input = vec![vec![0, 0], vec![2, 2], vec![3, 10], vec![5, 2], vec![7, 0]];
    let output = Solution::min_cost_connect_points(input);

    assert_eq!(20, output);
}

#[test]
fn test2() {
    let input = vec![vec![3, 12], vec![-2, 5], vec![-4, 1]];
    let output = Solution::min_cost_connect_points(input);

    assert_eq!(18, output);
}

#[test]
fn test3() {
    let input = vec![vec![0, 0], vec![1, 1]];
    let output = Solution::min_cost_connect_points(input);

    assert_eq!(2, output);
}
