fn main() {
    let res = Solution::my_sqrt(2147483647);
    println!("{}",res);
}


pub struct Solution;
impl Solution {
    pub fn my_sqrt(x: i32) -> i32 {
        // simple binary search approach..
        let (mut l,mut r) = (0,x);
        let mut res = 0;
        while l <= r{
            let mid = l + (r-l) / 2;
            let quotient = x/mid;
            
            if quotient == mid {
                return mid;
            } else if quotient < mid {
                r = mid - 1;
            } else {
                l = mid + 1;
                res = mid;
            }
        }
        return res;
    }
}