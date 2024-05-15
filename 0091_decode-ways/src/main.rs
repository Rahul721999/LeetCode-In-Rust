mod tests;

fn main() {
    let res = Solution::num_decodings(String::from("06"));
    assert_eq!(res, 0)
}

pub struct Solution;
impl Solution {
    pub fn num_decodings(s: String) -> i32 {
        let arr: Vec<char> = s.chars().collect();
        if arr.is_empty() || arr[0] == '0' {
            return 0;
        }

        let mut dp = vec![0; arr.len()+1];
        dp[0] = 1;
        for i in 1..dp.len() {
            dbg!(i);
            let mut temp = 0;
            let curr_digit = arr[i - 1];
            // if current single digit is_valid {temp += dp[i-1]}
            if curr_digit as u8 != b'0' {
                temp += dp[i - 1];
            }
            // if last two digit is_valid{temp += dp[i-2]}
            if i > 1 && (arr[i - 2] == '1' || (arr[i - 2] == '2' && curr_digit as u8 <= b'6')) {
                temp += dp[i - 2];
            }
            dp[i] = temp;
        }
        println!("{:?}", dp);
        dp.pop().unwrap()
    }
}
