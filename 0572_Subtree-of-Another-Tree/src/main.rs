mod b_tree;
mod tests;
use std::{cell::RefCell, rc::Rc};

use b_tree::*;
fn main() {
    let tree1 = vec![Some(3), Some(4), Some(5), Some(1), Some(2)];
    let tree2 = vec![Some(4), Some(1), Some(2)];

    let p = create_b_tree(tree1, 0, 5);
    let q = create_b_tree(tree2, 0, 3);

    assert_eq!(Solution::is_subtree(p, q), true);
    
}

pub struct Solution;
impl Solution {
    pub fn is_subtree(
        root: Option<Rc<RefCell<TreeNode>>>,
        sub_root: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        if sub_root.is_none() {
            return true;
        }
        if root.is_none() {
            return false;
        }

        fn same_tree(
            node1: Option<Rc<RefCell<TreeNode>>>,
            node2: Option<Rc<RefCell<TreeNode>>>,
        ) -> bool {
            match (node1, node2) {
                (None, None) => true,
                (Some(n1), Some(n2)) => {
                    let n1 = n1.borrow();
                    let n2 = n2.borrow();

                    n1.val == n2.val
                        && same_tree(n1.left.clone(), n2.left.clone())
                        && same_tree(n1.right.clone(), n2.right.clone())
                }
                _ => false,
            }
        }

        if same_tree(root.clone(), sub_root.clone()) {
            return true;
        };

        let r = root.as_ref().unwrap().borrow();
        Solution::is_subtree(r.left.clone(), sub_root.clone())
            || Solution::is_subtree(r.right.clone(), sub_root.clone())
    }
}
