mod tests;
fn main() {
    // Solution::rotate(&mut vec![vec![5,1,9,11],vec![2,4,8,10],vec![13,3,6,7],vec![15,14,12,16]]);
    let mut res = vec![
    vec![1, 2, 3], 
    vec![4, 5, 6], 
    vec![7, 8, 9]
    ];
    Solution::rotate(&mut res);
}

struct Solution;
impl Solution {
    pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
        let len = matrix.len();
        for row in 0..len {
            for col in row..len {
                // trying to avoid some loops
                println!("-matrix[{row}] [{col}] = {}    |   matrix[{col}] [{row}] = {}-", matrix[row][col], matrix[col][row]);

                if row != col {
                    // transpose the matrix ...
                    // [[1,4,7],
                    //  [2,5,8],
                    //  [3,6,9]]
                    let temp = matrix[row][col];
                    matrix[row][col] = matrix[col][row];
                    matrix[col][row] = temp;
                }
            }
        }
        for row in 0..len{
            matrix[row].reverse()
        }
        
    }
}

