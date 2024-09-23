mod b_tree;
use b_tree::*;

use std::rc::Rc;
use std::cell::RefCell;
fn main() {
    let tree1 = vec![Some(1), Some(2), Some(3)];
    let tree2 = tree1.clone();

    let p = create_b_tree(tree1, 0, 3);
    let q = create_b_tree(tree2, 0, 3);

    assert_eq!(Solution::is_same_tree(p, q), true);

    let tree1 = vec![Some(1), None, Some(2)];
    let tree2 = vec![Some(1), Some(2), None];

    let p = create_b_tree(tree1, 0, 3);
    let q = create_b_tree(tree2, 0, 3);

    assert_eq!(Solution::is_same_tree(p, q), false);
}

pub struct Solution;
impl Solution {
    pub fn is_same_tree(p: Option<Rc<RefCell<TreeNode>>>, q: Option<Rc<RefCell<TreeNode>>>) -> bool {
        fn recursive(node1: Option<Rc<RefCell<TreeNode>>>, node2: Option<Rc<RefCell<TreeNode>>>) -> bool{
            match (node1, node2){
               (None, None)  => true,
               (Some(n1), Some(n2)) => {
                    let n1 = n1.borrow();
                    let n2 = n2.borrow();

                    n1.val == n2.val
                        && recursive(n1.left.clone(), n2.left.clone())
                            && recursive(n1.right.clone(), n2.right.clone())
               }
               _ => false,
            }
        }
        recursive(p, q)
    }
}