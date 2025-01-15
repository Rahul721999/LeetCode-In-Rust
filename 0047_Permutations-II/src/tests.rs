#![allow(unused)]
use crate::*;

#[test]
fn test1() {
    let input = vec![1, 1, 2];
    let output = Solution::permute_unique(input);
    let expected = vec![vec![1, 1, 2], vec![1, 2, 1], vec![2, 1, 1]];

    expected.iter().for_each(|v| {
        if !output.contains(v) {
            panic!("Missing permutation: {v:?}")
        }
    });

    assert_eq!(output.len(), expected.len(), "Output length mismatch");
}

#[test]
fn test2() {
    let input = vec![1, 2, 3];
    let output = Solution::permute_unique(input);
    let expected = vec![
        vec![1, 2, 3],
        vec![1, 3, 2],
        vec![2, 1, 3],
        vec![2, 3, 1],
        vec![3, 1, 2],
        vec![3, 2, 1],
    ];

    expected.iter().for_each(|v| {
        if !output.contains(v) {
            panic!("Missing permutation: {v:?}")
        }
    });

    assert_eq!(output.len(), expected.len(), "Output length mismatch");
}

#[test]
fn test3() {
    let input = vec![1, 1, 1];
    let output = Solution::permute_unique(input);
    let expected = vec![vec![1, 1, 1]];

    expected.iter().for_each(|v| {
        if !output.contains(v) {
            panic!("Missing permutation: {v:?}")
        }
    });

    assert_eq!(output.len(), expected.len(), "Output length mismatch");
}
