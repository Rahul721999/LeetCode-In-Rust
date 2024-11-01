#![allow(unused)]
use crate::*;

#[test]
fn test1() {
    let (mut nums1, m) = (vec![1, 2, 3, 0, 0, 0], 3);
    let (mut nums2, n) = (vec![2, 5, 6], 3);

    Solution::merge(&mut nums1, m, &mut nums2, n);

    assert_eq!(nums1, vec![1, 2, 2, 3, 5, 6]);
}

#[test]
fn test2() {
    let (mut nums1, m) = (vec![1], 1);
    let (mut nums2, n) = (vec![], 0);

    Solution::merge(&mut nums1, m, &mut nums2, n);

    assert_eq!(nums1, vec![1]);
}

#[test]
fn test3() {
    let (mut nums1, n) = (vec![], 0);
    let (mut nums2, m) = (vec![1], 1);

    Solution::merge(&mut nums1, m, &mut nums2, n);

    assert_eq!(nums1, vec![1]);
}
