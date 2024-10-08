mod tests;
use helper::b_tree::*;
use std::cell::RefCell;
use std::rc::Rc;
fn main() {
    let vec = vec![Some(1), Some(2), Some(3)];
    let len = vec.len();
    let tree = create_b_tree(vec, 0, len);
    let output = Solution::has_path_sum(tree, 5);
    assert!(!output);
}

pub struct Solution;
impl Solution {
    pub fn has_path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> bool {
        fn recursive(root: T, target_sum: i32, mut curr_sum: i32) -> bool {
            if let Some(node) = root {
                let node = node.borrow();
                curr_sum += node.val;
                let left = node.left.clone();
                let right = node.right.clone();
                // If it's a leaf node, check if the sum matches the target sum
                if node.left.is_none() && node.right.is_none() {
                    return curr_sum == target_sum;
                }
                recursive(left, target_sum, curr_sum) || recursive(right, target_sum, curr_sum)
            } else {
                false
            }
        }
        recursive(root, target_sum, 0)
    }
}
