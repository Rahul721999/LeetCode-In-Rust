mod tests;
fn main() {
    let (mut nums1, m) = (vec![1, 2, 3, 0, 0, 0], 3);
    let (mut nums2, n) = (vec![2, 5, 6], 3);

    Solution::merge(&mut nums1, m, &mut nums2, n);

    assert_eq!(nums1, vec![1, 2, 2, 3, 5, 6]);
}
pub struct Solution;
impl Solution {
    /// Solution: two-pointer approach.
    /// 1. if using extra space: Start from index: 0,
    /// 2. if not : Start from last index.
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        let mut l = m - 1;
        let mut r = n - 1; 
        let mut i = (m + n - 1) as usize;

        while l >= 0 && r >= 0 {
            let (n1, n2) = (nums1[l as usize], nums2[r as usize]);
            if n1 >= n2 {
                nums1[i] = n1;
                l -= 1;
            } else {
                nums1[i] = n2;
                r -= 1;
            }
            i -= 1;
        }

        while r >= 0 {
            nums1[i] = nums2[r as usize];
            r -= 1;
            i -= 1;
        }
    }
}
