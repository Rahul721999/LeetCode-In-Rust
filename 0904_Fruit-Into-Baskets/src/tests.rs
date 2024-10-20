#![allow(unused)]
use crate::*;

#[test]
fn test1() {
    let input = vec![1,2,3,2,2];
    let output = Solution::total_fruit(input);
    assert_eq!(4, output);
}

#[test]
fn test2() {
    let input = vec![0,1,2,2];
    let output = Solution::total_fruit(input);
    assert_eq!(3, output);
}

#[test]
fn test3() {
    let input = vec![3, 3, 3, 1, 2, 1, 1, 2, 3, 3, 4];
    let output = Solution::total_fruit(input);
    assert_eq!(5, output);
}
