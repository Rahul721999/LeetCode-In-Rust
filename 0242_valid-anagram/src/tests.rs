#![allow(unused)]
use super::*;

#[test]
fn test1() {
    assert_eq!(
        Solution::is_anagram("anagram".to_string(), "nagaram".to_string()),
        true
    );
}

#[test]
fn test2() {
    assert_eq!(
        Solution::is_anagram("car".to_string(), "rat".to_string()),
        false
    );
}
#[test]
fn test3() {
    assert_eq!(
        Solution::is_anagram("racecar".to_string(), "carrace".to_string()),
        true
    );
}
