use std::i32;

mod tests;
fn main() {
    let target = 7;
    let nums = vec![2, 3, 1, 2, 4, 3];
    let output = Solution::min_sub_array_len(target, nums);
    assert_eq!(output, 2);
}
pub struct Solution;
impl Solution {
    pub fn min_sub_array_len(target: i32, nums: Vec<i32>) -> i32 {
        let mut subarr_len = i32::MAX;
        let mut curr_sum = 0;
        let mut left = 0;

        for (right, _ele) in nums.iter().enumerate() {
            curr_sum += nums[right];
            if nums[right] == target {
                return 1;
            }
            while curr_sum >= target {
                subarr_len = subarr_len.min((right - left + 1) as i32);
                curr_sum -= nums[left];
                left += 1;
            }
        }
        // If min_length was updated, return it; otherwise, return 0
        if subarr_len == i32::MAX {
            0
        } else {
            subarr_len
        }
    }
}
