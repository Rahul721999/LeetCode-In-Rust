#[allow(unused)]
use crate::Solution;
#[test]
fn test_generate_parenthesis_n_1() {
    // Simplest valid parentheses pair
    let expected = vec!["()"];
    assert_eq!(expected, Solution::generate_parenthesis(1));
}

#[test]
fn test_generate_parenthesis_n_2() {
    // n = 2, should generate the following combinations
    let mut expected = vec!["(())", "()()"];
    expected.sort(); // Sort to handle unordered results
    let mut result = Solution::generate_parenthesis(2);
    result.sort(); // Sort the result to compare
    assert_eq!(expected, result);
}

#[test]
fn test_generate_parenthesis_n_3() {
    // n = 3, multiple valid combinations
    let mut expected = vec!["((()))", "(()())", "(())()", "()(())", "()()()"];
    expected.sort(); // Sort to handle unordered results
    let mut result = Solution::generate_parenthesis(3);
    result.sort(); // Sort the result to compare
    assert_eq!(expected, result);
}

#[test]
fn test_generate_parenthesis_n_4() {
    // Larger case n = 4, should generate the following combinations
    let mut expected = vec![
        "(((())))", "((()()))", "((())())", "((()))()", "(()(()))", "(()()())", "(()())()",
        "(())(())", "(())()()", "()((()))", "()(()())", "()(())()", "()()(())", "()()()()",
    ];
    expected.sort(); // Sort to handle unordered results
    let mut result = Solution::generate_parenthesis(4);
    result.sort(); // Sort the result to compare
    assert_eq!(expected, result);
}
