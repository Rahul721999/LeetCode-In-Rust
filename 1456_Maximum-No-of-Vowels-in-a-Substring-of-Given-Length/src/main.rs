mod tests;
fn main() {
    let intput = "abciiidef".to_string();
    let output = Solution::max_vowels(intput, 3);
    assert_eq!(output, 3);
}
pub struct Solution;
impl Solution {
    pub fn max_vowels(s: String, k: i32) -> i32 {
        let mut l = 0;
        let mut max_vowel_count = 0;
        let mut curr_count = 0;
        let s = s.as_bytes();

        for (r, _ele) in s.iter().enumerate() {
            if Self::is_vowel(&s[r]){
                curr_count += 1;
            }
            max_vowel_count = max_vowel_count.max(curr_count);
            if r - l + 1 >= k as usize {
                if Self::is_vowel(&s[l]){
                    curr_count -= 1;
                };
                l += 1;
            }
        }
        max_vowel_count
    }
    fn is_vowel(ch: &u8) -> bool{
        match ch {
            b'a' | b'e' | b'i' | b'o' | b'u' => {
                true
            }
            _ => false,
        }
    }
}
