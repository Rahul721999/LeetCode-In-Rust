mod b_tree;
use b_tree::{TreeNode, create_b_tree};
use std::rc::Rc;
use std::cell::RefCell;


pub struct Solution;
impl Solution {
    pub fn diameter_of_binary_tree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut count = 0;
        Self::dfs(root, &mut count);
        count
    }
    pub fn dfs(root: Option<Rc<RefCell<TreeNode>>>, count: &mut i32) -> i32 {
    match root {
      None => 0,
      Some(root) => {
        let root = root.borrow();
        let left = Self::dfs(root.left.clone(), count);
        let right = Self::dfs(root.right.clone(), count);
        *count = core::cmp::max(*count, left + right);
        1 + core::cmp::max(left, right)
      }
    }
  }
}


fn main() {
    let test_vec = vec![1,2,3,4,5];
    let len = &test_vec.len();
    let tree = create_b_tree(test_vec, 0, *len);
    
    assert_eq!(Solution::diameter_of_binary_tree(tree), 3);
}