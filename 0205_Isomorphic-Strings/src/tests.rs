#![allow(unused)]
use crate::*;

#[test]
fn test1() {
    let s = "egg".to_string();
    let t = "add".to_string();
    let output = Solution::is_isomorphic(s, t);
    assert!(output);
}

#[test]
fn test2() {
    let s = "foo".to_string();
    let t = "bar".to_string();
    let output = Solution::is_isomorphic(s, t);
    assert!(!output);
}

#[test]
fn test3() {
    let s = "paper".to_string();
    let t = "title".to_string();
    let output = Solution::is_isomorphic(s, t);
    assert!(output);
}
