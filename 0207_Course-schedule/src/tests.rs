#![allow(unused)]
use crate::*;

#[test]
fn test1() {
    let num_courses = 4;
    let prerequisites = vec![vec![1, 0], vec![2, 0], vec![3, 1], vec![3, 2]];

    let output = Solution::can_finish(num_courses, prerequisites);
    assert!(output);
}

#[test]
fn test2() {
    let num_courses = 2;
    let prerequisites = vec![vec![0,1]];

    let output = Solution::can_finish(num_courses, prerequisites);
    assert!(output);
}

#[test]
fn test3() {
    let num_courses = 2;
    let prerequisites = vec![vec![0,1], vec![1,0]];

    let output = Solution::can_finish(num_courses, prerequisites);
    assert!(!output);
}
