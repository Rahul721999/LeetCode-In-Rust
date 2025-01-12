#![allow(unused)]
use crate::*;

#[test]
fn test1() {
    let mut input = vec![1, 3, 5, 4, 2];
    Solution::next_permutation(&mut input);

    let expected = vec![1, 4, 2, 3, 5];
    assert_eq!(input, expected);
}

#[test]
fn test2() {
    let mut input = vec![5, 4, 3, 2, 1];
    Solution::next_permutation(&mut input);

    let expected = vec![1, 2, 3, 4, 5]; // Reset to the smallest permutation
    assert_eq!(input, expected);
}

#[test]
fn test3() {
    let mut input = vec![1, 2, 3, 4, 5];
    Solution::next_permutation(&mut input);

    let expected = vec![1, 2, 3, 5, 4]; // Next permutation
    assert_eq!(input, expected);
}

#[test]
fn test4() {
    let mut input = vec![1, 2, 2, 3];
    Solution::next_permutation(&mut input);

    let expected = vec![1, 2, 3, 2]; // Handles duplicates correctly
    assert_eq!(input, expected);
}

#[test]
fn test5() {
    let mut input = vec![1];
    Solution::next_permutation(&mut input);

    let expected = vec![1]; // No change since only one permutation exists
    assert_eq!(input, expected);
}

#[test]
fn test6() {
    let mut input = vec![2, 1];
    Solution::next_permutation(&mut input);

    let expected = vec![1, 2]; // Swap to the smallest permutation
    assert_eq!(input, expected);
}
#[test]
fn test7() {
    let mut input = vec![1, 3, 2];
    Solution::next_permutation(&mut input);

    let expected = vec![2, 1, 3]; // Swap to the smallest permutation
    assert_eq!(input, expected);
}
