mod tests;
fn main() {
    let output = Solution::subarray_sum(vec![1,2,3], 3);
    assert_eq!(output, 2);
}
pub struct Solution;
impl Solution {
    pub fn subarray_sum(nums: Vec<i32>, k: i32) -> i32 {
        use std::collections::HashMap;

        let mut prefix_sum_count: HashMap<_, _> = HashMap::new();
        prefix_sum_count.insert(0, 1); // ! Initialize with sum 0 occurring once.

        let mut sum = 0; let mut count = 0;
        
        for num in nums{
            sum += num;
            
            if let Some(&freq) = prefix_sum_count.get(&(sum - k)){
                count += freq;
            }

            *prefix_sum_count.entry(sum).or_insert(0) += 1;
        }

        count
    }
}