mod tests;
fn main() {
    let mut input = vec![
        vec!['X', 'X', 'X', 'X'],
        vec!['X', 'O', 'O', 'X'],
        vec!['X', 'X', 'O', 'X'],
        vec!['X', 'O', 'X', 'X'],
    ];

    let expected = vec![
        vec!['X', 'X', 'X', 'X'],
        vec!['X', 'X', 'X', 'X'],
        vec!['X', 'X', 'X', 'X'],
        vec!['X', 'O', 'X', 'X'],
    ];

    Solution::solve(&mut input);

    assert_eq!(expected, input);
}
pub struct Solution;/*
    # Reverse Thinking Approach 💡

    Instead of trying to find which 'O' boxes are **surrounded** by 'X',
    we do the reverse —

    → Find the 'O' boxes that are **NOT surrounded** (i.e., touching the border),
      and mark all connected 'O's as safe.

    Steps:
    1️⃣  From the edges, mark all connected 'O's as 'T'  (safe ones)
    2️⃣  Flip remaining 'O's → 'X'                      (truly surrounded)
    3️⃣  Convert 'T's back → 'O'                        (restore safe ones)
*/

impl Solution {
    pub fn solve(board: &mut Vec<Vec<char>>) {
        let height = board.len();
        let length = board.first().unwrap().len();

        // 1️⃣  Select all edge 'O's — because they (and their connected region)
        //     can never be surrounded. Start DFS from each such border 'O'
        for row in 0..height {
            for col in 0..length {
                if board[row][col] == 'O'
                    && ([0, height - 1].contains(&row) || [0, length - 1].contains(&col))
                {
                    dfs(row, col, board)
                }
            }
        }

        // 2️⃣  Now flip all remaining 'O's → 'X'
        //     (these are the truly surrounded regions)
        for row in 0..height {
            for col in 0..length {
                if board[row][col] == 'O' {
                    board[row][col] = 'X'
                }
            }
        }

        // 3️⃣  Restore safe regions: 'T' → 'O'
        for row in 0..height {
            for col in 0..length {
                if board[row][col] == 'T' {
                    board[row][col] = 'O'
                }
            }
        }
    }
}

// DFS: visit all connected 'O's and mark them as temporary safe ('T')
fn dfs(row: usize, col: usize, board: &mut Vec<Vec<char>>) {
    // stop if out of bounds or not an 'O'
    if col >= board.first().unwrap().len() || row >= board.len() || board[row][col] != 'O' {
        return;
    }

    board[row][col] = 'T';

    // explore all 4 directions
    dfs(row + 1, col, board);
    dfs(row, col + 1, board);
    if row > 0 {
        dfs(row - 1, col, board)
    };
    if col > 0 {
        dfs(row, col - 1, board)
    }
}
