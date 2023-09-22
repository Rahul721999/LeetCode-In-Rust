
#[cfg(test)]

#[test]
fn test1() {
    assert_eq!(
        crate::Solution::str_str("leetcode".to_string(), "leeto".to_string()),
        -1
    );
}
#[test]
fn test2() {
    assert_eq!(
        crate::Solution::str_str("sadbutsad".to_string(), "sad".to_string()),0
    );
}

#[test]
fn test3() {
    assert_eq!(
        crate::Solution::str_str("adbutsad".to_string(), "sad".to_string()),
        5
    );
}
#[test]
fn test4() {
    assert_eq!(
        crate::Solution::str_str("hello".to_string(), "ll".to_string()),
        2
    );
}