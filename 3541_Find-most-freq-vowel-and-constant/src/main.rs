mod tests;
fn main() {
    let s = "aeiaeia".to_string();
    assert_eq!(Solution::max_freq_sum(s), 3);

    let s = "successes".to_string();
    assert_eq!(Solution::max_freq_sum(s), 6);
}

use std::collections::HashMap;
pub struct Solution;
impl Solution {
    pub fn max_freq_sum(s: String) -> i32 {
        if s.is_empty() {
            return 0;
        }
        let mut vowels = HashMap::new();
        let mut consonant_count = HashMap::new();

        for ch in s.chars(){
            if "aeiou".contains(ch){
                *vowels.entry(ch).or_insert(0) += 1;
            }else{
                *consonant_count.entry(ch).or_insert(0) += 1;
            }
        };

        let max_vowel = vowels.values().max().unwrap_or(&0);
        let max_consonant = consonant_count.values().max().unwrap_or(&0);

        *max_vowel + *max_consonant
    }
}
