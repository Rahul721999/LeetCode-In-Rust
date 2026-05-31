#![allow(unused)]
use crate::*;

#[test]
fn test1() {
    let input = vec![vec![1,2,3],vec![4,5,6],vec![7,8,9]];
   let output = Solution::spiral_order(input );
   assert_eq!(output, vec![1,2,3,6,9,8,7,4,5]);
}

#[test]
fn test2() {
    let input = vec![vec![1,2,3,4],vec![5,6,7,8],vec![9,10,11,12]];
   let output = Solution::spiral_order(input );
   assert_eq!(output, vec![1,2,3,4,8,12,11,10,9,5,6,7]);
}

#[test]
fn test3() {
    let input = vec![vec![1,2],vec![3,4]];
   let output = Solution::spiral_order(input );
   assert_eq!(output, vec![1,2,4,3]);
}
