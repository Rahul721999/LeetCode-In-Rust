#![allow(unused)]
use crate::*;

#[test]
fn test1() {
    let intput = "abciiidef".to_string();
    let output = Solution::max_vowels(intput, 3);
    assert_eq!(output, 3);
}

#[test]
fn test2() {
    let intput = "aeiou".to_string();
    let output = Solution::max_vowels(intput, 3);
    assert_eq!(output, 3);
}

#[test]
fn test3() {
    let intput = "leetcode".to_string();
    let output = Solution::max_vowels(intput, 3);
    assert_eq!(output, 2);
}
