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
        let (m, n) = (text1.len(), text2.len());
        let s1: Vec<char> = text1.chars().collect();
        let s2: Vec<char> = text2.chars().collect();

        let mut dp = vec![vec![0; n + 1]; m + 1];

        fn lcs(s1: &Vec<char>, s2: &Vec<char>, m: usize, n: usize, dp: &mut Vec<Vec<i32>>) -> i32 {
            if m == 0 || n == 0 {
                return 0;
            }
            if dp[m][n] != 0 {
                return dp[m][n];
            }

            if s1[m - 1] == s2[n - 1] {
                dp[m][n] = 1 + lcs(s1, s2, m - 1, n - 1, dp)
            } else {
                dp[m][n] = std::cmp::max(
                    lcs(s1, s2, m - 1, n, dp), // excluding last element of s1
                    lcs(s1, s2, m, n - 1, dp), // excluding last element of s2
                );
            }
            dp[m][n]
        }
        lcs(&s1, &s2, m, n, &mut dp)
    }
}
