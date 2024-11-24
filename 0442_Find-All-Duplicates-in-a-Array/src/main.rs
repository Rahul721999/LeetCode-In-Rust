mod tests;
fn main() {
    let input = vec![4,3,2,7,8,2,3,1];
    let output = Solution::find_duplicates(input);
    assert_eq!(output, vec![2,3]);
}

/* It is said that:
    - The Solution should not be using extra space.
    - & The time complexity should be O(n)

    IMP points:
        - Elements of the array will be in range from 1 to n
        n = arr.len()
*/

pub struct Solution;
impl Solution {
    pub fn find_duplicates(mut nums: Vec<i32>) -> Vec<i32> {
        let mut res = Vec::new();

        for i in 0..nums.len(){
            let index = (nums[i].abs() - 1) as usize;
            if nums[index].is_negative(){
                res.push((index + 1) as i32);
            }else{
                nums[index] = -nums[index]
            }
        }
        res
    }
}
