use std::collections::HashMap;

mod tests;
fn main() {
    let input = vec![0, 1, 0];
    let output = Solution::find_max_length(input);
    assert_eq!(output, 2);
}
pub struct Solution;
impl Solution {
    pub fn find_max_length(nums: Vec<i32>) -> i32 {
        let mut max_len = 0; // Variable to store the maximum length of subarray found
        let mut curr_sum: i32 = 0; // Tracks the cumulative sum (treat 0 as -1, 1 as +1)

        // HashMap to store the first occurrence of each cumulative sum
        // Key: cumulative sum (difference between count of 1s and 0s)
        // Value: index where this cumulative sum first occurred
        let mut map: HashMap<i32, i32> = HashMap::new();
        map.insert(0, -1); // Initialize with 0: -1 to handle cases where subarray starts at index 0

        // Iterate through the array with index and value
        for (i, &num) in nums.iter().enumerate() {
            let i = i as i32; // Convert index to i32 for calculations

            // Update cumulative sum: add +1 for 1, subtract -1 for 0
            curr_sum += if num == 0 { -1 } else { 1 };

            // Check if the cumulative sum has been seen before
            if let Some(&prev_index) = map.get(&curr_sum) {
                // If seen, calculate the length of the subarray and update max_len
                max_len = max_len.max(i - prev_index);
            } else {
                // If not seen, store the first occurrence of this cumulative sum
                map.insert(curr_sum, i);
            }
        }

        max_len as i32 // Return the maximum length of the subarray
    }
}
