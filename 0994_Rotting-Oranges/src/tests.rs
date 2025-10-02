#![allow(unused)]
use crate::*;

#[test]
fn test1() {
    let input = vec![vec![2, 1, 1], vec![1, 1, 0], vec![0, 1, 1]];
    let expected = 4;
    assert_eq!(expected, Solution::oranges_rotting(input));
}

#[test]
fn test2() {
   let input = vec![vec![2,1,1],vec![0,1,1],vec![1,0,1]];
    let expected = -1; 
    assert_eq!(expected, Solution::oranges_rotting(input));
}

#[test]
fn test3() {
    // Add test here
    let input = vec![vec![0,2]];
    let expected = 0;
    assert_eq!(expected, Solution::oranges_rotting(input));
}
