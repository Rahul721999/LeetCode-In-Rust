mod tests;
fn main() {
    let input = vec![2, 2, 1, 1, 1, 2, 2];
    let output = Solution::majority_element(input);
    assert_eq!(output, 2);
}
pub struct Solution;
impl Solution {
    pub fn majority_element(mut nums: Vec<i32>) -> i32 {
        nums.sort(); // Sort the array to group identical elements.
        let len = nums.len();

        let mut max_count = 0; // Track the highest frequency found.
        let mut max_counted_ele = nums[0]; // Track the element with the highest frequency.
        let mut i = 0;

        while i < len {
            let ele = nums[i]; // Current element to check.
            let mut j = i;

            while j < len && nums[j] == ele {
                j += 1; // Count occurrences of the current element.
            }

            if j - i > len / 2 {
                max_count = max_count.max(j - i); // Update max count if needed.
                max_counted_ele = nums[i]; // Update the majority element.
            }

            i = j; // Move to the next unique element.
        }

        max_counted_ele // Return the majority element.
    }
}
