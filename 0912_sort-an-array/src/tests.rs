#[cfg(test)]
use crate::Solution;
#[test]
fn test1() {
     let res = Solution::quick_sort_array(vec![5,1,0,0]);
     assert_eq!(res, vec![0,0,1,5])
}
#[test]
fn test2() {
     let res = Solution::quick_sort_array(vec![5,2,3,1]);
     assert_eq!(res, vec![1,2,3,5])
}
#[test]
fn test3() {
     let res = Solution::quick_sort_array(vec![5,1,1,2,0,0]);
     assert_eq!(res, vec![0,0,1,1,2,5])
}
