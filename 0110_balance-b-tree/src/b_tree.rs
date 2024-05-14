// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

use std::cell::RefCell;
use std::rc::Rc;
// Vec = [1,2,2,3,3,0,0,4,4]
pub fn create_b_tree(v: &Vec<i32>, len: usize, index: usize) -> Option<Rc<RefCell<TreeNode>>> {
    if index < len {
        let mut root = TreeNode::new(v[index]);
        if root.val != 0 {
            root.left = create_b_tree(v, len, 2 * index + 1);
            root.right = create_b_tree(v, len, 2 * index + 2);
            return Some(Rc::new(RefCell::new(root)));
        }
        return None;
    }
    None
}

//         1
//        / \
//       2   2
//      / \ 
//     3   3
//    / \
//   4   4