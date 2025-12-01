mod tests;
fn main() {
    let nums = vec![2, 5, 6, 9];
    let target = 9;
    let result = Solution::combination_sum(nums.to_vec(), target);
    let expected = vec![vec![2, 2, 5], vec![9]];
    assert_eq!(result.len(), expected.len());
    expected.iter().for_each(|v| 
        assert!(result.contains(v))
    );
}
pub struct Solution;

impl Solution {
    pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut res = vec![];
        let mut path = vec![];
        backtrack(candidates, target, &mut res, &mut path, 0);
        res
    }
}

fn backtrack(
    candidates: Vec<i32>,
    target: i32,
    res: &mut Vec<Vec<i32>>,
    path: &mut Vec<i32>,
    index: usize,
) {
    if target == 0 {
        res.push(path.clone());
        return;
    }

    if index >= candidates.len() || target < 0 {
        return;
    }

    let value = candidates[index];

    // Either take the current ele
    if value <= target {
        path.push(value);
        backtrack(candidates.clone(), target - value, res, path, index);
        path.pop();
    }

    // or Skip the current ele
    backtrack(candidates, target, res, path, index + 1);
}
