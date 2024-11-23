#![allow(unused)]
use crate::*;

#[test]
fn test1() {
    let input = vec![5, 0, 0, 0];
    assert_eq!(Solution::check_subarray_sum(input, 3), true);
}

#[test]
fn test2() {
    let input = vec![23, 2, 4, 6, 7];
    assert_eq!(Solution::check_subarray_sum(input.clone(), 6), true);
}

#[test]
fn test3() {
    let input = vec![23, 2, 4, 6, 7];
    assert_eq!(Solution::check_subarray_sum(input, 7), true);
}
