#![allow(unused)]
use std::cell::RefCell;
use std::rc::Rc;


pub type T = Option<Rc<RefCell<TreeNode>>>;

// macro rule to borrow the provided node..
#[macro_export]
macro_rules! borrow_node {($n:expr) =>{$n.as_ref().unwrap().borrow()}}


// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: T,
    pub right: T,
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

// fn to create Binary Tree
pub fn create_b_tree(
    v: Vec<Option<i32>>,
    index: usize,
    len: usize,
) -> T {
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

// fn to find the given node
pub fn find_node(root: T, val: i32) -> T{
    if let Some(node) = root{
        let node_borrow = node.borrow();
        
        // check if the curr node is the one
        if node_borrow.val == val{
            return Some(node.clone())
        }

        /* ------------------Recursively search for the node in left and right subtree------------------ */

        // check left & return if found
        let left_subtree = find_node(node_borrow.left.clone(), val);
        if left_subtree.is_some(){return left_subtree}


        // check right otherwise
        return find_node(node_borrow.right.clone(), val);
    }
    None
}