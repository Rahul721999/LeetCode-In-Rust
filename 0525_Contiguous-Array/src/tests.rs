#![allow(unused)]
use crate::*;

#[test]
fn test1() {
    // Test case with an equal number of 0s and 1s
    let nums = vec![0, 1, 0, 1];
    let result = Solution::find_max_length(nums);
    assert_eq!(result, 4); // The entire array has an equal number of 0s and 1s
}

#[test]
fn test2() {
    // Test case where the longest subarray is in the middle
    let nums = vec![0, 1, 1, 0, 0, 1];
    let result = Solution::find_max_length(nums);
    assert_eq!(result, 6); // The entire array is balanced
}

#[test]
fn test3() {
    // Test case with no balanced subarray
    let nums = vec![0, 0, 1, 1, 0];
    let result = Solution::find_max_length(nums);
    assert_eq!(result, 4); // The subarray [0, 0, 1, 1] has the maximum length
}
