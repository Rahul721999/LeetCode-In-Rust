fn main() {
    let res = Solution::search_insert_position(vec![1,3,5], 4);
    println!("{res}");
}


struct Solution;
impl Solution{
    pub fn search_insert_position(nums: Vec<i32>, target: i32) -> i32 {
        let len = nums.len();
        let (mut left,mut right) = (0, len-1);
        let mut mid = 0;

        if target > nums[right]{return (right+1) as i32}
        else if target< nums[0] {return 0};

        while left <= right{
            mid = left + (right - left)/2;
            dbg!(left,mid,right);

            if target == nums[mid]{
                return mid as i32;
            }
            else if target < nums[mid] {
                if mid == 0 || target > nums[mid-1]{
                    return mid as i32;
                }
                right = mid - 1;
            }
            else if target > nums[mid]{
                left = mid+1;
            }
        }
        return mid as i32;
    }
}