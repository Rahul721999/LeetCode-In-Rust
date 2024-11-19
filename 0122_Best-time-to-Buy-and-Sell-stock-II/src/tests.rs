#![allow(unused)]
use crate::*;

#[test]
fn test1() {
    let output = Solution::max_profit(vec![7,1,5,3,6,4]);
    assert_eq!(output, 7);
}

#[test]
fn test2() {
    let output = Solution::max_profit(vec![1,2,3,4,5]);
    assert_eq!(output, 4);
}

#[test]
fn test3() {
    let output = Solution::max_profit(vec![7,6,4,3,1]);
    assert_eq!(output, 0);
}
