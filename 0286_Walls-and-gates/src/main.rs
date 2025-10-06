mod tests;
fn main() {
    let mut input = vec![
        vec![2147483647, -1, 0, 2147483647],
        vec![2147483647, 2147483647, 2147483647, -1],
        vec![2147483647, -1, 2147483647, -1],
        vec![0, -1, 2147483647, 2147483647],
    ];
    let expected = vec![
        vec![3, -1, 0, 1],
        vec![2, 2, 1, -1],
        vec![1, -1, 2, -1],
        vec![0, -1, 3, 4],
    ];
    Solution::walls_and_gates(&mut input);
    assert_eq!(expected, input);
}
pub struct Solution;
impl Solution {
    pub fn walls_and_gates(room: &mut Vec<Vec<i32>>) {
        let height = room.len();
        let length = room.first().unwrap().len();

        use std::collections::VecDeque;
        let mut queue: VecDeque<(usize, usize)> = VecDeque::new();

        for row in 0..height {
            for col in 0..length {
                if room[row][col] == 0 {
                    queue.push_back((row, col));
                }
            }
        }

        let directions = [(1, 0), (-1, 0), (0, 1), (0, -1)];
        let inf = 2147483647;

        while let Some((row, col)) = queue.pop_front(){
            for (r, c) in directions {
                let r = r + row as i32;
                let c = c + col as i32;

                if r < 0 || c < 0 || r as usize >= height || c as usize >= length{
                    continue;
                }

                let (r,c) = (r as usize, c as usize);
                if room[r][c] == inf{
                    room[r][c] = room[row][col] + 1;
                    queue.push_back((r,c));
                }
            }
        }
    }
}
