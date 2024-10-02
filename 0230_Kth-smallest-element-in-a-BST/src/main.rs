mod b_tree;
use b_tree::*;
mod tests;
use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    let input = vec![Some(3), Some(1), Some(4), None, Some(2)];
    let len = input.len();
    let b_tree = create_b_tree(input, 0, len);

    let output = Solution::kth_smallest(b_tree, 1);
    assert_eq!(output, 1);

}

pub struct Solution;
impl Solution {
    pub fn kth_smallest(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> i32 {
        let mut n = 0;
        let mut stack = Vec::new();
        let mut curr = root.clone();

        while !stack.is_empty() || curr.is_some() {
            while let Some(node) = curr {
                stack.push(node.clone());
                curr = node.borrow().left.clone();
            }
            curr = stack.pop();
            if let Some(curr_node) = curr {
                let curr_borrow = curr_node.borrow();
                n += 1;

                if n == k {
                    return curr_borrow.val;
                }
                curr = curr_borrow.right.clone();
            }
        }
        panic!("k is out of bounds");
    }
}
