mod b_tree;
mod test;
use b_tree::*;
use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    let test_vec = vec![Some(3), Some(9), Some(20), None, None, Some(15), Some(7)];
    let len = test_vec.len();
    let test_tree = create_b_tree(test_vec, 0, len);
    assert_eq!(Solution::max_depth(test_tree), 3);
    
}

pub struct Solution;
impl Solution {
    pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if let Some(node) = root {
            let node = node.borrow();
            let left_subtree_len = Self::max_depth(node.left.clone());
            let right_subtree_len = Self::max_depth(node.right.clone());
            std::cmp::max(right_subtree_len, left_subtree_len) + 1
        } else {
            0
        }
    }
}
