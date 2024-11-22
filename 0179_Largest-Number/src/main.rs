mod tests;
fn main() {
    let output = Solution::largest_number(vec![3,30,34,5,9]);
    assert_eq!(output, "9534330".to_string());
}

pub struct Solution;
impl Solution {
    pub fn largest_number(nums: Vec<i32>) -> String {
        // Convert the numbers to strings to allow for custom sorting based on concatenated values
        let mut nums: Vec<String> = nums.into_iter().map(|n| n.to_string()).collect();
        
        // Sort the strings based on their concatenated order.
        // For example, between "30" and "3", compare "303" and "330".
        // The string resulting in the larger number (e.g., "330") should come first.
        nums.sort_by(|a, b| (b.clone() + a).cmp(&(a.clone() + b)));
    
        // If the largest number is "0", the entire number is zero (e.g., input is [0, 0]).
        // Return "0" to avoid leading zeros in the result.
        if nums[0] == "0" {
            return "0".to_string();
        };
    
        // Concatenate all sorted strings to form the largest number.
        nums.concat()
    }
    
}