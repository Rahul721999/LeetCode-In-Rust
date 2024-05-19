fn main() {
    assert_eq!(Solution::no_of_possible_subset(vec![2,3,5,6,8,10], 10), 3);
}
pub struct Solution;
impl Solution {
    fn no_of_possible_subset(nums: Vec<i32>, sum: i32) -> i32 {
        let len = nums.len();
        if sum == 0{return 1}
        let mut dp = vec![vec![0; sum as usize + 1]; len + 1];
        
        // initialize first row with initial value
        for i in 0..=sum as usize{
            dp[0][i] = 1;
        }
        
        fn helper(nums: &Vec<i32>, sum: usize, index: usize, dp: &mut Vec<Vec<i32>>) -> i32{
            if sum == 0 {return 1}
            if index == 0 { return 0}
            if dp[index][sum] != 0{return dp[index][sum]}
            let result = if nums[index - 1] <= sum as i32 {
                helper(nums, sum, index - 1, dp) + helper(nums, sum - nums[index - 1] as usize, index - 1, dp)
            } else {
                helper(nums, sum, index - 1, dp)
            };
            dp[index][sum] = result;
            result
        }

        helper(&nums, sum as usize, len, &mut dp)
    }
    
}
