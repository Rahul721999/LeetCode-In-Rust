#![allow(unused)]
use crate::*;

#[test]
fn test1() {
    let num_courses = 3;
    let prerequisites = vec![vec![1, 0]];

    let output = Solution::find_order(num_courses, prerequisites);
    assert_eq!(output, vec![0,2,1]);
}

#[test]
fn test2() {
    let num_courses = 4;
    let prerequisites = vec![vec![1, 0], vec![2, 0], vec![3, 1], vec![3, 2]];

    let output = Solution::find_order(num_courses, prerequisites);
    assert_eq!(output, vec![0,1,2,3]);
}

#[test]
fn test3() {
    let num_courses = 2;
    let prerequisites = vec![vec![1, 0]];

    let output = Solution::find_order(num_courses, prerequisites);
    assert_eq!(output, vec![0,1]);
}
