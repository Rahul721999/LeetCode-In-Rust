use std::{cmp::Reverse, collections::BinaryHeap};

mod tests;
fn main() {
    let input = vec![
        vec![0, 1, 2, 3, 4],
        vec![24, 23, 22, 21, 5],
        vec![12, 13, 14, 15, 16],
        vec![11, 17, 18, 19, 20],
        vec![10, 9, 8, 7, 6],
    ];
    let output = Solution::swim_in_water(input);
    println!("{}", output);
}
pub struct Solution;
impl Solution {
    pub fn swim_in_water(grid: Vec<Vec<i32>>) -> i32 {
        if grid.len() <= 0 {
            return 0;
        };
        let len = grid[0].len();
        let height = grid.len();

        let mut heap: BinaryHeap<Reverse<(i32, usize, usize)>> = BinaryHeap::new();
        let mut visited = vec![vec![false; len]; height];

        // initialise
        heap.push(Reverse((grid[0][0], 0, 0)));

        let directions: Vec<(i32, i32)> = vec![(0, 1), (1, 0), (0, -1), (-1, 0)];

        while let Some(Reverse((curr_max_height, row, col))) = heap.pop() {
            if visited[row][col] {
                continue;
            }
            visited[row][col] = true;

            // if it's already the destination return
            if row == height - 1 && col == len - 1 {
                return curr_max_height;
            }
            // check out the neighbers:
            for (r, c) in directions.iter() {
                let nr = row as i32 + r;
                let nc = col as i32 + c;

                if nr < 0 || nc < 0 || nr as usize >= height || nc as usize >= len {
                    continue;
                }

                let nr = nr as usize;
                let nc = nc as usize;
                let max_until_now = std::cmp::max(curr_max_height, grid[nr][nc]);
                heap.push(Reverse((max_until_now, nr, nc)));
            }
        }
        0
    }
}
