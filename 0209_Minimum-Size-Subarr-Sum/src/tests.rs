#![allow(unused)]
use crate::*;

#[test]
fn test1() {
    let target = 7;
    let nums = vec![2,3,1,2,4,3];
    let output = Solution::min_sub_array_len(target, nums);
    assert_eq!(output, 2);
}

#[test]
fn test2() {
    let target = 4;
    let nums = vec![1,4,4];
    let output = Solution::min_sub_array_len(target, nums);
    assert_eq!(output, 1);
}

#[test]
fn test3() {
    let target = 11;
    let nums = vec![1,1,1,1,1,1,1,1];
    let output = Solution::min_sub_array_len(target, nums);
    assert_eq!(output, 0);
}
