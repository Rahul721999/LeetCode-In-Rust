use std::collections::HashMap;

fn main() {
    let input = vec![1, 3, 2, 2, 5, 2, 3, 7];
    assert_eq!(Solution::find_lhs(input), 5);

    let input = vec![1, 1, 1, 1];
    assert_eq!(Solution::find_lhs(input), 0);

    let input = vec![1, 2, 3, 4];
    assert_eq!(Solution::find_lhs(input), 2);
}

pub struct Solution;
impl Solution {
    pub fn find_lhs(nums: Vec<i32>) -> i32 {
        let mut map: HashMap<i32, i32> = HashMap::new();

        // Count frequency of each element and store it in the hashmap
        for &ele in &nums {
            *map.entry(ele).or_insert(0) += 1;
        }

        let mut max_len = 0;

        // Iterate through the map
        for (&ele, &freq) in &map {
            if let Some(&next_freq) = map.get(&(ele + 1)) {
                max_len = i32::max(max_len, freq + next_freq);
            }
        }

        max_len
    }
}
