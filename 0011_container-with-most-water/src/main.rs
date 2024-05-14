fn main() {
    assert_eq!(Solution::max_area(vec![1, 8, 6, 2, 5, 4, 8, 3, 7]), 49);
    assert_eq!(Solution::max_area(vec![1, 1]), 1);
}

use std::cmp;
struct Solution;
impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut max = 0;
        let s = height.len();

        let (mut left, mut right) = (0, s - 1);
        while left < right {
            let h = cmp::min(height[left], height[right]);
            max = cmp::max(max, h * (right - left) as i32);
            if height[left] < height[right] {
                left += 1;
            } else {
                right -= 1;
            }
        }

        max
    }
}
