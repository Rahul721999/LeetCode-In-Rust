#![allow(unused)]
use crate::*;

#[test]
fn test1() {
    let intput = vec![2,2,2,2,5,5,5,8];
    let output = Solution::num_of_subarrays(intput, 3, 4);
    assert_eq!(output, 3);
}

#[test]
fn test2() {
    let intput = vec![11,13,17,23,29,31,7,5,2,3];
    let output = Solution::num_of_subarrays(intput, 3, 5);
    assert_eq!(output, 6);
}

#[test]
fn test3() {
    let intput = vec![1];
    let output = Solution::num_of_subarrays(intput, 1, 1);
    assert_eq!(output, 1);
}
