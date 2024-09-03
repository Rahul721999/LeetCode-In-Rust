#![allow(unused)]
use std::vec;

mod tests;

fn main() {
    let res = Solution::is_match("aa".to_string(), "a*".to_string());
    assert!(res);
    println!("{}", res);
}

struct Solution;
impl Solution {
    ///
    ///  recursive solution...(Very slow)
    ///
    pub fn is_match(s: String, p: String) -> bool {
        let (i, j) = (s.len(), p.len());
        // base case

        /* when the both the given string is empty */
        if i == 0 {
            return true;
        };

        /* when pattern string is empty but the i/p is not */
        if i > 0 && j == 0 {
            return false;
        };

        /* match s[0] == p[0] || p[1] == '.' */
        let first_match = i > 0 && (s.char_at(0) == p.char_at(0) || p.char_at(0) == Some('.'));

        /* check if the next char is '*' */
        if j >= 2 && p.char_at(1) == Some('*') {
            // Two options: 0 occurrences || 1 or more occurrences..
            Solution::is_match(s[..].to_string(), p[2..].to_string())
                || first_match && Solution::is_match(s[1..].to_string(), p)
        } else {
            first_match && Solution::is_match(s[1..].to_string(), p[1..].to_string())
        }
    }

    ///
    /// Dynamic Programming solution with memoization...
    ///
    pub fn regex(s: String, p: String) -> bool {
        let (m, n) = (s.len(), p.len());

        let s_str: Vec<char> = vec!['?'].into_iter().chain(s.chars()).collect();
        let p_str: Vec<char> = vec!['?'].into_iter().chain(p.chars()).collect();

        // create a matrix to store the results
        let mut dp = vec![vec![false; n + 1]; m + 1];
        dp[0][0] = true;

        for i in 0..=m {
            for j in 1..=n {
                if i == 0 && p_str[j] == '*' {
                    dp[i][j] = dp[i][j - 2];
                }
                if i == 0 {
                    continue;
                }

                // if s[i] ==p[j] || '.'
                if s_str[i] == p_str[j] || p_str[j] == '.' {
                    // if s[0..i-1] matches with p[0..j-1] and the curr char also match
                    // then dp[i][j] can be set to true.
                    dp[i][j] = dp[i - 1][j - 1];
                } else if p_str[j] == '*' {
                    // for zero occurrence
                    if dp[i][j - 2] {
                        dp[i][j] = dp[i][j - 2];
                    }
                    // for multiple occurrence
                    else if s_str[i] == p_str[j - 1] || p_str[j - 1] == '.' {
                        dp[i][j] = dp[i - 1][j];
                    }
                } else {
                    dp[i][j] = false;
                }
            }
        }
        dp[m][n]
    }
}

trait MyTrait {
    fn char_at(&self, t: usize) -> Option<char>;
}

impl MyTrait for String {
    fn char_at(&self, t: usize) -> Option<char> {
        if let Some(c) = &self.chars().nth(t.try_into().unwrap()) {
            Some(*c)
        } else {
            None
        }
    }
}
