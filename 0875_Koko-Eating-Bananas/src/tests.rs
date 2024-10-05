#![allow(unused)]
use crate::*;

#[test]
fn test1() {
    let res = Solution::min_eating_speed(vec![30, 11, 23, 4, 20], 5);
    assert_eq!(res, 30);
}

#[test]
fn test2() {
    let res = Solution::min_eating_speed(vec![3,6,7,11], 8);
    assert_eq!(res, 4);
}

#[test]
fn test3() {
    let res = Solution::min_eating_speed(vec![30,11,23,4,20], 6);
    assert_eq!(res, 23);
}
