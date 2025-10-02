mod tests;
fn main() {
    let input = vec![vec![2, 1, 1], vec![1, 1, 0], vec![0, 1, 1]];
    let expected = 4;
    assert_eq!(expected, Solution::oranges_rotting(input));
}
pub struct Solution;
impl Solution {
    pub fn oranges_rotting(mut grid: Vec<Vec<i32>>) -> i32 {
        let length = grid.first().unwrap().len();
        let height = grid.len();

        if length == 0 || height == 0 {
            return 0;
        }

        let mut queue = std::collections::VecDeque::new();
        let mut fresh_count = 0;
        let mut max_time = 0;
        let directions: [(isize, isize); 4] = [(1, 0), (0, 1), (0, -1), (-1, 0)];

        for row in 0..height {
            for col in 0..length {
                match grid[row][col] {
                    2 => queue.push_back((row, col, 0)),
                    1 => fresh_count += 1,
                    _ => {}
                }
            }
        }

        while let Some((row, col, time)) = queue.pop_front() {
            for (dr, dc) in directions {
                let nr = row as isize + dr;
                let nc = col as isize + dc;

                if nr >= 0 && nr < height as isize && nc >= 0 && nc < length as isize {
                    let (nr, nc) = (nr as usize, nc as usize);
                    if grid[nr][nc] == 1 {
                        grid[nr][nc] = 2;

                        queue.push_back((nr, nc, time + 1));
                        fresh_count -= 1;
                        max_time = max_time.max(time + 1);
                    }
                }
            }
        }

        if fresh_count > 0 { return -1 } else { max_time }
    }
}
