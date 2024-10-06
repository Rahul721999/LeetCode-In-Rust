#![allow(unused)]
use crate::*;

#[test]
fn test1() {
    let input = vec![-10, -3, 0, 5, 9];
    let expected_output = create_b_tree(
        vec![Some(0), Some(-3), Some(9), Some(-10), None, Some(5)],
        0,
        6,
    );
    let acceptable_output = create_b_tree(
        vec![Some(0),Some(-10),Some(5),None,Some(-3),None,Some(9)],
         0, 7);
    let res = Solution::sorted_array_to_bst(input);
    
    if res != expected_output{
        assert_eq!(res, acceptable_output);
    }else{
        assert_eq!(res, expected_output);
    }
}

#[test]
fn test2() {
    let input = vec![1,3];
    let expected_output = create_b_tree(
        vec![Some(3), Some(1)],
        0,
        2,
    );
    let acceptable_output = create_b_tree(
        vec![Some(1), None, Some(3)],
         0, 3);
    let res = Solution::sorted_array_to_bst(input);
    
    if res != expected_output{
        assert_eq!(res, acceptable_output);
    }else{
        assert_eq!(res, expected_output);
    }
}

#[test]
fn test3() {
    // Add test here
}
