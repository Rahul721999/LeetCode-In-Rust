mod tests;
fn main() {
    let nums = vec![4,5,6,7,0,1,2];
    let output = Solution::search(nums, 0);
    println!("{output}");
    assert_eq!(output, 4);
}
pub struct Solution;
impl Solution {
    /*
        Find the min value indx first,
        then do another BS based on the target
    */
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let len = nums.len();

        let mut left = 0 as i32;
        let mut right = len as i32 - 1 ;

        while left < right {
            let mid = left + (right - left)/ 2;
            let mid_val = nums[mid as usize];

            // searching min value
            if mid_val > nums[right as usize]{
                left = mid + 1;
            }else{
                right = mid;
            }
        };

        let min_index = left;
        let min_value = nums[min_index as usize];
        if min_value == target{ return min_index}

        (left, right) = if target >= min_value && target < nums[len - 1]{
            (0, min_index)
        }else{
            (min_index, len as i32 - 1)
        };

        while left <= right {
            let mid = left + (right - left)/ 2;
            let mid_value = nums[mid as usize];

            if mid_value > target{
                left = mid + 1;
            }else if mid_value < target{
                right = mid - 1;
            }else{
                return mid
            }
        }
        - 1
    }
}