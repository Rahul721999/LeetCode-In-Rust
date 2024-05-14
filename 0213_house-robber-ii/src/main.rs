fn main() {
    let v = [2, 3, 2];
    let len = v.len();
    // println!("{:?}", &v[0..len-1]);
    // println!("{:?}", &v[1..len]);
    let res = Solution::rob(vec![2, 3, 2]);
    println!("{res}");
}

pub struct Solution;
impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        let len = nums.len();
        if len < 2 {
            return nums[0];
        }
        let first = Self::max_loot((nums[0..len - 1]).to_vec());
        let second = Self::max_loot((nums[1..len]).to_vec());
        std::cmp::max(first, second)
    }

    pub fn max_loot(nums: Vec<i32>) -> i32 {
        use std::cmp;
        let len = nums.len();

        if len < 2 {
            nums[0]
        } else {
            let mut max = vec![0; len];

            max[0] = nums[0];
            max[1] = cmp::max(nums[0], nums[1]);

            for i in 2..nums.len() {
                max[i] = cmp::max(max[i - 2] + nums[i], max[i - 1])
            }

            max[nums.len() - 1]
        }
    }
}
