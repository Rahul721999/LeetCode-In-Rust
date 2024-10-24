mod tests;
fn main() {
    let intput = vec![3, 2, 20, 1, 1, 3];
    let output = Solution::min_operations(intput, 10);
    assert_eq!(output, 5);
}
pub struct Solution;
impl Solution {
    /* Explanation: We need to remove elements from the front and back of the array in order to make the sum equal to `x`.

        let sum = arr.iter().sum(); // Calculate the sum of all elements in the array.

        ** Next, we define:
            y (the leftover middle portion) = sum - x;
        ** To find the minimum number of operations required,
            we need to maximize the length of the subarray `y` (middle portion) such that its sum equals `sum - x`.
            The number of operations will be = the total length of the array - the length of `y`.


        **** So the problem converted to: Find the Longest-Sub-Arr, whose sum of the element is (sum - x);

        https://youtu.be/w7u9sMlx7zc?si=jI1rcfuUwTY9eCHa&t=355
    */

    pub fn min_operations(nums: Vec<i32>, x: i32) -> i32 {
        let sum: i32 = nums.iter().sum();
        let len = nums.len();
        let target = sum - x;
        let mut max_window = 0;
        let mut curr_sum = 0;
        let mut left = 0;

        // If the target is negative, it's impossible to get that sum
        if target < 0 {
            return -1;
        }

        for right in 0..len {
            curr_sum += nums[right];

            while left <= right && curr_sum > target {
                curr_sum -= nums[left];
                left += 1;
            }
            
            if curr_sum == target {
                max_window = max_window.max(right - left + 1);
            }
        }

        if max_window == 0 && target != 0 {
            return -1;
        } else {
            return (len - max_window) as i32;
        }
    }
}
