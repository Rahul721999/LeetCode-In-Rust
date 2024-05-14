mod tests;
fn main() {
    // [1,2,3,4] => [2*3*4, 1*3*4, 1*2*4, 1*2*3] => [24,12,8,6]
    assert_eq!(Solution::product_except_self(vec![1,2,3,4]), vec![24,12,8,6])
}


struct Solution;
impl Solution{
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let len = nums.len();
        if len <= 1{return nums}

        let mut res = vec![0; len];
        res[0] = 1;
        for i in 1..len{
            res[i] = nums[i-1] * res[i-1];
        }

        let mut r = 1;
        for i in (0..len-1).rev(){
            res[i] = res[i] * nums[i+1] * r;
            r *= nums[i+1];
        } 
        res
    } 
}