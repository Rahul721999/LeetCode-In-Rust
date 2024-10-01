mod b_tree;
use b_tree::*;
mod tests;
use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    let input = vec![Some(3), Some(1), Some(4), Some(3), None, Some(1), Some(5)];
    let len = input.len();
    let b_tree = create_b_tree(input, 0, len);

    let output = Solution::good_nodes(b_tree);
    assert_eq!(output, 4);
}

pub struct Solution;
impl Solution {
    pub fn good_nodes(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn recursive(curr: Option<Rc<RefCell<TreeNode>>>, max: i32) -> i32 {
            if let Some(node) = curr {
                let n = node.borrow();
                let new_max = max.max(n.val);

                let mut count = if n.val >= max {1} else {0};

                count += recursive(n.left.clone(), new_max);
                count += recursive(n.right.clone(), new_max);
                return count;
            }
            return 0;
        }
        if let Some(curr) = root {
            let n = curr.borrow();
            return recursive(Some(curr.clone()), n.val);
        }
        return 0;
    }
}
