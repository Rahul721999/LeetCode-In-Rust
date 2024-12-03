mod tests;
fn main() {
    let nums = vec![3, 2, 3];
    assert_eq!(Solution::majority_element(nums), vec![3]);
}
pub struct Solution;
impl Solution {
    pub fn majority_element(mut nums: Vec<i32>) -> Vec<i32> {
        nums.sort(); // Sort the array to group identical elements.
        let len = nums.len();
        let mut result = Vec::new(); // Store elements that appear more than len / 3 times.
        let mut i = 0;

        while i < len {
            let current_element = nums[i];
            let mut j = i;

            // Count occurrences of the current element.
            while j < len && nums[j] == current_element {
                j += 1;
            }

            // Check if the element occurs more than len / 3 times.
            if j - i > len / 3 {
                result.push(current_element); // Add to the result if the condition is met.
            }

            i = j; // Move to the next unique element.
        }

        result // Return the result containing all majority elements.
    }
}
