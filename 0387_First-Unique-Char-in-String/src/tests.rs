#![allow(unused)]
use crate::*;

#[test]
fn test1() {
    let output = Solution::first_uniq_char("leetcode".to_string());
    assert_eq!(output, 0);
}

#[test]
fn test2() {
    let output = Solution::first_uniq_char("loveleetcode".to_string());
    assert_eq!(output, 2);
}

#[test]
fn test3() {
    let output = Solution::first_uniq_char("aabb".to_string());
    assert_eq!(output, -1);
}
