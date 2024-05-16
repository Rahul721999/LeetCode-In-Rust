mod memoization;
mod optimized;
fn main() {
    let res = Solution::recursive_knapsack(3, 4, &[1, 2, 3], &[4, 5, 1]);
    assert_eq!(res, 3);
    let res = Solution::knapsack_recursive_dp(3, 4, &[1, 2, 3], &[4, 5, 1]);
    assert_eq!(res, 3);
    let res = Solution::optimized_dp(3, 4, &[1, 2, 3], &[4, 5, 1]);
    assert_eq!(res, 3);
}

/*
    Given N items where each item has some weight and profit associated with it and also given a bag with capacity W,
[i.e., the bag can hold at most W weight in it].
The task is to put the items into the bag such that the sum of profits associated with them is the maximum possible.

    Note: The constraint here is we can either put an item completely into the bag or
cannot put it at all [It is not possible to put a part of an item into the bag].
    Example_1 :
        input:                  |   output: 3
            n = 3,              |
            w = 4,              |
            profit = [1,2,3],   |
            weight = [4,5,1]    |

*/
pub struct Solution;
impl Solution {
    pub fn recursive_knapsack(
        n: i32,          // no of item
        max_weight: i32, // max weight the bag can hold
        profit: &[i32],  // max profit on each item
        weight: &[i32],  // weight of the each given item
    ) -> i32 {
        if n == 0 || max_weight == 0 {
            return 0;
        }
        use std::cmp;
        let index = n as usize - 1;
        if weight[index] <= max_weight {
            cmp::max(
                profit[index]
                    + Self::recursive_knapsack(n - 1, max_weight - weight[index], profit, weight),
                Self::recursive_knapsack(n - 1, max_weight, profit, weight),
            )
        } else {
            Self::recursive_knapsack(n - 1, max_weight, profit, weight)
        }
    }

    
}

#[cfg(test)]
mod tests{
    use crate::Solution;
    #[test]
    fn test_recursive_knapsack1(){
        let res = Solution::recursive_knapsack(3, 4, &[1, 2, 3], &[4, 5, 1]);
        assert_eq!(res, 3);
    }
    #[test]
    fn test_recursive_knapsack2() {
        let res = Solution::recursive_knapsack(3, 50, &[60,100,120], &[10,20,30]);
        assert_eq!(res, 220);
    }
}