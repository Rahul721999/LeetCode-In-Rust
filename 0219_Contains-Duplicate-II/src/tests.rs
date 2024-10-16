#![allow(unused)]
use crate::*;

#[test]
fn test1() {
    let input = vec![1,2,3,1];
    let output = Solution::contains_nearby_duplicate(input, 3);
    assert!(output);
}

#[test]
fn test2() {
    let input = vec![1,0,1,1];
    let output = Solution::contains_nearby_duplicate(input, 1);
    assert!(output);
}

#[test]
fn test3() {
    let input = vec![1,2,3,1,2,3];
    let output = Solution::contains_nearby_duplicate(input, 2);
    assert!(!output);
}
