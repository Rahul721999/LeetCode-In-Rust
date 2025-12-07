#![allow(unused)]
use crate::*;

#[test]
fn test1() {
    let input = vec![1, 2, 2];
    let output = Solution::subsets_with_dup_recursive(input);
    let expected = vec![
        vec![],
        vec![1],
        vec![1, 2],
        vec![1, 2, 2],
        vec![2],
        vec![2, 2],
    ];
    expected.iter().for_each(|s|{
        if !output.contains(&s){
            panic!("output doesn't contains: {:?}", s);
        }
    })
}

#[test]
fn test2() {
    let input = vec![1, 2, 3];
    let output = Solution::subsets(input);
    let expected = vec![
        vec![],
        vec![1],
        vec![2],
        vec![1, 2],
        vec![3],
        vec![1, 3],
        vec![2, 3],
        vec![1, 2, 3],
    ];
    assert_eq!(output.len(), expected.len());
    expected.iter().for_each(|s|{
        if !output.contains(&s){
            panic!("output doesn't contains: {:?}", s);
        }
    })
}

#[test]
fn test3() {
    let input = vec![0];
    let output = Solution::subsets(input);
    let expected = vec![vec![], vec![0]];
    assert_eq!(output.len(), expected.len());
    expected.iter().for_each(|s|{
        if !output.contains(&s){
            panic!("output doesn't contains: {:?}", s);
        }
    })
}
