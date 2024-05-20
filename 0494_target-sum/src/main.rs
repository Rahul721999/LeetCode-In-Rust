fn main() {
    assert_eq!(Solution::find_target_sum_ways(vec![1, 1, 1, 1, 1], 3), 5);
    assert_eq!(Solution::find_target_sum_ways(vec![1], 1), 1);
    assert_eq!(Solution::find_target_sum_ways(vec![1, 1, 1], 0), 0);
    assert_eq!(
        Solution::find_target_sum_ways(vec![0, 0, 0, 0, 0, 0, 0, 0, 1], 1),
        256
    );
}

// problem statement : subset1 - subset2 = target
// statement1: s1 - s2 = diff, where diff is the target
// statement2: s1 + s2 = sum ; (where sum: is addition of of array element)
// statement1 - statement2: 2*s2 = (diff - sum);
// hence, s2 = (diff - sum)/2;
// edge cases
pub struct Solution;
impl Solution {
    pub fn find_target_sum_ways(nums: Vec<i32>, target: i32) -> i32 {
        let sum: i32 = nums.iter().sum();
        if target > sum || (sum - target) % 2 == 1 {
            return 0;
        }
        let n = nums.len();
        let s2 = (sum - target) / 2;

        let mut dp = vec![vec![0; s2 as usize + 1]; n + 1];
        dp[0][0] =1;

        for i in 1..=n {
            for j in 0..=s2 as usize {
                if nums[i - 1] <= j as i32 {
                    dp[i][j] += dp[i - 1][j - nums[i - 1] as usize];
                }
                dp[i][j] += dp[i - 1][j];
            }
        }

        dp[n][s2 as usize]
    }
}
