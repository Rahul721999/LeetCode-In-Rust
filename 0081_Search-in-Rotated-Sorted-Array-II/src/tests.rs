#![allow(unused)]
use crate::*;

#[test]
fn test1() {
    let nums = vec![1,1,1,1,1,1,1,1,1,1,1,1,1,2,1,1,1,1,1];
    let output = Solution::search(nums, 2);
    assert!(output);
}

#[test]
fn test2() {
    let nums = vec![1,3,1,1,1];
    let target = 3;
    let output = Solution::search(nums, target);
    assert!(output);
}

#[test]
fn test3() {
    let nums = vec![3,4,4,5,6,1,2,2];
    let target = 1;
    let output = Solution::search(nums, target);
    assert!(output);
}
