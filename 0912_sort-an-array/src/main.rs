mod tests;
fn main() {
    let res = Solution::quick_sort_array(vec![5,1,0,0]);
    println!("{:?}", res);
}

struct Solution;
impl Solution {
    pub fn quick_sort_array(mut nums: Vec<i32>) -> Vec<i32> {
        quick_sort(&mut nums[..]);
        return nums;
        fn quick_sort(nums: &mut [i32]) {
            let len = nums.len();
            if len < 2 {
                return;
            }
            let pivot = nums[(len-1) / 2];
            let (mut l, mut r) = (0, len+1);
            let partition_point = loop {
                while { l+=1; nums[l-1] < pivot } {}
                while { r-=1; nums[r-1] > pivot } {}
                if r <= l {
                    break r-1;
                }
                nums.swap(l-1, r-1);
            };
            quick_sort(&mut nums[0..=partition_point]);
            quick_sort(&mut nums[partition_point+1..]);
        }
        
    }
}

