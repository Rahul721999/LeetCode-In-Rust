#![allow(unused)]
use crate::*;

#[test]
fn test1() {
    let input = String::from("Tree");
    let output = Solution::frequency_sort(input);
    println!("{output}");
    // Expected output could be "eeTr" or "eetr" depending on implementation
    // because 'e' appears twice and 'T' and 'r' appear once.
    assert!(output == "eeTr" || output == "eetr");
}

#[test]
fn test2() {
    let input = String::from("cccaaa");
    let output = Solution::frequency_sort(input);
    println!("{output}");
    // Expected output: "cccaaa" or "aaaccc"
    // Both 'c' and 'a' have the same frequency, so their order may vary.
    assert!(output == "cccaaa" || output == "aaaccc");
}

#[test]
fn test3() {
    let input = String::from("Aabb");
    let output = Solution::frequency_sort(input);
    println!("{output}");
    // Expected output: "bbAa" or "bbaA"
    // 'b' appears twice, 'A' and 'a' appear once each.
    // Capitalization does not change order but depends on implementation.
    assert!(output == "bbAa" || output == "bbaA");
}
