#![allow(unused)]
use crate::*;

#[test]
fn test1() {
    let nums = vec![2, 20, 4, 10, 3, 4, 5];
    let result = Solution::longest_consecutive(nums);
    assert_eq!(4, result);
}

#[test]
fn test2() {
    let nums = vec![0, 3, 2, 5, 4, 6, 1, 1];
    let result = Solution::longest_consecutive(nums);
    assert_eq!(7, result);
}

#[test]
fn test3() {
    let nums = vec![0,3,7,2,5,8,4,6,0,1];
    let result = Solution::longest_consecutive(nums);
        assert_eq!(9, result);

}
