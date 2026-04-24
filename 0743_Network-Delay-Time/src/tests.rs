#![allow(unused)]
use crate::*;

#[test]
fn test1() {
    let times = vec![vec![2, 1, 1], vec![2, 3, 1], vec![3, 4, 1]];
    let output = Solution::network_delay_time(times, 4, 2);
    assert_eq!(2, output);
}

#[test]
fn test2() {
    let times = vec![vec![1,2,1], vec![2, 3, 1], vec![1,4,4], vec![3,4,1]];
    let output = Solution::network_delay_time(times, 4, 1);
    assert_eq!(3, output);
}

#[test]
fn test3() {
    let times = vec![vec![1,2,1], vec![2,3,1]];
    let output = Solution::network_delay_time(times, 3, 2);
    assert_eq!(-1, output);
}
