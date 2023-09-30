#![allow(unused)]
use std::{collections::HashSet, mem::swap};

fn main() {
    assert_eq!(Solution::first_missing_positive_optimised(vec![1, 2, 0]), 3);
    assert_eq!(
        Solution::first_missing_positive_optimised(vec![3, 4, -1, 1]),
        2
    );
    assert_eq!(
        Solution::first_missing_positive_optimised(vec![7, 8, 9, 11, 12]),
        1
    );
    assert_eq!(Solution::first_missing_positive_optimised(vec![0]), 1);
}

struct Solution;
impl Solution {
    pub fn first_missing_positive(nums: Vec<i32>) -> i32 {
        let set: HashSet<i32> = HashSet::from_iter(nums);

        for i in 1..=set.len() + 1 {
            if !set.contains(&(i as i32)) {
                return i as i32;
            }
        }
        return 1;
    }
    pub fn first_missing_positive_optimised(mut nums: Vec<i32>) -> i32 {
        let len = nums.len();

        for i in 0..len {
            let mut val = nums[i];

            // pos-> 0, 1, 2, 3, 4, 5,
            // val-> 3, 4,-1, 1

            // val(3) > 0 && val(3<= len(3)) && nums[3] = ()
            while (val > 0) && (val <= len as i32) && nums[val as usize - 1] != val {
                nums.swap(i, val as usize - 1);
                val = nums[i];
            }
        }

        // find the missing positive integer
        for i in 0..len {
            if i as i32 != nums[i] - 1 {
                return i as i32 + 1;
            }
        }
        // If all positive integers from 1 to len are present, return len + 1
        len as i32 + 1
    }
    // time-complexity is: O(n^2)
    // space-complexity is: O(1)
}

#[cfg(test)]
mod tests {
    use crate::Solution;
    #[test]
    fn test1() {
        assert_eq!(Solution::first_missing_positive_optimised(vec![1, 2, 0]), 3);
        assert_eq!(
            Solution::first_missing_positive_optimised(vec![3, 4, -1, 1]),
            2
        );
        assert_eq!(
            Solution::first_missing_positive_optimised(vec![7, 8, 9, 11, 12]),
            1
        );
        assert_eq!(Solution::first_missing_positive_optimised(vec![0]), 1);
    }
    #[test]
    fn test2() {
        assert_eq!(Solution::first_missing_positive(vec![1, 2, 0]), 3);
        assert_eq!(Solution::first_missing_positive(vec![3, 4, -1, 1]), 2);
        assert_eq!(Solution::first_missing_positive(vec![7, 8, 9, 11, 12]), 1);
        assert_eq!(Solution::first_missing_positive(vec![0]), 1);
    }
}
