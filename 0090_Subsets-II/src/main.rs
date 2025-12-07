pub mod tests;

fn main() {
    let input = vec![1, 2, 2];
    let output = Solution::subsets_with_dup_recursive(input);
    let expected = vec![
        vec![],
        vec![1],
        vec![1, 2],
        vec![1, 2, 2],
        vec![2],
        vec![2, 2],
    ];
    expected.iter().for_each(|s|{
        if !output.contains(&s){
            panic!("output doesn't contains: {:?}", s);
        }
    })
}
pub struct Solution;
impl Solution {
    pub fn subsets_with_dup(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        nums.sort();
        let mut subsets: Vec<Vec<i32>> = vec![vec![]];
        let mut start: usize;
        let mut prev_size: usize = 0; // prev subsets size

        for i in 0..nums.len() {
            if i > 0 && nums[i] == nums[i - 1] {
                start = prev_size;
            } else {
                start = 0;
            }

            prev_size = subsets.len();

            for j in start..prev_size {
                let mut new_subset_item = subsets[j].clone();
                new_subset_item.push(nums[i]);
                subsets.push(new_subset_item);
            }
        }
        subsets
    }

    pub fn subsets_with_dup_recursive(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        nums.sort();
        let mut subsets: Vec<Vec<i32>> = vec![vec![]];

        fn recursion(nums: &Vec<i32>, subsets: &mut Vec<Vec<i32>>, index: usize, prev_size: usize) {
            if index >= nums.len(){
                return;
            }

            let start: usize;
            if index > 0 && nums[index] == nums[index - 1] {
                start = prev_size;
            }else{
                start = 0;
            }

            let prev_size = subsets.len();
            for i in start..prev_size {
                let mut new_subset_item = subsets[i].clone();
                new_subset_item.push(nums[index]);
                subsets.push(new_subset_item);
            }
            recursion(nums, subsets, index + 1, prev_size);
        }

        recursion(&nums, &mut subsets, 0, 0);

        subsets
    }
}
