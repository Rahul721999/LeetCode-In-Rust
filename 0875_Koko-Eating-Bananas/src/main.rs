mod tests;
fn main() {
    let res = Solution::min_eating_speed(vec![30, 11, 23, 4, 20], 5);
    assert_eq!(res, 30);
}

pub struct Solution;
impl Solution {
    pub fn min_eating_speed(piles: Vec<i32>, h: i32) -> i32 {
        let max = piles.iter().max().unwrap().to_owned(); // search the max value in the given vec
        let (mut left, mut right) = (1, max); // set lower and upper bound
        let mut min_rate = right; // min rate of eating bananas.

        while right >= left {
            let mid = left + (right - left) / 2;
            let mut hours = 0;

            // Calculate the total hours required for the current eating speed `mid`
            for &p in &piles {
                hours += (p + mid-1) / mid; // rounding up the number is imp
            }
            if hours <= h { // If Koko can eat all bananas within `h` hours, try to minimize the rate
                min_rate = min_rate.min(mid);
                right = mid - 1;
            } else { // Otherwise, increase the eating speed
                left = mid + 1;
            }
        }
        min_rate
    }
}
