mod tests;

use helper::b_tree::*;
use std::cell::RefCell;
use std::rc::Rc;
fn main() {
    let input = vec![-10, -3, 0, 5, 9];
    let expected_output = create_b_tree(
        vec![Some(0), Some(-3), Some(9), Some(-10), None, Some(5)],
        0,
        6,
    );
    let acceptable_output = create_b_tree(
        vec![Some(0),Some(-10),Some(5),None,Some(-3),None,Some(9)],
         0, 7);
    let res = Solution::sorted_array_to_bst(input);
    
    if res != expected_output{
        assert_eq!(res, acceptable_output);
    }else{
        assert_eq!(res, expected_output);
    }
}
pub struct Solution;

impl Solution {
    pub fn sorted_array_to_bst(nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        fn helper(nums: &[i32], l: i32, r: i32) -> T {
            if l > r {
                return None;
            }
            let mid = (l + r) / 2;
            let mut root = TreeNode::new(nums[mid as usize]);
            root.left = helper(nums, l, mid - 1);
            root.right = helper(nums, mid + 1, r);

            Some(Rc::new(RefCell::new(root)))
        }
        helper(&nums, 0, nums.len() as i32 - 1)
    }
}
