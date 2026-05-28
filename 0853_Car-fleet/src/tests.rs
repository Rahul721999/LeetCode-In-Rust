#![allow(unused)]
use crate::*;

#[test]
fn test1() {
    let target = 12;
    let position = vec![10, 8, 0, 5, 3];
    let speed = vec![2, 4, 1, 1, 3];
    let output = Solution::car_fleet(target, position, speed);
    assert_eq!(output, 3);
}

#[test]
fn test2() {
    let target = 10;
    let position = vec![4,1,0,7];
    let speed = vec![2,2,1,1];
    let output = Solution::car_fleet(target, position, speed);
    assert_eq!(output, 3);
}

#[test]
fn test3() {
    let target = 10;
    let position = vec![1,4];
    let speed = vec![3,2];
    let output = Solution::car_fleet(target, position, speed);
    assert_eq!(output, 1);
}
