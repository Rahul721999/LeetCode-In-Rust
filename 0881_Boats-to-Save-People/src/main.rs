mod tests;
fn main() {
    let input = vec![3,5,3,4];
    let limit = 5;
    assert_eq!(Solution::num_rescue_boats(input, limit), 4);
}
pub struct Solution;
impl Solution {
    pub fn num_rescue_boats(mut people: Vec<i32>, limit: i32) -> i32 {
        let len = people.len();

        let (mut l, mut r) = (0, len - 1);
        let mut res = 0;

        // Step 1: Sort the array first
        people.sort();

        
        while l < r {
            let rem = limit - people[r];
            r -= 1;
            res += 1;

            if l <= r && rem >= people[l]{
                l += 1;
            }
        }

        // One person left
        if l == r{
            res += 1;
        }
        res
    }
}
