use std::collections::HashSet;

fn main() {
    assert_eq!(Solution::first_missing_positive(vec![1,2,0]), 3);
    assert_eq!(Solution::first_missing_positive(vec![3,4,-1,1]), 2);
    assert_eq!(Solution::first_missing_positive(vec![7,8,9,11,12]), 1);
    assert_eq!(Solution::first_missing_positive(vec![0]), 1);
}

struct Solution;
impl Solution{
    pub fn first_missing_positive(nums: Vec<i32>) -> i32 {
        let set: HashSet<i32> = HashSet::from_iter(nums);
        
        for i in 1..=set.len()+1{
            if !set.contains(&(i as i32)){
                return i as i32
            }
        }
        return 1
    }
}