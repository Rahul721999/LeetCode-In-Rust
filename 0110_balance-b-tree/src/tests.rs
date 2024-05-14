#![allow(unused)]
use crate::{Solution, create_b_tree};

#[test]
fn test_with_empty_root(){
    let input_vec = vec![];
    let input_tree = create_b_tree(&input_vec, 0, input_vec.len());
    assert!(Solution::is_balanced(input_tree));
}
#[test]
fn test_with_balanced_tree(){
    let input_vec = vec![3,9,20,0,0,15,7];
    let input_tree = create_b_tree(&input_vec,  input_vec.len(), 0);
    assert!(Solution::is_balanced(input_tree));
}
#[test]
fn test_with_unbalanced_tree(){
    let input_vec = vec![1,2,2,3,3,0,0,4,4];
    let input_tree = create_b_tree(&input_vec, input_vec.len(), 0);
    assert!(!Solution::is_balanced(input_tree));
}