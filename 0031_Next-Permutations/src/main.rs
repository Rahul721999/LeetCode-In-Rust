mod tests;
fn main() {
    let mut input = vec![1, 3, 5, 4, 2];
    Solution::next_permutation(&mut input);

    let expected = vec![1, 4, 2, 3, 5];
    assert_eq!(input, expected);
}
pub struct Solution;
impl Solution {
    /// Updates the given vector `nums` to the next lexicographical permutation in-place.
    /// If no such permutation exists (the array is in descending order), it rearranges
    /// the vector to the smallest permutation (sorted in ascending order).
    pub fn next_permutation(nums: &mut Vec<i32>) {
        let len = nums.len();
        if len < 2 {
            return; // No permutations possible for arrays of size 0 or 1
        }
        let mut pivot_index = len - 1;

        // Step 1: Find the first decreasing element from the right
        // This is the pivot, where nums[pivot_index - 1] < nums[pivot_index]
        while pivot_index > 0 {
            if nums[pivot_index - 1] < nums[pivot_index] {
                // pivot_index -= 1;
                break;
            }
            pivot_index -= 1;
        }

        // Step 2: Handle the case where the array is in descending order
        // If no pivot is found (pivot_index == 0), reverse the array to get the smallest permutation
        if pivot_index == 0 {
            nums.reverse();
            return;
        }

        // Step 3: Find the smallest element in the suffix (nums[pivot_index..]) that is larger than nums[pivot]
        let pivot = pivot_index - 1;
        let mut swap_index = len - 1;
        for i in (pivot_index..len).rev() {
            if nums[i] > nums[pivot] {
                swap_index = i;
                break;
            }
        }

        // Step 4: Swap nums[pivot] and nums[swap_index]
        nums.swap(pivot, swap_index);

        // Step 5: Reverse the suffix to get the next smallest lexicographical order
        nums[pivot_index..].reverse();
    }
}
