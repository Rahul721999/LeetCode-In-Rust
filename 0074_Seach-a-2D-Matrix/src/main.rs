mod tests;
fn main() {
    let matrix = vec![vec![1, 3, 5, 7], vec![10, 11, 16, 20], vec![23, 30, 34, 60]];
    let output = Solution::search_matrix(matrix, 34);
    println!("{output}");
}
pub struct Solution;
impl Solution {
    /*
        Time Complexity: O(log(m*n)); Where m = row, n = col;
        Space Complexity: O(1); constant space / no extra space;
    */
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        let height = matrix.len();
        let width = matrix[0].len();

        let mut left = 0 as i32;
        let mut right = height as i32 - 1;
        let mut pivot = 0;
        while left <= right {
            pivot = left + (right - left) / 2;
            let mid_ele = matrix[pivot as usize][0];
            if mid_ele == target { return true}
            if target > mid_ele{
                left = pivot + 1;
            }
            if mid_ele > target {
                right = pivot - 1;
            }
        }
        let row = pivot as usize;

        let mut left = 1 as i32; // because we've already searched 0th col
        let mut right = width as i32 - 1;

        while left <= right{
            let pivot = left + (right - left) / 2;
            let mid_val = matrix[row][pivot as usize];
            if mid_val == target{
                return true;
            }
            if target > mid_val{
                left = pivot + 1;
            }else{
                right = pivot - 1;
            }
        };

        false
    }
}
