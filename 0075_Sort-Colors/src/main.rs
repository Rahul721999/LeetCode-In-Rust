mod tests;
fn main() {
    let mut intput = vec![2,0,2,1,1,0];
    Solution::sort_colors(&mut intput);
    assert_eq!(intput, vec![0,0,1,1,2,2]);
}
pub struct Solution;
impl Solution {
    // Solution: Insertion Sort (an in-place sorting algorithm)
    pub fn sort_colors(nums: &mut Vec<i32>) {
        if nums.is_empty() {
            return;
        } // Handle edge case: empty input

        // Start iterating from index 1, as the first element (index 0) is considered already sorted
        for i in 1..nums.len() {
            let curr = nums[i]; // Current element to be placed in the correct position
            let mut j = i; // Start from the current index and move backward

            // Shift elements to the right until the correct position for 'curr' is found
            while j > 0 && curr < nums[j - 1] {
                // While 'curr' is smaller than the previous element
                nums[j] = nums[j - 1]; // Shift the previous element one position to the right
                j -= 1; // Move to the previous position
            }

            // Insert 'curr' at its correct position
            nums[j] = curr;
        }
    }
}
