mod b_tree;
use b_tree::*;
mod tests;
use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    let preorder = vec![3,9,20,15,7];
    let inorder = vec![9,3,15,20,7];
    let tree = Solution::build_tree(preorder, inorder);

    let expected_vec = vec![Some(3), Some(9), Some(20), None, None, Some(15), Some(7)];
    let len = expected_vec.len();
    let expected_tree = create_b_tree(expected_vec, 0, len);

    assert_eq!(expected_tree, tree);
}

pub struct Solution;
impl Solution {
    pub fn build_tree(preorder: Vec<i32>, inorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        fn helper(preorder: &[i32], inorder: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
            if preorder.is_empty() || inorder.is_empty() {
                return None;
            }
    
            // The first element of preorder is always the root.
            let root_val = preorder[0];
    
            // Find the index of the root in inorder.
            let mid = inorder.iter().position(|&r| r == root_val).unwrap();
    
            // Build the left and right subtrees recursively.
            let left = helper(&preorder[1..mid + 1], &inorder[..mid]);
            let right = helper(&preorder[mid + 1..], &inorder[mid + 1..]);
    
            // Create the current node.
            let node = TreeNode {
                val: root_val,
                left,
                right,
            };
    
            // Return the current node wrapped in Rc and RefCell.
            Some(Rc::new(RefCell::new(node)))
        }
    
        // Start the recursive function.
        helper(&preorder, &inorder)
    }
    
}
