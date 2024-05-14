use std::cell::RefCell;
use std::rc::Rc;

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

pub fn create_b_tree(
    v: Vec<Option<i32>>,
    index: usize,
    len: usize,
) -> Option<Rc<RefCell<TreeNode>>> {
    if index < len {
        if let Some(val) = v[index] {
            let mut temp = TreeNode::new(val);
            temp.left = create_b_tree(v.clone(), 2 * index + 1, len);
            temp.right = create_b_tree(v, 2 * index + 2, len);
            return Some(Rc::new(RefCell::new(temp)));
        }
    }
    None
}
