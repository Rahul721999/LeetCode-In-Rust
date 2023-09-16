use std::collections::HashSet;
use std::{panic, vec};

fn main() {
    assert_eq!(Solution::three_sum(vec![0, 0, 0]), vec![vec![0, 0, 0]]);
    assert_eq!(Solution::three_sum(vec![0, 1, 1]), Vec::<Vec<i32>>::new());

    let result = Solution::three_sum(vec![-1, 0, 1, 2, -1, -4]);
    if result == [[-1, -1, 2], [-1, 0, 1]] {
        ..
    } else if result == [[-1, 0, 1], [-1, -1, 2]] {
        ..
    } else {
        panic!("Result: {:?}", result)
    };
}

struct Solution;
impl Solution {
    pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> { // [-1, 0, 1, 2, -1, -4]
        let len = nums.len();
        if len < 3 {
            return vec![];
        }
        nums.sort(); // [-1, -1, 0, 1, 2, 4]
        let mut res_set = HashSet::new();
        for i in 0..len -2 {
            let (mut j, mut k) = (i + 1, len - 1);
            while j < k {
                match nums[i] + nums[j] + nums[k] {
                    s if s < 0 => j += 1,
                    s if s > 0 => k -= 1,
                    _ => {
                        res_set.insert((nums[i], nums[j], nums[k]));
                        j += 1;
                        while nums[j] == nums[j - 1] && j < k {
                            j += 1;
                        }
                    }
                }
            }
        }

        res_set
            .iter()
            .map(|list| vec![list.0, list.1, list.2])
            .collect()
    }
}
