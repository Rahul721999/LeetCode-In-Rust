mod tests;
fn main() {
    let input = vec![
        vec![0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0],
        vec![0, 1, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0],
        vec![0, 1, 0, 0, 1, 1, 0, 0, 1, 0, 1, 0, 0],
        vec![0, 1, 0, 0, 1, 1, 0, 0, 1, 1, 1, 0, 0],
        vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0],
        vec![0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0],
        vec![0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0],
    ];
    let output = Solution::max_area_of_island(input);
    assert_eq!(output, 6);
}
pub struct Solution;
impl Solution {
    pub fn max_area_of_island(grid: Vec<Vec<i32>>) -> i32 {
        let length = grid.first().unwrap().len();
        let height = grid.len();

        if length == 0 || height == 0 {
            return 0;
        }

        let mut result = 0;
        let mut visited = vec![vec![false; length]; height];

        for row in 0..height {
            for col in 0..length {
                if grid[row][col] == 1 && !visited[row][col] {
                    let output = dfs(row, col, &grid, &mut visited);
                    result = result.max(output);
                }
            }
        }

        result
    }
}

pub fn dfs(row: usize, col: usize, grid: &Vec<Vec<i32>>, v: &mut Vec<Vec<bool>>) -> i32 {
    if col >= grid.first().unwrap().len() || row >= grid.len() || v[row][col] || grid[row][col] == 0
    {
        return 0;
    }

    v[row][col] = true;
    let mut island_size= 1; // for the current block.

    let prev_row = if row > 0 {
        dfs(row - 1, col, grid, v)
    } else {
        0
    };
    let prev_col = if col > 0 {
        dfs(row, col - 1, grid, v)
    } else {
        0
    };
    island_size +=  dfs(row + 1, col, grid, v) + dfs(row, col + 1, grid, v) + prev_row + prev_col;

    island_size
}
