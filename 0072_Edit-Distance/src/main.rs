mod tests;
fn main() {
    let word1 = "horse".to_string();
    let word2 = "ros".to_string();
    let output = Solution::min_distance(word1, word2);
    assert_eq!(output, 3);
}

// convert word1 to word2

// word1: horse
// word2: ros

// i | 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7
//
//   | h | o | r | s | e |   |
//   | r | o | s
//   |

// make a table table of these two arrays
//  i| 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7
//   | h | o | r | s | e | ''|
//  r| 3 | 3 | 2 | 3 | 3 | 3 |
//  o| 3 | 2 | 2 | 2 | 2 | 2 |
//  s| 4 | 3 | 2 | 1 | 1 | 1 |
// ''| 5 | 4 | 3 | 2 | 1 | 0 |

// word1: intention
// word2: execution

// i | 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 |
//
//   | i | n | t | e | n | t | i | o | n |
//   | e | x | e | c | u | t | i | o | n |
//
//   | 5 | 4 | 3 | 2 | 1 | 0 | 0 | 0 | 0 |

pub struct Solution;
impl Solution {
    pub fn min_distance(word1: String, word2: String) -> i32 {
        let len1 = word1.len();
        let len2 = word2.len();
    
        // Create a 2D DP array with dimensions (len2 + 1) x (len1 + 1)
        let mut dp = vec![vec![0; len1 + 1]; len2 + 1];
    
        // Fill the base cases
        for i in 0..=len1 {
            dp[0][i] = i; // Cost of deleting all characters from word1
        }
        for j in 0..=len2 {
            dp[j][0] = j; // Cost of inserting all characters into word1
        }
    
        // Fill the DP array
        let word1_chars: Vec<char> = word1.chars().collect();
        let word2_chars: Vec<char> = word2.chars().collect();
    
        for j in 1..=len2 {
            for i in 1..=len1 {
                if word1_chars[i - 1] == word2_chars[j - 1] {
                    dp[j][i] = dp[j - 1][i - 1]; // No cost if characters match
                } else {
                    dp[j][i] = 1 + dp[j - 1][i - 1].min(dp[j][i - 1].min(dp[j - 1][i]));
                    // Minimum of substitution, insertion, or deletion
                }
            }
        }
    
        // The result is in the bottom-right cell
        dp[len2][len1] as i32
    }
    
}
