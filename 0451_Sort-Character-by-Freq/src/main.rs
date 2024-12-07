use std::collections::HashMap;

mod tests;
fn main() {
    let input = String::from("Tree");
    let output = Solution::frequency_sort(input);
    println!("{output}");
}
pub struct Solution;
impl Solution {
    pub fn frequency_sort(s: String) -> String {
        // Step 1: Count the frequency of each character in the string
        let mut frequency_map: HashMap<char, i32> = HashMap::new();
        for ch in s.chars() {
            // Increment the count for each character in the hashmap
            *frequency_map.entry(ch).or_insert(0) += 1;
        }

        // Step 2: Collect the characters and their frequencies into a vector
        let mut frequency_list: Vec<(char, i32)> = frequency_map.into_iter().collect();

        // Step 3: Sort the vector by frequency in descending order
        frequency_list.sort_by(|a, b| b.1.cmp(&a.1));

        // Step 4: Build the result string by repeating each character based on its frequency
        let mut result = String::new();
        for (ch, freq) in frequency_list {
            result.extend(std::iter::repeat(ch).take(freq as usize));
        }

        result
    }
}
