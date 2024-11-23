use std::collections::HashMap;

mod tests;
fn main() {
    let input = vec![23, 2, 4, 6, 7];
    assert_eq!(Solution::check_subarray_sum(input.clone(), 6), true);
    assert_eq!(Solution::check_subarray_sum(input.clone(), 13), true);
    assert_eq!(Solution::check_subarray_sum(input, 7), true);

    
}
pub struct Solution;
impl Solution {
    pub fn check_subarray_sum(nums: Vec<i32>, k: i32) -> bool {
        // HashMap based solution, store remainder & index
        let mut map = HashMap::new();
        map.insert(0, -1); //

        let mut prefix_sum = 0;
        for (i, ele) in nums.iter().enumerate() {
            prefix_sum += ele;
            let rem = (prefix_sum % k + k) % k;

            if let Some(&prev_index) = map.get(&rem) {
                let curr_sub_arr_len = i as i32 - prev_index;
                if curr_sub_arr_len > 1 {
                    return true;
                }
            } else {
                map.insert(rem, i as i32);
            }
        }
        false
    }
}
