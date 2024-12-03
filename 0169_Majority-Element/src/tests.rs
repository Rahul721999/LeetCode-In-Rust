#![allow(unused)]
use crate::*;

#[test]
fn test1() {
    // Test case with a simple majority element
    let nums = vec![3, 2, 3];
    assert_eq!(Solution::majority_element(nums), 3);
}

#[test]
fn test2() {
    // Test case with a single element
    let nums = vec![1];
    assert_eq!(Solution::majority_element(nums), 1);
}

#[test]
fn test3() {
    // Test case where the majority element appears multiple times
    let nums = vec![2, 2, 1, 1, 1, 2, 2];
    assert_eq!(Solution::majority_element(nums), 2);
}
