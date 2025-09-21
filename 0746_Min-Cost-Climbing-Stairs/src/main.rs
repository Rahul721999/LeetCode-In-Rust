mod tests;
fn main() {
    let input = vec![1, 100, 1, 1, 1, 100, 1, 1, 100, 1];
    let expected = 6;

    let output = Solution::min_cost_climbing_stairs(input);
    assert_eq!(output, expected);
}
pub struct Solution;
impl Solution {
    pub fn min_cost_climbing_stairs(cost: Vec<i32>) -> i32 {
        use std::cmp;
        let len = cost.len();
        if len <= 1 {
            return cost[0]
        }
        let mut min_cost = vec![0; len + 1]; // as the top is actually n + 1 as per problem statement

        // As you can start from step 0 or step 1

        min_cost[0] = 0; // the cost to reach 0th stair is 0
        min_cost[1] = 0; // the cost to reach 0th stais is also 0 cause you are paying no cost at all

        for i in 2..=len{

            // as you can take either one step / two step at a time
            let a = cost[i - 1] + min_cost[i - 1]; // cost of the (n-1) step + min cost to reach (n-1) step
            let b = cost[i - 2] + min_cost[i - 2]; // cost of the (n-2) step + min cost to reach (n-2) step

            // so the cost to reach to the top (n) is
            // min of (a | b)
            min_cost[i] = cmp::min(a, b);
        }
        min_cost[len]
    }
}
