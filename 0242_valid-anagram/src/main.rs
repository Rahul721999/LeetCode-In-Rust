mod tests;
fn main() {
    assert_eq!(
        Solution::is_anagram("anagram".to_string(), "nagaram".to_string()),
        true
    );
    assert_eq!(
        Solution::is_anagram("car".to_string(), "rat".to_string()),
        false
    );
}
struct Solution;
impl Solution {
    #[allow(unused)]
    pub fn is_anagram_unoptimized(s: String, t: String) -> bool {
        if s.len() != t.len() {
            return false;
        }
        use std::collections::HashMap;
        let mut char_map: HashMap<char, i32> = HashMap::new();
        for (sc, tc) in s.chars().zip(t.chars()) {
            char_map.entry(sc).and_modify(|c| *c += 1).or_insert(1);
            char_map.entry(tc).and_modify(|c| *c -= 1).or_insert(-1);
        }

        char_map.into_iter().all(|(_k, v)| v == 0)
    }

    pub fn is_anagram(s: String, t: String) -> bool {
        let s_len = s.len();
        let t_len = t.len();

        if s_len != t_len {
            return false;
        }

        let mut s_vec = vec![0; 26];
        let mut t_vec = vec![0; 26];

        for ele in s.chars() {
            s_vec[ele as usize - 97] += 1;
        }

        for ele in t.chars() {
            t_vec[ele as usize - 97] += 1;
        }

        for ele in s.chars() {
            let i = ele as usize - 97;
            if s_vec[i] != t_vec[i] {
                return false;
            }
        }
        true
    }
}
