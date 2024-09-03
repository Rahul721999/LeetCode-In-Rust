mod tests;
use std::cmp::{max, min};
fn main() {
    let res = Solution::trap(vec![4, 2, 0, 3, 2, 5]);
    println!("max amount of water can be trapped is: {} unit", res);
}

struct Solution;
impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        let len = height.len();
        if len == 0 {
            return 0;
        };

        // example: [4, 2, 0, 3, 2, 5]

        // calculate max height to the left of i and store in an array
        // >>> [4, 4, 4, 4, 4, 5] >>>
        let mut max_l: Vec<i32> = vec![0; len];
        max_l[0] = height[0];
        for i in 1..len {
            let l_max = max(max_l[i - 1], height[i]);
            max_l[i] = l_max;
        }
        println!("max_l{:?}", max_l);

        // calculate max height to the right of i and store in an array
        // <<< [5, 5, 5, 5, 5, 5] <<<
        let mut max_r: Vec<i32> = vec![0; len];
        max_r[len - 1] = height[len - 1];
        for i in (0..len - 1).rev() {
            let r_max = max(max_r[i + 1], height[i]);
            max_r[i] = r_max;
        }
        println!("max_r{:?}", max_r);

        // calculate water trapped for arr[i]
        let mut res = 0;
        for i in 0..len - 1 {
            // >>> [4, 4, 4, 4, 4, 5] >>>
            // <<< [5, 5, 5, 5, 5, 5] <<<
            // --- [4, 2, 0, 3, 2, 5]
            let water_at_i = min(max_l[i], max_r[i]) - height[i];
            println!("water_at[{}] -> {}", i, water_at_i);
            // add to the result and return
            if water_at_i > 0 {
                res += water_at_i;
            }
        }

        res
    }
}
