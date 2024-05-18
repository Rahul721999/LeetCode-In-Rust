fn main() {
    assert!(Solution::can_partition(vec![1, 5, 11, 5]));
    assert!(!Solution::can_partition(vec![1, 2, 3, 5]));
}

pub struct Solution;
impl Solution {
    pub fn can_partition(nums: Vec<i32>) -> bool {
        let sum: i32 = nums.iter().sum();
        if sum % 2 != 0 {
            return false;
        }
        let target = (sum / 2) as usize;
        let mut dp = vec![false; target + 1];
        dp[0] = true;

        for num in nums {
            let num = num as usize;
            if num > target {
                continue;
            }
            for j in (num..=target).rev() {
                if dp[j - num] {
                    dp[j] = true;
                }
                if dp[target] {
                    return true; // Early exit if target is reached
                }
            }
        }
        dp[target]
    }
}
