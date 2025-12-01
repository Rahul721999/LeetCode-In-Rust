#![allow(unused)]
use crate::*;

#[test]
fn test1() {
    let nums = vec![2, 5, 6, 9];
    let target = 9;
    let result = Solution::combination_sum(nums.to_vec(), target);
    let expected = vec![vec![2, 2, 5], vec![9]];
    expected.iter().for_each(|v| assert!(result.contains(v)));
}

#[test]
fn test2() {
    let nums = vec![3, 4, 5];
    let target = 16;
    let result = Solution::combination_sum(nums.to_vec(), target);
    let expected = vec![
        vec![3, 3, 3, 3, 4],
        vec![3, 3, 5, 5],
        vec![4, 4, 4, 4],
        vec![3, 4, 4, 5],
    ];
    expected.iter().for_each(|v| assert!(result.contains(v)));
}

#[test]
fn test3() {
    let nums = vec![3];
    let target = 5;
    let result = Solution::combination_sum(nums.to_vec(), target);
    assert!(result.is_empty());
}
