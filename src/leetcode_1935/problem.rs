pub struct Solution;
impl Solution {
    pub fn can_be_typed_words(text: String, broken_letters: String) -> i32 {
        let broken: Vec<char> = broken_letters.chars().collect();
        let mut count = 0;
        for word in text.split_whitespace(){
            if !broken.iter().any(|&ch|{ word.contains(ch)}){
                count += 1;
            }
        }
        count
    } 
}
