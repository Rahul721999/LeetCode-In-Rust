#![allow(unused)]
use crate::*;

#[test]
fn test1() {
    // Only vowels
    let s = "aaeei".to_string(); 
    // vowels: a=2, e=2, i=1 -> max=2
    // consonants: none -> max=0
    assert_eq!(Solution::max_freq_sum(s), 2);
}

#[test]
fn test2() {
    // Only consonants
    let s = "zzzzbcd".to_string(); 
    // vowels: none -> max=0
    // consonants: z=4, b=1, c=1, d=1 -> max=4
    assert_eq!(Solution::max_freq_sum(s), 4);
}

#[test]
fn test3() {
    // Mixed vowels and consonants
    let s = "programming".to_string();
    // vowels: o=1, a=1, i=1 -> max=1
    // consonants: r=2, g=2, m=2, p=1, n=1 -> max=2
    // total = 1 + 2 = 3
    assert_eq!(Solution::max_freq_sum(s), 3);
}
