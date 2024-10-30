mod tests;
fn main() {
    let mut input = vec![1, 2, 3, 4, 5, 6, 7];
    Solution::rotate(&mut input, 3);
    assert_eq!(input, vec![5, 6, 7, 1, 2, 3, 4]);

    let mut input = vec![1];
    Solution::rotate(&mut input, 0);
    assert_eq!(input, vec![1]);
}
pub struct Solution;
impl Solution {
    pub fn rotate(nums: &mut Vec<i32>, k: i32) {
        let n = nums.len();

        if n <= 1 || k == 0 || n == k as usize {
            return;
        }

        // avoid unnecessary rotation
        let k = k as usize % n;
        if n == 0 {
            return;
        }

        // Inline closure to reverse a range in-place
        let mut reverse = |start: usize, end: usize| {
            let (mut l, mut r) = (start, end);

            while l < r {
                nums.swap(l, r);
                l += 1;
                r -= 1;
            }
        };

        // Step 1: Reverse the entire array
        reverse(0, n - 1);

        // Step 2: Reverse the first k element in the reversed array
        reverse(0, k - 1);

        // Step 3: Reverse the remaining element in the reversed array
        reverse(k, n - 1);
    }
}
