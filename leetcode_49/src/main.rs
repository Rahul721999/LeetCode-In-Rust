use std::collections::HashMap;
mod tests;

fn main() {
    let res = Solution::group_anagrams(vec!["".to_string()]);
    let expected = vec![vec!["".to_string()]];
    assert_eq!(res, expected);
}


struct Solution;
impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        // create a hashmap to store the res
        // The 'key' in the hashMap will be a fixed sized arr with 26 bytes representing the no of each char in the ip string. 
        // The 'value' in the hashMap will be the list of strings that are anagrams of each other.
        let mut res = HashMap::<[u8; 26], Vec<String>>::new();
        
        // early return if the len is 1
        if strs.len() == 1{return vec![vec![strs[0].clone()]]}

        // else traverse through the vec and check if it's anagram
        for s in strs{
            // create an arr with len 26
            let mut char_count: [u8; 26] = [0; 26];
            for c in s.chars(){
                // char index = ASCII val of the char - ASCII val of 'a'
                // then increment the char count in the arr
                char_count[c as usize - 'a' as usize] += 1;
            }
            // Insert the curr string into the corresponding anagram group in the hashMap 
            // if group(key) doesn't exist then create a new empty vec as a value
            // and then push the curr string into the vector
            res.entry(char_count).or_insert(Vec::new()).push(s);
        }
        // convert the 'values'(not the 'keys') of hashMap to a vector and return 
        res.into_values().collect()
    }
}
