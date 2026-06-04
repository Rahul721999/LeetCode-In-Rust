mod tests;
fn main() {
    let nums = vec![1,7,3,6,5,6];
    let output = Solution::pivot_index(nums);
    assert_eq!(output, 3);
}
pub struct Solution;
impl Solution {
    pub fn pivot_index(nums: Vec<i32>) -> i32 {
        let len = nums.len();
        
        let mut prefix: Vec<i32> = vec![0;len];
        let mut suffix: Vec<i32> = vec![0;len];

        let mut left: i32 = 1; let mut right = len as i32 - 2;
        while left < len as i32 && right >= 0{
            let l = left as usize; let r = right as usize;
            prefix[l] = prefix[l - 1] + nums[l - 1];
            suffix[r] = suffix[r + 1] + nums[r + 1];
            left += 1;
            right -= 1;
        }

        for i in 0..len{
            if prefix[i] == suffix[i]{
                return i as i32
            }
        }

        -1
    }
}