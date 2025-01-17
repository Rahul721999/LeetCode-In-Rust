mod tests;
fn main() {
    let text1 = String::from("abcde");
    let text2 = String::from("ace");
    let expected_result = 3;
    let result = Solution::longest_common_subsequence(text1, text2);
    assert_eq!(result, expected_result);
}
pub struct Solution;
impl Solution {
    pub fn longest_common_subsequence(text1: String, text2: String) -> i32 {
        let a = text1.as_bytes();
        let b = text2.as_bytes();

        let mut dp = vec![0; b.len() + 1];

        for i in 0..a.len() {
            let mut prev;
            let mut curr = 0;
            for j in 0..b.len() {
                prev = curr;
                curr = dp[j + 1];
                dp[j + 1] = if a[i] == b[j] {
                    prev + 1
                } else {
                    i32::max(dp[j], dp[j + 1])
                };
            }
        }

        return dp[b.len()];
    }
}
