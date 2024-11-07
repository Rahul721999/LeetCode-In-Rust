#![allow(unused)]
use crate::*;

#[test]
fn test1() {
    let input = vec![17,18,5,4,6,1];
    let output = Solution::replace_elements(input);
    assert_eq!(output, vec![18,6,6,6,1,-1]);
}

#[test]
fn test2() {
    let input = vec![1];
    let output = Solution::replace_elements(input);
    assert_eq!(output, vec![-1]);
}
