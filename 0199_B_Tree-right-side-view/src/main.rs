mod tests;
mod b_tree;
use b_tree::*;

use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

fn main() {
    let input = vec![Some(3),Some(9),Some(20),None,None,Some(15),Some(7)];
    let len = input.len();
    let b_tree = create_b_tree(input, 0, len);
    let expected_output = vec![3,20,7];

    let output = Solution::right_side_view(b_tree);
    assert_eq!(output, expected_output);
}

pub struct Solution;
impl Solution {
    pub fn right_side_view(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut res: Vec<i32> = Vec::new();
        let mut queue = VecDeque::new();

        if let Some(r) = root{
            queue.push_back(r);
        }

        while !queue.is_empty(){
            let level_size = queue.len();

            for i in 0..level_size{
                let node = queue.pop_front().unwrap();
                let n = node.borrow();

                // push the value of the current node.
                if i==0{res.push(n.val);}

                if let Some(right) = n.right.clone(){
                    queue.push_back(right);
                }
                if let Some(left) = n.left.clone(){
                    queue.push_back(left);
                }
            }
        }
        res
    }
}
