mod tests;
fn main() {
   let input = vec![vec![1,2,3],vec![4,5,6],vec![7,8,9]];
   let output = Solution::spiral_order(input );
   assert_eq!(output, vec![1,2,3,6,9,8,7,4,5]);
}
pub struct Solution;
impl Solution {
    pub fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
        let height = matrix.len() as i32;
        let width = matrix[0].len() as i32;

        let mut left = 0;
        let mut right = width - 1;

        let mut top = 0;
        let mut bottom = height - 1;

        let mut res = Vec::new();

        // how long should we do this ?
        while left <= right && top <= bottom {
            // top left to top right
            for i in left..=right {
                res.push(matrix[top as usize][i as usize]);
            }
            top += 1; // move the row pointer to point next row
            if top > bottom {break}

            // last-col top to bottom
            for i in top..=bottom {
                res.push(matrix[i as usize][right as usize]);
            }
            right -= 1;
            if left > right{break}

            // bottom right to bottom left
            for i in (left..=right).rev() {
                res.push(matrix[bottom as usize][i as usize]);
            }
            bottom -= 1;
            if top > bottom {break}

            // first col bottom to top
            for i in (top..=bottom).rev() {
                res.push(matrix[i as usize][left as usize]);
            }
            left += 1;
            if left > right {break}
        }
        res
    }
}
