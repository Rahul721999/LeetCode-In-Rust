pub struct Solution;
impl Solution {
    pub fn num_islands(grid: Vec<Vec<char>>) -> i32 {
        let length = grid.first().unwrap().len();
        let height = grid.len();

        if length == 0 || height == 0 {
            return 0;
        };

        let mut visited = vec![vec![false; length]; height];
        let mut count = 0;

        for row in 0..height {
            for col in 0..length {
                if grid[row][col] == '1' && !visited[row][col] {
                    dfs(&grid, &mut visited, row, col);
                    count += 1;
                }
            }
        }
        count
    }
}

pub fn dfs(grid: &Vec<Vec<char>>, visited: &mut Vec<Vec<bool>>, row: usize, col: usize) {
    if col >= grid.first().unwrap().len()
        || row >= grid.len()
        || visited[row][col]
        || grid[row][col] == '0'
    {
        return;
    }
    visited[row][col] = true;

    if row > 0 {
        dfs(grid, visited, row - 1, col);
    }
    if col > 0 {
        dfs(grid, visited, row, col - 1);
    }

    dfs(grid, visited, row, col + 1);
    dfs(grid, visited, row + 1, col);
}
