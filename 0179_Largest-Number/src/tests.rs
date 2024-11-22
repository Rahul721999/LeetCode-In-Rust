#![allow(unused)]
use crate::*;

#[test]
fn test1() {
    let output = Solution::largest_number(vec![3,30,34,5,9]);
    assert_eq!(output, "9534330".to_string());
}

#[test]
fn test2() {
    let output = Solution::largest_number(vec![10,2]);
    assert_eq!(output, "210".to_string());
}

#[test]
fn test3() {
    let output = Solution::largest_number(vec![700000000,500000000]);
    assert_eq!(output, "700000000500000000".to_string());
}

#[test]
fn test4() {
    let output = Solution::largest_number(vec![0,0,0,0,0,0]);
    assert_eq!(output, "0".to_string());
}
#[test]
fn test5() {
    let output = Solution::largest_number(vec![0,0,0,0,0,0, 1]);
    assert_eq!(output, "1000000".to_string());
}