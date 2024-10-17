mod tests;
fn main() {
    let intput = vec![2,2,2,2,5,5,5,8];
    let output = Solution::num_of_subarrays(intput, 3, 4);
    assert_eq!(output, 3);
}
pub struct Solution;
impl Solution {
    pub fn num_of_subarrays(arr: Vec<i32>, k: i32, threshold: i32) -> i32 {
        let mut sub_arr_count = 0;
        let mut start = 0;     
        let mut sum = 0;
        
        for (end, num) in arr.iter().enumerate(){
            sum += num;
            if end >= k as usize - 1{
                if sum / k >= threshold{
                    sub_arr_count += 1;
                }
                sum -= arr[start];
                start += 1;
            }
        }

        sub_arr_count
    }
}
