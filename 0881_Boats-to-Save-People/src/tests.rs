#![allow(unused)]
use crate::*;

#[test]
fn test1() {
    let input = vec![1,2];
    let limit = 3;
    assert_eq!(Solution::num_rescue_boats(input, limit), 1);
}

#[test]
fn test2() {
    let input = vec![3,2,2,1];
    let limit = 3;
    assert_eq!(Solution::num_rescue_boats(input, limit), 3);
}

#[test]
fn test3() {
    let input = vec![3,5,3,4];
    let limit = 5;
    assert_eq!(Solution::num_rescue_boats(input, limit), 4);
}

#[test]
fn test4() {
    let input = vec![1,1,1,2,3,2];
    let limit = 3;
    assert_eq!(Solution::num_rescue_boats(input, limit), 4);
}
