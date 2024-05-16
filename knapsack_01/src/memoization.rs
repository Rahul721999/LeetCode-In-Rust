use crate::Solution;

impl Solution {
    pub fn knapsack_recursive_dp(
        n: usize,       // no of item
        w: usize,       // max weight the bag can hold
        profit: &[i32], // max profit on each item
        weight: &[i32], // weight of the each given item
    ) -> i32 {
        if n == 0 || w == 0 {
            return 0;
        }

        let height = n + 1;
        let width = w + 1;
        let mut dp = vec![vec![-1; width]; height];

        fn rec_dp(
            n: usize,
            w: usize,
            profit: &[i32],
            weight: &[i32],
            dp: &mut Vec<Vec<i32>>,
        ) -> i32 {
            if n == 0 || w == 0 {
                return 0;
            }
            if dp[n][w] != -1 {
                return dp[n][w];
            }

            use std::cmp;
            let index = n - 1;
            if weight[index] <= w as i32 {
                dp[n][w] = cmp::max(
                    profit[index] + rec_dp(n - 1, w - weight[index] as usize, profit, weight, dp),
                    rec_dp(n - 1, w, profit, weight, dp),
                );
                dp[n][w]
            } else {
                dp[n][w] = rec_dp(n - 1, w, profit, weight, dp);
                dp[n][w]
            }
        }
        rec_dp(n, w, profit, weight, &mut dp)
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;
    #[test]
    fn test_knapsack_recursive_dp1() {
        let res = Solution::knapsack_recursive_dp(3, 4, &[1, 2, 3], &[4, 5, 1]);
        assert_eq!(res, 3);
    }
    #[test]
    fn test_knapsack_recursive_dp2() {
        let res = Solution::knapsack_recursive_dp(3, 50, &[60,100,120], &[10,20,30]);
        assert_eq!(res, 220);
    }
}
