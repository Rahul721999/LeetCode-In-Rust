mod b_tree;
mod tests;
use b_tree::*;
use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    let input_vec = vec![
        Some(1),
        Some(2),
        Some(2),
        Some(3),
        Some(4),
        Some(4),
        Some(3),
    ];
    let len = input_vec.len();
    let input = create_b_tree(input_vec, 0, len);
    assert_eq!(Solution::is_symmetric(input), true);
}

pub struct Solution;
impl Solution {
    pub fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        fn recursive(
            left: Option<Rc<RefCell<TreeNode>>>,
            right: Option<Rc<RefCell<TreeNode>>>,
        ) -> bool {
            match (left, right) {
                (None, None) => true, // Both nodes are None
                (Some(left_node), Some(right_node)) => {
                    let left_borrow = left_node.borrow();
                    let right_borrow = right_node.borrow();

                    // Check if the values are the same and the subtrees are mirrors
                    left_borrow.val == right_borrow.val // value of the both node should be same
                        && recursive(left_borrow.left.clone(), right_borrow.right.clone()) // left.left == right.right
                        && recursive(left_borrow.right.clone(), right_borrow.left.clone()) // left.right == right.left
                }
                _ => false, // One node is None and the other is not
            }
        }
        if let Some(node) = root {
            let node = node.borrow();
            recursive(node.left.clone(), node.right.clone())
        } else {
            true
        }
    }
}
