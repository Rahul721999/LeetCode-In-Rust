mod tests;
use helper::b_tree::*;
use std::cell::RefCell;
use std::rc::Rc;
fn main() {
    let input = vec![Some(1), None, Some(2), None, None, Some(3)];
    let len = input.len();
    let tree = create_b_tree(input, 0, len);
    let output = Solution::inorder_traversal(tree);
    println!("{output:?}");
    let expeted = vec![1, 3, 2];

    assert_eq!(output, expeted);
}

pub struct Solution;
impl Solution {
    pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut res: Vec<i32> = Vec::new();
        fn recursive(root: T, res: &mut Vec<i32>) -> Vec<i32> {
            if let Some(node) = root {
                let n = node.borrow();
                let left = n.left.clone();
                let right = n.right.clone();

                recursive(left, res);
                res.push(n.val);
                recursive(right, res);
            }
            return res.to_vec()
        }
        recursive(root, &mut res)
    }
}
