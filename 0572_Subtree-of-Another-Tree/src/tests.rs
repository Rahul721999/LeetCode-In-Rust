#![allow(unused)]
use crate::{Solution, create_b_tree};
#[test]
fn test1() {
    let tree1 = vec![Some(3), Some(4), Some(5), Some(1), Some(2)];
    let tree2 = vec![Some(4), Some(1), Some(2)];

    let p = create_b_tree(tree1, 0, 5);
    let q = create_b_tree(tree2, 0, 3);

    assert_eq!(Solution::is_subtree(p, q), true);
}


#[test]
fn test2() {
    let tree1 = vec![
        Some(3),
        Some(4),
        Some(5),
        Some(1),
        Some(2),
        None,
        None,
        None,
        None,
        Some(0),
    ];
    let tree2 = vec![Some(4), Some(1), Some(2)];

    let p = create_b_tree(tree1, 0, 5);
    let q = create_b_tree(tree2, 0, 3);

    assert_eq!(Solution::is_subtree(p, q), true);
}
#[test]
fn test3() {
    let tree1 = vec![
        Some(1), None, Some(1), None, Some(1), None, Some(1), None, Some(1), None, 
        Some(1), None, Some(1), None, Some(1), None, Some(1), None, Some(1), None, 
        Some(1), None, Some(1), Some(2)
    ];
    let tree1_len = tree1.len();
    let tree2 = vec![
        Some(1), None, Some(1), None, Some(1), None, Some(1), None, Some(1), None,
        Some(1), Some(2)
    ];
    let tree2_len = tree2.len();


    let p = create_b_tree(tree1, 0, tree1_len);
    let q = create_b_tree(tree2, 0, tree2_len);

    assert_eq!(Solution::is_subtree(p, q), true);
}
