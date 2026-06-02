#![allow(unused)]
use crate::*;

#[test]
fn test1() {
    let nums = vec![4,5,6,7,0,1,2];
    let output = Solution::search(nums, 0);
    assert_eq!(output, 4);
}

#[test]
fn test2() {
    let nums = vec![4,5,6,7,0,1,2];
    let output = Solution::search(nums,-1);
    assert_eq!(output, -1);
}

#[test]
fn test3() {
    let nums = vec![1];
    let output = Solution::search(nums, 0);
    assert_eq!(output, -1);
}
