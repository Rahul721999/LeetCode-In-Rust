#![allow(unused)]
use crate::*;

#[test]
fn test1() {
    let input = "cbaebabacd".to_string();
    let anagram = "abc".to_string();

    let output = Solution::find_anagrams(input, anagram);

    assert_eq!(output, vec![0, 6]);
}

#[test]
fn test2() {
    let input = "abab".to_string();
    let anagram = "ab".to_string();

    let output = Solution::find_anagrams(input, anagram);

    assert_eq!(output, vec![0, 1, 2]);
}

#[test]
fn test3() {
    let input = "abcdef".to_string();
    let anagram = "xyz".to_string();

    let output = Solution::find_anagrams(input, anagram);

    assert_eq!(output, vec![]);
}

#[test]
fn test4() {
    let input = "aaabbbccc".to_string();
    let anagram = "ccc".to_string();

    let output = Solution::find_anagrams(input, anagram);

    assert_eq!(output, vec![6]);
}

