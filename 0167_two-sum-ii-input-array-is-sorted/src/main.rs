fn main() {
    assert_eq!(Solution::two_sum(vec![2,7,11,15], 9), [1,2]);
}


struct Solution;
impl Solution {
    pub fn two_sum(nums: Vec<i32>, t: i32) -> Vec<i32> {
        let len = nums.len();
        let (mut left, mut right) = (0 as usize, (len as i32 -1) as usize);

        while left < right{
            match nums[left] + nums[right]{
                s if s > t => {right-=1},
                s if s < t => {left+=1},
                _ =>{return vec![left as i32 +1, right as i32 +1]}
            }
        }
        unreachable!()
    }
}