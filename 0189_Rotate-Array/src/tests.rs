#![allow(unused)]
use crate::*;

#[test]
fn test1() {
    let mut input = vec![1, 2, 3, 4, 5, 6, 7];
    Solution::rotate(&mut input, 3);
    assert_eq!(input, vec![5, 6, 7, 1, 2, 3, 4]);
}

#[test]
fn test2() {
    let mut input = vec![1];
    Solution::rotate(&mut input, 0);
    assert_eq!(input, vec![1]);
}

#[test]
fn test3() {
    let mut input = vec![-1, -100, 3, 99];
    Solution::rotate(&mut input, 2);
    assert_eq!(input, vec![3, 99, -1, -100]);
}

#[test]
fn test4() {
    let mut input = vec![3, 99];
    Solution::rotate(&mut input, 0);
    assert_eq!(input, vec![3, 99]);
}
#[test]
fn test5() {
    let mut input = vec![3, 99];
    Solution::rotate(&mut input, 2);
    assert_eq!(input, vec![3, 99]);
}
