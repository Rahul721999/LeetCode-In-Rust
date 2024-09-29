mod b_tree;
mod tests;
use b_tree::*;
use std::cell::RefCell;
pub use std::collections::VecDeque;
use std::rc::Rc;

fn main() {
    let input = vec![Some(3), Some(9), Some(20), None, None, Some(15), Some(7)];
    let len = input.len();
    let b_tree = create_b_tree(input, 0, len);
    let expected_output = vec![vec![15, 7], vec![9, 20], vec![3]];

    let output = Solution::level_order(b_tree);
    assert_eq!(output, expected_output);
}

pub struct Solution;
impl Solution {
    pub fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut res: Vec<Vec<i32>> = Vec::new();
        let mut queue: VecDeque<Rc<RefCell<TreeNode>>> = VecDeque::new();

        if let Some(r) = root {
            queue.push_back(r);
        }
        while !queue.is_empty() {
            let len = queue.len();
            let mut subarr: Vec<i32> = Vec::new();
            for _ in 0..len {
                let curr_node = queue.pop_front().unwrap();
                let n = curr_node.borrow();
                subarr.push(n.val); 

                if let Some(left) = n.left.clone() {
                    queue.push_back(left);
                }
                if let Some(right) = n.right.clone() {
                    queue.push_back(right);
                }
            }
            res.insert(0, subarr);
        }
        res 
    }
}
