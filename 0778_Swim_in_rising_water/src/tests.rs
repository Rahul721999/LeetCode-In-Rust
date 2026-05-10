#![allow(unused)]
use crate::*;

#[test]
fn test1() {
    let input = vec![
        vec![0, 1, 2, 3, 4],
        vec![24, 23, 22, 21, 5],
        vec![12, 13, 14, 15, 16],
        vec![11, 17, 18, 19, 20],
        vec![10, 9, 8, 7, 6],
    ];
    let output = Solution::swim_in_water(input);
    assert_eq!(output, 16)
}

#[test]
fn test2() {
    let input = vec![vec![0,2],vec![1,3]];
    let output = Solution::swim_in_water(input);
    assert_eq!(output, 3)
}

#[test]
fn test3() {
    let input = vec![
        vec![0, 1, 2, 10],
        vec![9, 14, 4, 13],
        vec![12, 3, 8, 15],
        vec![11, 5, 7, 6],
    ];
    let output = Solution::swim_in_water(input);
    assert_eq!(output, 8)
}
