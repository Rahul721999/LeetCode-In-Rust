#![allow(unused)]
use crate::*;

#[test]
fn test1() {
    let input = vec![10, 15, 20];
    let expected = 15;

    let output = Solution::min_cost_climbing_stairs(input);
    assert_eq!(output, expected);
}

#[test]
fn test2() {
    let input = vec![1, 100, 1, 1, 1, 100, 1, 1, 100, 1];
    let expected = 6;

    let output = Solution::min_cost_climbing_stairs(input);
    assert_eq!(output, expected);
}

#[test]
fn test3() {
    let input = vec![10, 15];
    let expected = 10;

    let output = Solution::min_cost_climbing_stairs(input);
    assert_eq!(output, expected);
}
