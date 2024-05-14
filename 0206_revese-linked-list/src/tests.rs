#[allow(unused)]
use crate::{helper, Solution};

#[test]
fn test_reverse_empty_list() {
    let test_case = None;
    let expected_result = None;
    assert_eq!(Solution::reverse_list(test_case), expected_result);
}


#[test]
fn test_reverse_list1(){
    let test_case = helper(vec![1,2,3,4,5]);
    let expected_result = helper(vec![5,4,3,2,1]);
    assert_eq!(Solution::reverse_list(test_case), expected_result)
}
#[test]
fn test_reverse_list2(){
    let test_case = helper(vec![1,2]);
    let expected_result = helper(vec![2,1]);
    assert_eq!(Solution::reverse_list(test_case), expected_result)
}
#[test]
fn test_reverse_list3(){
    let test_case = helper(vec![]);
    let expected_result = helper(vec![]);
    assert_eq!(Solution::reverse_list(test_case), expected_result)
}
#[test]
fn test_reverse_list4(){
    let test_case = helper(vec![1]);
    let expected_result = helper(vec![1]);
    assert_eq!(Solution::reverse_list(test_case), expected_result)
}