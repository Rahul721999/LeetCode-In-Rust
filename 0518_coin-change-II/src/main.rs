// maximum no of ways to make up the amount with change
// Input: amount = 5, coins = [1,2,5]
// Output: 4
// Explanation: there are four ways to make up the amount:
// 5=5
// 5=2+2+1
// 5=2+1+1+1
// 5=1+1+1+1+1

fn main() {
    assert_eq!(Solution::change(5, vec![1, 2, 5]), 4);
    assert_eq!(Solution::change(3, vec![2]), 0);
    assert_eq!(Solution::change(3, vec![3]), 1);
    assert_eq!(Solution::coin_exchange_optimized(5, vec![1, 2, 5]), 4);
    assert_eq!(Solution::coin_exchange_optimized(3, vec![2]), 0);
    assert_eq!(Solution::coin_exchange_optimized(3, vec![3]), 1);
}

pub struct Solution;
impl Solution {
    pub fn change(amount: i32, coins: Vec<i32>) -> i32 {
        let n = coins.len();
        if n == 0 || amount == 0 {
            return 1;
        }
        if n == 1 && (coins[0] == 0 || coins[n - 1] == amount) {
            return 1;
        }
        let mut dp = vec![vec![0; amount as usize + 1]; n + 1];

        // Initializing the first row
        for dp_row in dp.iter_mut().take(n + 1) {
            dp_row[0] = 1;
        }

        for i in 1..=n {
            for j in 0..=amount as usize {
                if coins[i - 1] <= j as i32 && dp[i][j] == 0 {
                    dp[i][j] = dp[i - 1][j] + dp[i][j - coins[i - 1] as usize];
                } else {
                    dp[i][j] = dp[i - 1][j];
                }
            }
        }
        dp[n][amount as usize]
    }
    pub fn coin_exchange_optimized(amount: i32, coins: Vec<i32>) -> i32 {
        if amount == 0 {
            return 1;
        }
        if coins.is_empty() {
            return 0;
        }
        let mut dp = vec![0; amount as usize + 1];
        dp[0] = 1;

        for &coin in &coins {
            for j in coin as usize..=amount as usize {
                if coin <= j as i32 {
                    dp[j] += dp[j - coin as usize]
                }
            }
        }

        dp[amount as usize]
    }
}
