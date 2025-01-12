mod tests;
fn main() {
    let input = vec![1, 2, 3];
    let output = Solution::permute(input);
    let expected = vec![
        vec![3, 2, 1],
        vec![2, 3, 1],
        vec![3, 1, 2],
        vec![2, 1, 3],
        vec![1, 3, 2],
        vec![1, 2, 3],
    ];
    expected.iter().for_each(
        |v| if !output.contains(v){panic!("Missing permutation: {v:?}")});
}
pub struct Solution;
impl Solution {
    /*
        Recursive solution:
            for every ele in array
                pop the curr ele
                find perms recursively for rest of the array
                for each perm in perms
                    add the poped value to each perm
                add the perms to the res.
    */

    pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let len = nums.len();
        let mut res: Vec<Vec<i32>> = vec![];

        // return if len <= 1
        if len == 1 {
            return vec![nums.clone()];
        };

        for i in 0..len {
            let mut num_clone = nums.clone();
            let n = num_clone.remove(i);
            let mut perms = Self::permute(num_clone);

            for perm in perms.iter_mut() {
                perm.push(n);
            }
            res.extend(perms);
        }
        return res;
    }
}