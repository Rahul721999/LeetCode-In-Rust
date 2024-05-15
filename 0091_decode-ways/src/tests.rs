#![allow(unused)]
use crate::Solution;

#[test]
fn test1(){
    let res = Solution::num_decodings(String::from("06"));
    assert_eq!(res, 0)
}
#[test]
fn test2(){
    let res = Solution::num_decodings(String::from("122016"));
    assert_eq!(res, 4) 
}
#[test]
fn test3(){
    let res = Solution::num_decodings(String::from("11106"));
    assert_eq!(res, 2)
}
#[test]
fn test4(){
    let res = Solution::num_decodings(String::from("226"));
    assert_eq!(res, 3)
}
#[test]
fn test5(){
    let res = Solution::num_decodings(String::from("12"));
    assert_eq!(res, 2)
}