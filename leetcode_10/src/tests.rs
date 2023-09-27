#[cfg(test)]
use crate::Solution;
#[test]
fn test1(){
     let res = Solution::regex("aa".to_string(), "a".to_string());
     assert_eq!(res, false);
}
#[test]
fn test2(){
     let res = Solution::regex("aa".to_string(), "a*".to_string());
     assert_eq!(res, true);
}
#[test]
fn test3(){
     let res = Solution::regex("ab".to_string(), ".*".to_string());
     assert_eq!(res, true);
}
#[test]
fn test4(){
     let res = Solution::regex("aab".to_string(), "c*a*b".to_string());
     assert_eq!(res, true);
}
#[test]
fn test5(){
     let res = Solution::regex("mississippi".to_string(), "mis*is*p*.".to_string());
     assert_eq!(res, false);
     let res = Solution::regex("mississippi".to_string(), "mis*is*ip*.".to_string());
     assert_eq!(res, true);
}
#[test]
fn test6(){
     let res = Solution::regex("".to_string(), "a*b*".to_string());
     assert_eq!(res, true);
}

#[test]
    fn test_regex() {
        assert_eq!(
            Solution::regex(String::from("abc"), String::from("abc")),
            true
        );
        assert_eq!(
            Solution::regex(String::from("abc"), String::from("a.c")),
            true
        );
        assert_eq!(
            Solution::regex(String::from("abc"), String::from("a.d")),
            false
        );
        assert_eq!(
            Solution::regex(String::from("abc"), String::from("c*a.c")),
            true
        );
     //    assert_eq!(
     //        Solution::regex(String::from("a"), String::from(".*..a*")),
     //        false
     //    );
        assert_eq!(
            Solution::regex(String::from("a"), String::from("")),
            false
        );
    }