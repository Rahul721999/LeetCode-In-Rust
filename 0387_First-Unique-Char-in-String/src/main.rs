use std::collections::HashMap;

mod tests;
fn main() {
    let output = Solution::first_uniq_char("loveleetcode".to_string());
    assert_eq!(output, 2);
}
pub struct Solution;
impl Solution {
    pub fn first_uniq_char(s: String) -> i32 {
        let mut map : HashMap<_, _>= HashMap::new();

        for ele in s.chars(){
            *map.entry(ele).or_insert(0) += 1;
        };

        // Find the first character with a frequency of 1
        for (i, c) in s.chars().enumerate(){
            if let Some(&count) = map.get(&c){
                if count == 1{
                    return i as i32
                }
            }
        }

        // if not unique value found
        -1
    }
}