#![allow(unused)]
use crate::*;

#[test]
fn test1() {
    let intput = vec![
        "test.email+alex@leetcode.com".to_string(),
        "test.e.mail+bob.cathy@leetcode.com".to_string(),
        "testemail+david@lee.tcode.com".to_string(),
    ];
    let output = Solution::num_unique_emails(intput);
    assert_eq!(output, 2);
}

#[test]
fn test2() {
    let intput = vec![
        "test.email.leet+alex@code.com".to_string(),
        "test.email+alex@leetcode.com".to_string(),
    ];
    let output = Solution::num_unique_emails(intput);
    assert_eq!(output, 2);
}

#[test]
fn test3() {
    let intput = vec![
        "a@leetcode.com".to_string(),
        "b@leetcode.com".to_string(),
        "c@leetcode.com".to_string(),
    ];
    let output = Solution::num_unique_emails(intput);
    assert_eq!(output, 3);
}
