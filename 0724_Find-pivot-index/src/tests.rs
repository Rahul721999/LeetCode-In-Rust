#![allow(unused)]
use crate::*;

#[test]
fn test1() {
    let nums = vec![1,7,3,6,5,6];
    let output = Solution::pivot_index(nums);
    assert_eq!(output, 3);
}

#[test]
fn test2() {
    let nums = vec![2,1,-1];
    let output = Solution::pivot_index(nums);
    assert_eq!(output, 0);
}

#[test]
fn test3() {
    let nums = vec![0];
    let output = Solution::pivot_index(nums);
    assert_eq!(output, 0);
}
#[test]
fn test4() {
    let nums = vec![1,2,3];
    let output = Solution::pivot_index(nums);
    assert_eq!(output, -1);
}
