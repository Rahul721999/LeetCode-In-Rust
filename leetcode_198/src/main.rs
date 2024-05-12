fn main() {
    let test_vec = vec![1,2,3,1];
    assert_eq!(Solution::rob(test_vec), 4);
}
pub struct Solution;
impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        use std::cmp;
        let len = nums.len();
        if len <= 1 {
            return nums[0];
        }
        let mut max = vec![0; len];

        max[0] = nums[0];
        max[1] = cmp::max(nums[0], nums[1]);

        for i in 2..nums.len() {
            max[i] = cmp::max(max[i - 2] + nums[i], max[i - 1])
        }

        max[len - 1]
    }
}

#[cfg(test)]
mod test{
    use crate::Solution;
    #[test]
    fn test1(){
        let test_vec = vec![2,7,9,3,1];
        assert_eq!(Solution::rob(test_vec), 12)
    }
    #[test]
    fn test2(){
        let test_vec = vec![2,1];
        assert_eq!(Solution::rob(test_vec), 2)
    }
}
