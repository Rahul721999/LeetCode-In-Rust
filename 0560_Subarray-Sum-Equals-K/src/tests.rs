#![allow(unused)]
use crate::Solution;

#[test]
fn test1() {
    // Test with positive numbers
    let nums = vec![1, 2, 3];
    let k = 3;
    assert_eq!(Solution::subarray_sum(nums, k), 2); // Subarrays: [1, 2], [3]
}

#[test]
fn test2() {
    // Test with negative numbers
    let nums = vec![1, -1, 1, -1, 1];
    let k = 0;
    assert_eq!(Solution::subarray_sum(nums, k), 6); // Subarrays: [1, -1], [-1, 1], [1, -1, 1, -1], [1, -1], [-1, 1, -1, 1], [-1, 1]
}

#[test]
fn test3() {
    // Test with all elements equal
    let nums = vec![5, 5, 5];
    let k = 5;
    assert_eq!(Solution::subarray_sum(nums, k), 3); // Subarrays: [5], [5], [5]
}

#[test]
fn test4() {
    // Test with a mix of positive and negative numbers
    let nums = vec![3, 4, -7, 1, 3, 3, 1, -4];
    let k = 7;
    assert_eq!(Solution::subarray_sum(nums, k), 4); 
    // Subarrays: [3, 4], [3, 4, -7, 1, 3, 3], [1, 3, 3], [7]
}

#[test]
fn test5() {
    // Test with no valid subarrays
    let nums = vec![1, 2, 3];
    let k = 10;
    assert_eq!(Solution::subarray_sum(nums, k), 0); // No subarrays sum to 10
}

#[test]
fn test6() {
    // Test with empty input
    let nums: Vec<i32> = vec![];
    let k = 0;
    assert_eq!(Solution::subarray_sum(nums, k), 0); // No subarrays
}

#[test]
fn test7() {
    // Test with a single element equal to k
    let nums = vec![3];
    let k = 3;
    assert_eq!(Solution::subarray_sum(nums, k), 1); // Subarray: [3]
}

#[test]
fn test8() {
    // Test with a single element not equal to k
    let nums = vec![3];
    let k = 5;
    assert_eq!(Solution::subarray_sum(nums, k), 0); // No subarrays
}
