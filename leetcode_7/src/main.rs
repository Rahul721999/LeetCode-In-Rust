fn main() {
    assert_eq!(Solution::reverse(123), 321);
    assert_eq!(Solution::reverse(-123), -321);
    assert_eq!(Solution::reverse(-120), -21);
    
    assert_eq!(Solution::reverse_optimized(123), 321);
    assert_eq!(Solution::reverse_optimized(-123), -321);
    assert_eq!(Solution::reverse_optimized(-120), -21);
    
}

struct Solution;
impl Solution {
    pub fn reverse(x: i32) -> i32 {
        x.signum() * // fetch sign
        x.abs() // fetch absolute value 
            .to_string() // convert i32 to string
            .chars() 
            .rev() // reverse the string
            .collect::<String>() // then collect them to a new string
            .parse::<i32>()// convert the new string to i32
            .unwrap_or(0)
        // return sign * i32
    }

    /// Optimized approach...
    pub fn reverse_optimized(x: i32) -> i32{
        let mut res = 0; // this will be our result
        let mut x = x as i64; // taking x as i64 to prevent overflow

        while x.abs() > 0 { // cause the value can be negative
            res *= 10; // first mult r with 10
            res += x % 10;
            x /= 10;
        }

        // finally convert the i64 to i32 and return
        i32::try_from(res).unwrap_or(0)
    }
}