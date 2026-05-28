#![allow(unused)]
use crate::*;

#[test]
fn test1() {
    let input = vec![-1,0,3,5,9,12];
    let output = Solution::search(input, 9);
    assert_eq!(output, 4);
}

#[test]
fn test2() {
    let input = vec![-1,0,3,5,9,12];
    let output = Solution::search(input, 2);
    assert_eq!(output, -1);
}

#[test]
fn test3() {
    let input = vec![-1,0,2,4,6,8];
    let output = Solution::search(input, 4);
    assert_eq!(output, 3);
}
#[test]
fn test4() {
    let input = vec![5];
    let output = Solution::search(input, 4);
    assert_eq!(output, -1);
}
