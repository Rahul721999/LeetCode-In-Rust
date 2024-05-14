mod b_tree;
mod tests;
use b_tree::{create_b_tree, TreeNode};

use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    let test_vec = vec![3,9,20,0,0,15,7];
    let len = test_vec.len();
    let tree = create_b_tree(&test_vec, len, 0);
    println!("{tree:?}");
}

pub struct Solution;
impl Solution {
    pub fn is_balanced(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        Self::b_tree_hight(root).0
    }

    fn b_tree_hight(root: Option<Rc<RefCell<TreeNode>>>) -> (bool, i32) {
        if let Some(node) = root {
            let node = node.borrow();
            let (l_tree_balanced, l_hight) = Self::b_tree_hight(node.left.clone());
            let (r_tree_balanced, r_hight) = Self::b_tree_hight(node.right.clone());

            if !l_tree_balanced || !r_tree_balanced {
                (false, 0)
            } else if l_hight > r_hight + 1 || r_hight > l_hight + 1 {
                return (false, 0);
            } else {
                (true, std::cmp::max(l_hight, r_hight) + 1)
            }
        } else {
            (true, 0)
        }
    }
}
