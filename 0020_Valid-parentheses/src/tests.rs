#![allow(unused)]
use crate::*;

#[test]
fn test1() {
    let output = Solution::is_valid("[{}".to_string());
    assert_eq!(output, false);
}

#[test]
fn test2() {
    let output = Solution::is_valid("[{}]".to_string());
    assert_eq!(output, true);
}

#[test]
fn test3() {
    let output = Solution::is_valid(")(".to_string());
    assert_eq!(output, false);
}
