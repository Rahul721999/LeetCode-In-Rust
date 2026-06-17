mod tests;
fn main() {
    let nums = vec![1,1,1,1,1,1,1,1,1,1,1,1,1,2,1,1,1,1,1];
    let output = Solution::search(nums, 2);
    println!("{output}");
    assert!(output);
}
pub struct Solution;

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> bool {
        let len = nums.len();
        let mut left = 0;
        let mut right = len as i32 - 1;


        while left <= right {
            let mid = left + (right - left) / 2;
            let mid_val = nums[mid as usize];

            // if mid is the target
            if mid_val == target {return true}

            // is it preventing us to determine which half is sorted ?
            // left-val == mid-val == right-val ?
            if nums[left as usize] == mid_val && mid_val == nums[right as usize]{
                left += 1;
                right -= 1;
            }

            // left is sorted
            else if mid_val >= nums[left as usize]{
                if nums[left as usize] <= target && target < mid_val{ // check the range
                    right = mid - 1;
                }else{
                    left = mid+1;
                }
            }
            // right side is sorted
            else {    
                if mid_val < target && target <= nums[right as usize]{ // check the range
                    left = mid + 1;
                }else{
                    right = mid - 1;
                }
            }
        }
        false
    }
}