#[cfg(test)]
use crate::Solution;
#[test]
fn test1(){
     let res = Solution::is_match("aa".to_string(), "a".to_string());
     assert_eq!(res, false);
}
#[test]
fn test2(){
     let res = Solution::is_match("aa".to_string(), "a*".to_string());
     assert_eq!(res, true);
}
#[test]
fn test3(){
     let res = Solution::is_match("ab".to_string(), ".*".to_string());
     assert_eq!(res, true);
}
#[test]
fn test4(){
     let res = Solution::is_match("aab".to_string(), "c*a*b".to_string());
     assert_eq!(res, true);
}
#[test]
fn test5(){
     let res = Solution::is_match("mississippi".to_string(), "mis*is*p*.".to_string());
     assert_eq!(res, false);
     let res = Solution::is_match("mississippi".to_string(), "mis*is*ip*.".to_string());
     assert_eq!(res, true);
}
#[test]
fn test6(){
     let res = Solution::is_match("".to_string(), "a*b*".to_string());
     assert_eq!(res, true);
}

#[test]
    fn test_is_match() {
        assert_eq!(
            Solution::is_match(String::from("abc"), String::from("abc")),
            true
        );
        assert_eq!(
            Solution::is_match(String::from("abc"), String::from("a.c")),
            true
        );
        assert_eq!(
            Solution::is_match(String::from("abc"), String::from("a.d")),
            false
        );
        assert_eq!(
            Solution::is_match(String::from("abc"), String::from("c*a.c")),
            true
        );
     //    assert_eq!(
     //        Solution::is_match(String::from("a"), String::from(".*..a*")),
     //        false
     //    );
        assert_eq!(
            Solution::is_match(String::from("a"), String::from("")),
            false
        );
    }