mod tests;
fn main() {
    let output = Solution::character_replacement("abaa".to_string(), 0);
    assert_eq!(output, 2);
}

pub struct Solution;
impl Solution {
    pub fn character_replacement(s: String, k: i32) -> i32 {
        use std::collections::HashMap;

        let mut res = 0;
        let (mut start, mut end) = (0_usize, 0_usize);
        let mut map: HashMap<char, i32> = HashMap::new();

        let bytes = s.as_bytes(); // Use bytes to access the characters more efficiently

        while end < s.len() {
            let char = bytes[end] as char;
            *map.entry(char).or_insert(0) += 1;

            let max_val = map.values().max().cloned().unwrap_or(0); // Get the highest frequency of any character

            let curr_window_size = (end - start + 1) as i32;

            if curr_window_size - max_val <= k {
                res = res.max(curr_window_size);
            } else {
                let start_char = bytes[start] as char;
                if let Some(val) = map.get_mut(&start_char) {
                    *val -= 1;
                }
                start += 1;
            }

            end += 1;
        }
        res
    }
}
