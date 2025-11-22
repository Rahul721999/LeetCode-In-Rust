mod tests;
fn main() {
    let input = vec![1, 2, 3];
    let output = Solution::subsets_recursive_soln(input);
    let expected = vec![
        vec![],
        vec![1],
        vec![2],
        vec![1, 2],
        vec![3],
        vec![1, 3],
        vec![2, 3],
        vec![1, 2, 3],
    ];
    assert_eq!(output.len(), expected.len());
    expected.iter().for_each(|s| {
        if !output.contains(&s) {
            panic!("output doesn't contains: {:?}", s);
        }
    });
}
pub struct Solution;
impl Solution {
    pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
        // start with the empty subset
        let mut result = vec![vec![]];

        for &num in &nums {
            let mut expanded = Vec::new();

            // for each existing subset, create a new one that includes `num`
            for subset in &result {
                let mut with_num = subset.clone();
                with_num.push(num);
                expanded.push(with_num);
            }

            // add all newly created subsets
            result.extend(expanded);
        }

        result
    }

    pub fn subsets_recursive_soln(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut result = vec![vec![]];

        fn recurse(nums: &Vec<i32>, result: &mut Vec<Vec<i32>>, index: usize) {
            if index == nums.len() {
                return;
            }

            // create subsets that include nums[index]
            let mut expanded = Vec::new();
            for subset in result.iter() {
                let mut with_num = subset.clone();
                with_num.push(nums[index]);
                expanded.push(with_num);
            }

            // add them to the result
            result.extend(expanded);

            // move to the next element
            recurse(nums, result, index + 1);
        }

        recurse(&nums, &mut result, 0);

        result
    }
}
