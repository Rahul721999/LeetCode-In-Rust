mod tests;
fn main() {
    let input = vec![17, 18, 5, 4, 6, 1];
    let output = Solution::replace_elements(input);
    assert_eq!(output, vec![18, 6, 6, 6, 1, -1]);
}
pub struct Solution;
impl Solution {
    pub fn replace_elements(mut arr: Vec<i32>) -> Vec<i32> {
        let len = arr.len();
        let mut max = -1;
        for i in (0..len).rev() {
            let temp = arr[i];
            arr[i] = max;
            max = temp.max(max);
        }
        arr
    }
}
