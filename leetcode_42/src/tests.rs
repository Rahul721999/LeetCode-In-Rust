#[cfg(test)]
use crate::Solution;
#[test]
fn test1(){
     let res = Solution::trap(vec![0,1,0,2,1,0,1,3,2,1,2,1]);
     assert_eq!(res, 6);
}
#[test]
fn test2(){
     let res = Solution::trap(vec![4,2,0,3,2,5]);
     assert_eq!(res, 9);
}