mod tests;
mod b_tree;
use b_tree::*;
use std::cell::RefCell;
pub use std::collections::VecDeque;
use std::rc::Rc;

fn main() {
    let input = vec![Some(3),Some(9),Some(20),None,None,Some(15),Some(7)];
    let len = input.len();
    let b_tree = create_b_tree(input, 0, len);
    let expected_output = vec![vec![3],vec![9,20],vec![15,7]];

    let output = Solution::level_order(b_tree);
    assert_eq!(output, expected_output);
}

pub struct Solution;
impl Solution {
    pub fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut res: Vec<Vec<i32>> = Vec::new();
        let mut queue = VecDeque::new();
        if let Some(r) = root {
            queue.push_back(r);
        };

        while !queue.is_empty() {
            let level_size = queue.len(); // no of nodes in the current lvl
            let mut curr_lvl: Vec<i32> = Vec::new();

            for _ in 0..level_size {
                let node = queue.pop_front();
                let n = borrow_node!(node);

                curr_lvl.push(n.val);

                // Push the child node of the curr_node
                if let Some(left) = n.left.clone() {
                    queue.push_back(left);
                };
                if let Some(right) = n.right.clone() {
                    queue.push_back(right);
                };
            }

            // At last push the curr_lvl vec to actual result
            res.push(curr_lvl);
        }

        res
    }
}
