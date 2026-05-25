#![allow(unused)]
use crate::*;

#[test]
fn test1() {
    let tickets = vec![
        vec!["BUF".to_string(), "HOU".to_string()],
        vec!["HOU".to_string(), "SEA".to_string()],
        vec!["JFK".to_string(), "BUF".to_string()],
    ];

    let output = Solution::find_itinerary(tickets);
    let expected = vec![
        "JFK".to_string(),
        "BUF".to_string(),
        "HOU".to_string(),
        "SEA".to_string(),
    ];

    assert_eq!(output, expected);
}

#[test]
fn test2() {
    let tickets = vec![
        vec!["HOU".to_string(), "JFK".to_string()],
        vec!["SEA".to_string(), "JFK".to_string()],
        vec!["JFK".to_string(), "SEA".to_string()],
        vec!["JFK".to_string(), "HOU".to_string()],
    ];

    let output = Solution::find_itinerary(tickets);
    let expected = vec![
        "JFK".to_string(),
        "HOU".to_string(),
        "JFK".to_string(),
        "SEA".to_string(),
        "JFK".to_string(),
    ];

    assert_eq!(output, expected);
}

#[test]
fn test3() {
    // Add test here
}
