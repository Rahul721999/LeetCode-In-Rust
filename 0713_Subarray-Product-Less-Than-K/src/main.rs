mod tests;
fn main() {
    let input = vec![10, 5, 2, 6];
    let output = Solution::num_subarray_product_less_than_k(input, 100);
    assert_eq!(output, 8);
    println!("{output}");
}
pub struct Solution;

impl Solution {
    pub fn num_subarray_product_less_than_k(nums: Vec<i32>, k: i32) -> i32 {
        if k <= 1{
            return 0;
        }
        let len = nums.len();
        let mut count = 0;
        let mut curr_product = 1;
        let mut left = 0;

        for right in 0..len {
            curr_product *= nums[right];

            while curr_product >= k {
                curr_product /= nums[left];
                left += 1;
            }

            count += right - left + 1;
        }
        return count as i32;
    }
}
