use crate::Solution;

impl Solution{
     pub fn optimal_three_sum(nums : Vec<i32>) -> Vec<Vec<i32>>{
          let len = nums.len();
          if len < 3{
               return vec![]
          }
          use std::collections::HashSet;
          let mut res_set: HashSet<Vec<i32>> = HashSet::new();
          for i in 0..len-2{
               let mut container: HashSet<i32> = HashSet::new();
               for j in i+1..len-1{
                    let remain = -(nums[i]+nums[j]);
                    if container.contains(&remain){
                         let mut list = vec![nums[i], nums[j], remain];
                         list.sort();
                         res_set.insert(list);
                    }
                    container.insert(nums[j]);
               }
          }
          res_set.iter().map(|l| vec![l[0],l[1],l[2]]).collect()
     }
}

#[cfg(test)]
mod test {
    use crate::Solution;

    #[test]
    fn test1() {
        let result = Solution::optimal_three_sum(vec![-1, 0, 1, 2, -1, -4]);
        let expected = vec![vec![-1, -1, 2], vec![-1, 0, 1]];
        assert_eq!(result.len(), expected.len());
        assert!(result.iter().all(|val| expected.contains(val)));
    }
    #[test]
    fn test2() {
        let result = Solution::optimal_three_sum(vec![0, 0, 0]);
        let expected = vec![vec![0, 0, 0]];
        assert!(result.iter().all(|val| expected.contains(val)));
    }
    #[test]
    fn test3() {
        let result = Solution::optimal_three_sum(vec![0, 1, 1]);
        let expected = vec![];
        assert_eq!(result.len(), expected.len());
        assert!(result.iter().all(|val| expected.contains(val)));
    }
}