#![allow(unused)]
use crate::*;

#[test]
fn test1() {
    let mut intput = vec![2,0,2,1,1,0];
    Solution::sort_colors(&mut intput);
    assert_eq!(intput, vec![0,0,1,1,2,2]);
}

#[test]
fn test2() {
    let mut intput = vec![2,0,1];
    Solution::sort_colors(&mut intput);
    assert_eq!(intput, vec![0,1,2]);
}

#[test]
fn test3() {
    let mut intput = vec![2,0,0,1];
    Solution::sort_colors(&mut intput);
    assert_eq!(intput, vec![0,0,1,2]);
}
