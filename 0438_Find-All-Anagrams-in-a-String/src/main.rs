use std::vec;

mod tests;
fn main() {
    let input = "abab".to_string();
    let anagram = "ab".to_string();

    let output = Solution::find_anagrams(input, anagram);

    assert_eq!(output, vec![0,1,2]);
}
pub struct Solution;
impl Solution {
    pub fn find_anagrams(s: String, p: String) -> Vec<i32> {
        let len = p.len();
        // base case
        if len > s.len() {
            return Vec::new();
        }

        // result vec to store res
        let mut res: Vec<i32> = Vec::new();

        // converting String to vec<i32>
        fn string_to_vec(p: String) -> Vec<i32> {
            let mut v = vec![0; 26];
            for c in p.chars() {
                v[c as usize - 97] += 1;
            }
            v
        }
        let p = string_to_vec(p);


        let s: Vec<char> = s.chars().collect();
        let mut map = vec![0; 26];

        // Sliding window technique
        let mut l = 0_usize; // left pointer
        for (r, c) in s.iter().enumerate() {
            map[*c as usize - 97] += 1;

            if r - l + 1 == len {
                if map == p {
                    res.push(l as i32);
                }

                let first_char_index = s[l] as usize - 97;
                if map[first_char_index] > 0 {
                    map[first_char_index] -= 1;
                    l += 1;
                };
            }
        }
        res
    }
}

pub fn string_to_vec(s: String) -> Vec<i32> {
    let mut v = vec![0; 26];
    for c in s.chars() {
        v[c as usize - 97] += 1;
    }
    v
}
