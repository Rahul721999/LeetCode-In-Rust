fn main() {
    assert_eq!(Solution::coin_change(vec![1, 2, 5], 11), 3);
    assert_eq!(Solution::coin_change(vec![2], 3), -1);
    assert_eq!(Solution::coin_change(vec![1], 0), 0);
    assert_eq!(Solution::coin_change(vec![1, 2, 5], 5), 1);
    
    assert_eq!(Solution::coin_change_optimized(vec![1, 2, 5], 11), 3);
    assert_eq!(Solution::coin_change_optimized(vec![2], 3), -1);
    assert_eq!(Solution::coin_change_optimized(vec![1], 0), 0);
    assert_eq!(Solution::coin_change_optimized(vec![1, 2, 5], 5), 1);
}
pub struct Solution;
impl Solution {
    // min no of coin needed to change the amount.
    pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
        if amount == 0 {
            return 0;
        }
        let len = coins.len();
        let amount = amount as usize;
        let mut dp = vec![vec![std::i32::MAX; amount + 1]; len + 1];
        dp[0][0] = 0;
        for i in 1..=len {
            for j in 0..=amount {
                let result = if coins[i - 1] <= j as i32 {
                    let with_current_coin = dp[i][j - coins[i - 1] as usize].saturating_add(1);
                    let without_current_coin = dp[i - 1][j];
                    with_current_coin.min(without_current_coin)
                } else {
                    dp[i - 1][j]
                };
                dp[i][j] = result;
            }
        }
        if dp[len][amount] == std::i32::MAX {
            -1
        } else {
            dp[len][amount]
        }
    }

    pub fn coin_change_optimized(coins: Vec<i32>, amount: i32) -> i32 {
        if amount == 0 {
            return 0;
        }
        let amount = amount as usize;
        let mut dp = vec![std::i32::MAX; amount + 1];
        dp[0] = 0;

        for i in 1..=amount {
            for &coin in &coins {
                let coin = coin as usize;
                if coin <= i && dp[i - coin] != std::i32::MAX {
                    dp[i] = dp[i].min(dp[i - coin] + 1)
                }
            }
        }
        if dp[amount] == std::i32::MAX {
            -1
        } else {
            dp[amount]
        }
    }
}

// [1, 2, 5]
// 0 1 2 3 4 5 6 7 8 9 10 11
// 0 1 1 2 2 1 2 2 3 3 2  3
