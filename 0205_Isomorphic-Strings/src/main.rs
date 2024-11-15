mod tests;
fn main() {
    let s = "egg".to_string();
    let t = "add".to_string();
    let output = Solution::is_isomorphic(s, t);
    assert!(output);

    let s = "foo".to_string();
    let t = "bar".to_string();
    let output = Solution::is_isomorphic(s, t);
    assert!(!output);
}

pub struct Solution;
impl Solution {
    pub fn is_isomorphic(s: String, t: String) -> bool {
        use std::collections::HashMap;
        if s.len() != t.len() {
            return false;
        }
        let mut map1: HashMap<char, char> = HashMap::new();
        let mut map2: HashMap<char, char> = HashMap::new();

        for i in 0..s.len() {
            let ch1 = s.chars().nth(i).unwrap();
            let ch2 = t.chars().nth(i).unwrap();

            if let Some(c) = map1.insert(ch1, ch2) {
                if c != ch2 {
                    return false;
                }
            };
            if let Some(c) = map2.insert(ch2, ch1) {
                if c != ch1 {
                    return false;
                }
            };
        }
        true
    }
}
