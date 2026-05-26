mod tests;
use std::cmp::{max, min};
fn main() {
    let res = Solution::trap(vec![4, 2, 0, 3, 2, 5]);
    println!("max amount of water can be trapped is: {} unit", res);
}

struct Solution;
impl Solution {
    /// Time Complexity: O(n)
    /// Space Complexity: O(n) ---> can be optimized to O(1) using two pointers approach
    pub fn _trap(height: Vec<i32>) -> i32 {
        let len = height.len();
        if len == 0 {
            return 0;
        };

        let mut max_l: Vec<i32> = vec![0; len];
        max_l[0] = height[0];
        for i in 1..len {
            let l_max = max(max_l[i - 1], height[i]);
            max_l[i] = l_max;
        }

        let mut max_r: Vec<i32> = vec![0; len];
        max_r[len - 1] = height[len - 1];
        for i in (0..len - 1).rev() {
            let r_max = max(max_r[i + 1], height[i]);
            max_r[i] = r_max;
        }

        // calculate water trapped for arr[i]
        let mut res = 0;
        for i in 0..len - 1 {
            let water_at_i = min(max_l[i], max_r[i]) - height[i];
            // add to the result and return
            if water_at_i > 0 {
                res += water_at_i;
            }
        }

        res
    }

    /// Time Complexity: O(n)
    /// Space Complexity: O(1)
    fn trap(height: Vec<i32>) -> i32 {
        let len = height.len();

        let mut l_max = 0;
        let mut r_max = 0;

        let mut l = 0;
        let mut r = len - 1;

        let mut water = 0;
        while l < r {
            if height[l] < height[r] {
                l_max = l_max.max(height[l]);
                water += l_max - height[l];
                l += 1;
            } else {
                r_max = r_max.max(height[r]);
                water += r_max - height[r];
                r -= 1;
            }
        }

        water
    }
}
