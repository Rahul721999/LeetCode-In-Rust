fn main() {
    assert_eq!(Solution::is_anagram("anagram".to_string(), "nagaram".to_string()), true);
    assert_eq!(Solution::is_anagram("car".to_string(), "rat".to_string()), false);
}
struct Solution;
impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        if s.len() != t.len() {return  false;}
        use std::collections::HashMap;
        let mut char_map: HashMap<char, i32> = HashMap::new();
        for (sc, tc) in s.chars().zip(t.chars()){
            char_map.entry(sc).and_modify(|c| *c +=1).or_insert(1);
            char_map.entry(tc).and_modify(|c| *c -=1).or_insert(-1);
        }
        
        char_map.into_iter().all(|(_k,v)| v == 0)
    }
}
