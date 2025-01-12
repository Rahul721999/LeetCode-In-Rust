#![allow(unused)]
use crate::*;

#[test]
fn test1() {
    let input = vec![1, 2, 3];
    let output = Solution::permute(input);
    let expected = vec![
        vec![3, 2, 1],
        vec![2, 3, 1],
        vec![3, 1, 2],
        vec![2, 1, 3],
        vec![1, 3, 2],
        vec![1, 2, 3],
    ];
    expected.iter().for_each(|v| {
        if !output.contains(v) {
            panic!("Missing permutation: {v:?}")
        }
    });
}

#[test]
fn test2() {
    let input = vec![1];
    let output = Solution::permute(input);
    let expected = vec![vec![1]];
    expected.iter().for_each(|v| {
        if !output.contains(v) {
            panic!("Missing permutation: {v:?}")
        }
    });
}

#[test]
fn test3() {
    let input = vec![0,1];
    let output = Solution::permute(input);
    let expected = vec![vec![1,0], vec![0,1]];
    expected.iter().for_each(|v| {
        if !output.contains(v) {
            panic!("Missing permutation: {v:?}")
        }
    });
}
