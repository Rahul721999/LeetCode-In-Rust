fn main() {
    let res = Solution::my_sqrt(2147483647);
    println!("{}",res);
}


pub struct Solution;
impl Solution {
    pub fn my_sqrt(x: i32) -> i32 {
        if x <= 1 {
            return x;
        }
    
        let (mut left, mut right) = (1, x);
        let mut result = 0;
    
        while left <= right {
            let mid = left + (right - left) / 2;
            let quotient = x / mid;
    
            if quotient == mid {
                return mid;
            } else if quotient < mid {
                right = mid - 1;
            } else {
                left = mid + 1;
                result = mid;
            }
        }
    
        result
    }
}