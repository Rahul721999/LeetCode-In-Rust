#![allow(unused)]
use crate::*;

#[test]
fn test1() {
    let input = vec![10, 5, 2, 6];
    let output = Solution::num_subarray_product_less_than_k(input, 100);
    assert_eq!(output, 8);
}

#[test]
fn test2() {
    let input = vec![1,2,3];
    let output = Solution::num_subarray_product_less_than_k(input, 0);
    assert_eq!(output, 0);
}

#[test]
fn test3() {
    let input = vec![1,1,1];
    let output = Solution::num_subarray_product_less_than_k(input, 2);
    assert_eq!(output, 6);
}
