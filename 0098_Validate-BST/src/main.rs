mod tests;
mod b_tree;
use b_tree::*;
fn main() {
    let input = vec![Some(2), Some(1), Some(3)];
    let len = input.len();
    let tree = create_b_tree(input, 0, len);
    let output = Solution::is_valid_bst(tree);
    assert!(output);
}

pub struct Solution;
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        fn validate(
            min: Option<i32>,
            root: Option<Rc<RefCell<TreeNode>>>,
            max: Option<i32>,
        ) -> bool {
            if root.is_none() {
                return true;
            }
            if let Some(node) = root {
                let n = node.borrow();
                if let Some(min) = min {
                    if n.val <= min {
                        return false;
                    }
                }
                if let Some(max) = max {
                    if n.val >= max {
                        return false;
                    }
                }

                return validate(min, n.left.clone(), Some(n.val)) // for left subtree curr_node.val will be the max value
                    && validate(Some(n.val), n.right.clone(), max); // for right subtree curr_node.val will be the min value
            }
            true
        }
        // An empty tree is a valid BTree
        if root.is_none() {
            return true;
        };
        validate(None, root, None)
    }
}
