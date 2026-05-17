mod tests;
fn main() {
    let input = vec![vec![3, 12], vec![-2, 5], vec![-4, 1]];
    let output = Solution::min_cost_connect_points(input);

    assert_eq!(18, output);
}
pub struct Solution;
impl Solution {
    pub fn min_cost_connect_points(points: Vec<Vec<i32>>) -> i32 {
        use std::cmp::Reverse;
        use std::collections::BinaryHeap;

        let len = points.len();
        let mut visited: Vec<bool> = vec![false; len];
        let mut min_heap: BinaryHeap<Reverse<(i32,usize)>> = BinaryHeap::new();

        // initializing...
        // visited.insert((0, 0)); // NOTE: if we do this, the loop will break in the first run
        min_heap.push(Reverse((0, 0)));

        let mut count = 0;
        let mut visited_count = 0;

        while let Some(Reverse((edge, i))) = min_heap.pop() {
            let (xi, yi) = (points[i][0], points[i][1]);

            if visited_count == len {
                break;
            };

            // check if already visited
            if visited[i] {
                continue;
            };

            // mark visited if not already
            count += edge;
            visited[i] = true;
            visited_count += 1;

            // check dist of all the other nodes from the curr node
            for (j, dest) in points.iter().enumerate() {
                let (xj, yj) = (dest[0], dest[1]);

                // calculate edge weight
                let edge = (xi - xj).abs() + (yi - yj).abs();

                if !((xi, yi) == (xj, yj)) {
                    // NOTE: dont push the curr node again
                    min_heap.push(Reverse((edge, j)));
                }
            }
        }

        count
    }
}
