#![allow(unused)]
use crate::*;

#[test]
fn test1() {
    let word1 = "horse".to_string();
    let word2 = "ros".to_string();
    let output = Solution::min_distance(word1, word2);
    assert_eq!(output, 3);
}

#[test]
fn test2() {
    let word1 = "intention".to_string();
    let word2 = "execution".to_string();
    let output = Solution::min_distance(word1, word2);
    assert_eq!(output, 5);
}

#[test]
fn test3() {
    let word1 = "abc".to_string();
    let word2 = "ros".to_string();
    let output = Solution::min_distance(word1, word2);
    assert_eq!(output, 3);
}
