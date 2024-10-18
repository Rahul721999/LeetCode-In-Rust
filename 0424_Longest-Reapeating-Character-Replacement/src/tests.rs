#![allow(unused)]
use crate::*;

#[test]
fn test1() {
    let output = Solution::character_replacement("ABAB".to_string(), 2);
    assert_eq!(output, 4);
}

#[test]
fn test2() {
    let output = Solution::character_replacement("AABABBA".to_string(), 1);
    assert_eq!(output, 4);
}

#[test]
fn test3() {
    let output = Solution::character_replacement("abaa".to_string(), 0);
    assert_eq!(output, 2);
}
