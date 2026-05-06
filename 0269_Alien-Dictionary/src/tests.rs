#![allow(unused)]
use crate::*;

#[test]
fn test1() {
    // Standard valid ordering
    // w -> e -> r -> t -> f
    let words = vec![
        "wrt".to_string(),
        "wrf".to_string(),
        "er".to_string(),
        "ett".to_string(),
        "rftt".to_string(),
    ];
    let result = Solution::alien_order(words);
    // println!("{result}");
    assert_eq!(result, "wertf");
}

#[test]
fn test2() {
    // Invalid case: prefix conflict
    // "abc" comes before "ab" → impossible ordering
    let words = vec!["abc".to_string(), "ab".to_string()];
    let result = Solution::alien_order(words);
    assert_eq!(result, "");
}

#[test]
fn test3() {
    // Multiple valid orders possible (non-unique topo sort)
    // Constraints: z -> c, a -> b
    let words = vec!["za".to_string(), "zb".to_string(), "ca".to_string(), "cb".to_string()];
    let result = Solution::alien_order(words);

    // Accept either valid ordering
    let valid1 = "zacb";
    let valid2 = "zabc";
    assert!(result == valid1 || result == valid2);
}
