mod tests;

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
    v: Vec<i32>,
    index: usize,
    len: usize,
) -> Option<Rc<RefCell<TreeNode>>> {
    if index < len {
        let mut temp = TreeNode::new(v[index]);
        temp.left = create_b_tree(v.clone(), 2 * index + 1, len);
        temp.right = create_b_tree(v, 2 * index + 2, len);

        return Some(Rc::new(RefCell::new(temp)));
    }
    None
}

fn main() {
    let test_vec = vec![4, 2, 7, 1, 3, 6, 9];
    let len = test_vec.len();
    let input = create_b_tree(test_vec, 0, len);

    let expected_vec = vec![4,7,2,9,6,3,1];
    let len = expected_vec.len();
    let output = create_b_tree(expected_vec, 0, len);

    assert_eq!(output, Solution::invert_tree(input))
}

pub struct Solution;
impl Solution {
    pub fn invert_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
      if let Some(node) = root{
        let node = node.borrow();

        let new_node = TreeNode{
          val: node.val,
          left: Self::invert_tree(node.right.clone()),
          right: Self::invert_tree(node.left.clone())
        };
        return Some(Rc::new(RefCell::new(new_node)));
      }
      None
    }
}
