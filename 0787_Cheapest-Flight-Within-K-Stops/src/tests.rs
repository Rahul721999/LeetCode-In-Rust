#![allow(unused)]
use crate::*;

#[test]
fn test1() {
    let flights = vec![
        vec![0, 1, 100],
        vec![1, 2, 100],
        vec![2, 0, 100],
        vec![1, 3, 600],
        vec![2, 3, 200],
    ];
    let output = Solution::find_cheapest_price(4, flights, 0, 3, 1);
    assert_eq!(700, output);
}

#[test]
fn test2() {
    let flights = vec![vec![0, 1, 100], vec![1, 2, 100], vec![0, 2, 500]];
    let output = Solution::find_cheapest_price(3, flights, 0, 2, 0);
    assert_eq!(500, output);
}

#[test]
fn test3() {
    let flights = vec![
        vec![0, 1, 5],
        vec![1, 2, 5],
        vec![0, 3, 2],
        vec![3, 1, 2],
        vec![1, 4, 1],
        vec![4, 2, 1],
    ];
    let output = Solution::find_cheapest_price(5, flights, 0, 2, 2);
    assert_eq!(7, output);
}
