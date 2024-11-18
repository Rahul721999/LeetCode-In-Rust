mod tests;
fn main() {
    let output = Solution::max_profit(vec![7,1,5,3,6,4]);
    assert_eq!(output, 5);
}
pub struct Solution;
impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32{
        let mut max_profit = 0;
        let mut min_so_far  = i32::MAX;

        for price in prices{
            if price > min_so_far {
                max_profit = max_profit.max(price - min_so_far); // Calculate profit if possible
            } else {
                min_so_far = price; // Update minimum price
            }
        }
        max_profit
    }
    pub fn max_profit_unoptimized(prices: Vec<i32>) -> i32 {
        let len= prices.len();
        let mut v1 = vec![0; len];
        let mut v2 = vec![0; len];

        let mut min_so_far = prices[0];
        for i in 0..len{
            min_so_far = prices[i].min(min_so_far);
            v1[i] = min_so_far;
        }
        let mut max_so_far = prices[len-1];
        for i in (0..len).rev(){
            max_so_far = prices[i].max(max_so_far);
            v2[i] = max_so_far;
        }
        let mut max_profit = 0;
        for i in 0..len{
            max_profit = max_profit.max(v2[i] - v1[i]);
        }
        max_profit
    }
}