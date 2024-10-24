#![allow(unused)]
use crate::*;

#[test]
fn test1() {
    let intput = vec![1, 1, 4, 2, 3];
    let output = Solution::min_operations(intput, 5);
    assert_eq!(output, 2);
}

#[test]
fn test2() {
    let intput = vec![5,6,7,8,9];
    let output = Solution::min_operations(intput, 4);
    assert_eq!(output, -1);
}

#[test]
fn test3() {
    let intput = vec![3,2,20,1,1,3];
    let output = Solution::min_operations(intput, 10);
    assert_eq!(output, 5);
}
