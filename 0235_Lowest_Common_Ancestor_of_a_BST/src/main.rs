mod tests;
mod b_tree;
use b_tree::*;
use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    let tree = vec![
        Some(6),
        Some(2),
        Some(8),
        Some(0),
        Some(4),
        Some(7),
        Some(9),
        None,
        None,
        Some(3),
        Some(5),
    ];
    let len = tree.len();
    let root = create_b_tree(tree, 0, len);
    let p = find_node(root.clone(), 2);
    let q = find_node(root.clone(), 8);
    
    let output = Solution::lowest_common_ancestor(root.clone(), p, q);

    // let expected_output = find_node(root.clone(), 6);
    assert_eq!(output, root);
}

pub struct Solution;
impl Solution {
    pub fn lowest_common_ancestor(
        root: Option<Rc<RefCell<TreeNode>>>,
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if root.is_none() || p.is_none() || q.is_none() {
            return None;
        }
        let p = p.unwrap();
        let q = q.unwrap();
        let p_borrow = p.borrow();
        let q_borrow = q.borrow();

        let mut curr = root;
        while let Some(node) = curr.clone() {
            let node_borrow = node.borrow();
            let curr_val = node_borrow.val;
            
            // if both p and q are greater than curr_val, go to the right subtree
            if p_borrow.val > curr_val && q_borrow.val > curr_val {
                curr = node_borrow.right.clone();
            } 
            // if both p and q are less than curr_val, go to the left subtree
            else if p_borrow.val < curr_val && q_borrow.val < curr_val {
                curr = node_borrow.left.clone();
            }
            else{
                return Some(node.clone())
            }
        }
        None
    }
}
