mod tests;
fn main() {
    let input = vec![-1,0,3,5,9,12];
    let output = Solution::search(input, 9);
    assert_eq!(output, 4);
}
pub struct Solution;
impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let mut left = 0 as i32;
        let mut right = nums.len() as i32;

        while left <= right{
            let pivot = left + (right-left) / 2;
            let mid_value = nums[pivot as usize];
            if target == mid_value{
                return pivot as i32;
            }
            if target > mid_value{
                left = pivot+1;
            }
            if target < mid_value{
                right = pivot - 1;
            }
        }

        -1
    }
}