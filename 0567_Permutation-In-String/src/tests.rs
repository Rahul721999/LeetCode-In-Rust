#![allow(unused)]
use crate::*;

#[test]
fn test1() {
    let s1 = "abc".to_string();
    let s2 = "lecabee".to_string();
    assert!(Solution::check_inclusion(s1, s2));
}

#[test]
fn test2() {
    let s1 = "abc".to_string();
    let s2 = "lecaeeb".to_string();
    assert!(!Solution::check_inclusion(s1, s2));
}

#[test]
fn test3() {
    let s1 = "ab".to_string();
    let s2 = "eidbaooo".to_string();
    assert!(Solution::check_inclusion(s1, s2));
}
#[test]
fn test4() {
    let s1 = "ab".to_string();
    let s2 = "eidboaooo".to_string();
    assert!(!Solution::check_inclusion(s1, s2));
}
