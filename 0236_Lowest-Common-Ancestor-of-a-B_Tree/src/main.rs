mod tests;
mod b_tree;
use b_tree::*;


fn main() {
    let tree = vec![Some(3),Some(5),Some(1),Some(6),Some(2),Some(0),Some(8),None,None,Some(7),Some(4)];
    let len = tree.len();
    let root = create_b_tree(tree, 0, len);
    let p = find_node(root.clone(), 5);
    let q = find_node(root.clone(), 1);
    
    let output = Solution::lowest_common_ancestor(root.clone(), p, q);

    let expected_output = find_node(root.clone(), 3);
    assert_eq!(output, expected_output);
}

pub struct Solution;
impl Solution {
    pub fn lowest_common_ancestor(
        root: T,
        p: T,
        q: T,
    ) -> T {
        if root.is_none() {return None}

        // check if the current node is p or q
        if root == p || root == q{return root}

        // Rrecursively search left and right subtree
        let root_node = root.clone();
        let root_node = borrow_node!(root_node);
        let left = Self::lowest_common_ancestor(root_node.left.clone(), p.clone(), q.clone());
        let right = Self::lowest_common_ancestor(root_node.right.clone(), p.clone(), q.clone());
        
        // if both subtree have a result, then curr_node is the LCA
        if left.is_some() && right.is_some(){return root}

        // if one of the subtree has the result
        if left.is_some(){return left}
        else {return right}
    }
}
