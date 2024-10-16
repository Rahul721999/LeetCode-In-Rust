mod tests;
use std::collections::HashSet;

fn main() {
    let input = vec![1,2,3,1];
    let output = Solution::contains_nearby_duplicate(input, 3);
    assert!(output);
}

pub struct Solution;
impl Solution {
    pub fn contains_nearby_duplicate(nums: Vec<i32>, k: i32) -> bool {
        let mut window : HashSet<i32>= HashSet::new();
        let mut l = 0_usize;

        for (r, ele) in nums.iter().enumerate(){
            if r - l > k as usize{
                window.remove(ele);
                l += 1;
            }
            if window.contains(ele){
                return true
            }
            window.insert(*ele);
        }
        false
    }
}
