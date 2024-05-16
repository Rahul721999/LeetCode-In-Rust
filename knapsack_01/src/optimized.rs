use crate::Solution;

// space optimized knapsack solution. using 1D array
impl Solution {
    pub fn optimized_dp(
        n: i32,          // no of items
        max_weight: i32, // max weight the bag can hold
        profit: &[i32],  // max profit on each item
        weight: &[i32],  // weight of each given item
    ) -> i32 {
        if n == 0 || max_weight == 0 {
            return 0;
        }

        let mut dp = vec![0; (max_weight + 1) as usize];

        for i in 0..n as usize {
            // println!("i= {}: dp{:?}", i, dp);
            for j in (0..=max_weight as usize).rev() {
                // dbg!(j);
                if weight[i] as usize <= j {
                    // dbg!(dp[j],weight[i],profit[i],j - weight[i] as usize,dp[j - weight[i] as usize] + profit[i]);
                    dp[j] = dp[j].max(dp[j - weight[i] as usize] + profit[i]);
                }
            }
            
        }

        println!("Final DP Array: {:?}", dp);

        dp[max_weight as usize]
    }
}
