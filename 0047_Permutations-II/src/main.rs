use std::collections::HashMap;

mod tests;
fn main() {
    let input = vec![1, 1, 2];
    let output = Solution::permute_unique(input);
    let expected = vec![vec![1, 1, 2], vec![1, 2, 1], vec![2, 1, 1]];

    println!("{output:?}");
    expected.iter().for_each(|v| {
        if !output.contains(v) {
            panic!("Missing permutation: {v:?}")
        }
    });
}
pub struct Solution;
impl Solution {
    pub fn permute_unique(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let len = nums.len();
        let mut freq_map: HashMap<i32, usize> = HashMap::new();
        let mut res = vec![];
        let mut perm = vec![];
        nums.iter()
            .for_each(|ele| *freq_map.entry(*ele).or_insert(0) += 1);

        fn backtrack(
            result: &mut Vec<Vec<i32>>,
            freq_map: &mut HashMap<i32, usize>,
            curr_perm: &mut Vec<i32>,
            nums_len: usize,
        ) {
            // Base Case:
            if curr_perm.len() == nums_len {
                result.push(curr_perm.clone());
                return;
            }

            for (&num, &count) in freq_map.clone().iter() {
                if count > 0 {
                    *freq_map.get_mut(&num).unwrap() -= 1;
                    curr_perm.push(num);

                    backtrack(result, freq_map, curr_perm, nums_len);

                    curr_perm.pop();
                    *freq_map.get_mut(&num).unwrap() += 1;
                }
            }
        }

        backtrack(&mut res, &mut freq_map, &mut perm, len);

        res
    }
}
