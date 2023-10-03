#[cfg(test)]

use crate::Solution;

#[test]
fn test1(){
     assert_eq!(Solution::group_anagrams(vec!["abc".to_string()]), vec![vec!["abc".to_string()]]);
}
#[test]
fn test2(){
     let res = Solution::group_anagrams(vec!["".to_string()]);
     let expected = vec![vec!["".to_string()]];
     assert_eq!(res, expected);
}