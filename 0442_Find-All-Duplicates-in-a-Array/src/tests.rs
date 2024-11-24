#![allow(unused)]
use crate::*;

#[test]
fn test1() {
    let input = vec![4,3,2,7,8,2,3,1];
    let output = Solution::find_duplicates(input);
    assert_eq!(output, vec![2,3]);
}

#[test]
fn test2() {
    let input = vec![1,1,2];
    let output = Solution::find_duplicates(input);
    assert_eq!(output, vec![1]);
}

#[test]
fn test3() {
    let input = vec![1];
    let output = Solution::find_duplicates(input);
    assert_eq!(output, vec![]);
}
