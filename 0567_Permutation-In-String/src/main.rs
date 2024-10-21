mod tests;
fn main() {
    let s1 = "abc".to_string();
    let s2 = "lecabee".to_string();
    assert!(Solution::check_inclusion(s1, s2));

    let s1 = "abc".to_string();
    let s2 = "lecaeeb".to_string();
    assert!(!Solution::check_inclusion(s1, s2));
}
pub struct Solution;
impl Solution {
    pub fn check_inclusion(s1: String, s2: String) -> bool {
        let s1_len = s1.len();
        let s2_len = s2.len();

        // return false if s1 is longer than s2
        if s1_len > s2_len {
            return false;
        }

        // initialize two frequency count array
        let mut count1 = vec![0; 26];
        let mut count2 = vec![0; 26];

        // convert string to byte_arr
        let s1 = s1.as_bytes();
        let s2 = s2.as_bytes();

        // fill count1 for the frequency of the characters in s1
        for ch in s1 {
            count1[(*ch - b'a') as usize] += 1;
        }

        // fill count2 with frequency of the char in the first window of s2 
        for ch in &s2[0..s1_len]{
            count2[(*ch - b'a') as usize] += 1;
        }

        for right in s1_len..s2_len{
            if count1 == count2{return true}

            // add the next char to the window
            count2[(s2[right] - b'a') as usize] += 1;

            // remove the first char from the window
            let left = right - s1_len;
            count2[(s2[left] - b'a') as usize] -= 1;
        }
        count1 == count2 // check the last window
    }
}
