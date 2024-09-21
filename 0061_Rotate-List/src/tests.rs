#![allow(unused)]
use crate::{helper, Solution};

#[test]
fn test1(){
    let input = helper(vec![1, 2, 3, 4, 5]);
    let expected_output = helper(vec![4,5,1,2,3]);
    let output = Solution::rotate_right(input, 2);
    assert_eq!(output, expected_output);
}
#[test]
fn test2(){
    let input = helper(vec![0,1,2]);
    let expected_output = helper(vec![2,0,1]);
    let output = Solution::rotate_right(input, 4);
    assert_eq!(output, expected_output);
}
#[test]
fn test3(){
    let input = helper(vec![0,1,2]);
    let expected_output = helper(vec![0,1,2]);
    let output = Solution::rotate_right(input, 9);
    assert_eq!(output, expected_output);
}
#[test]
fn test4(){
    let input = helper(vec![1, 2, 3, 4, 5]);
    let expected_output = helper(vec![4,5,1,2,3]);
    let output = Solution::rotate_right(input, 17);
    assert_eq!(output, expected_output);
}