use std::collections::BinaryHeap;

fn main() {
    assert_eq!(Solution::top_k_frequent(vec![1,1,1,2,2,3,4], 2), vec![1,2]);
    assert_eq!(Solution::top_k_frequent(vec![4,1,-1,2,-1,2,3], 2), vec![-1,2]);
}


struct Solution;
impl Solution{
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32>{
        let len = nums.len();
        if len == k as usize{return nums};
        // create a HashMap for keep track of counting
        use std::collections::HashMap;
        let mut map = HashMap::<i32,i32>::with_capacity(len);

        // Store the count of each values in nums arr
        // key=> numbers && values => occurrence of the numbers
        for n in nums{
            *map.entry(n).or_insert(0) += 1;
        };

        let mut res = Vec::<i32>::new();
        let mut heap = BinaryHeap::<(i32,i32)>::new();
        for(key, val) in &map{
            heap.push((*val, *key));
        }
        
        while let Some((_, val)) = heap.pop(){
            if res.len() < k as usize{res.push(val);}
        }
        res.sort();
        res
    }
}


#[cfg(test)]
mod tests{
    use crate::Solution;
    #[test]
    fn test1() {
        assert_eq!(Solution::top_k_frequent(vec![1,2,1,3,1,2,4], 2), vec![1,2]);
    }
    #[test]
    fn test2() {
        assert_eq!(Solution::top_k_frequent(vec![1], 1), vec![1]);
    }
    #[test]
    fn test3() {
        assert_eq!(Solution::top_k_frequent(vec![3,0,1,0], 1), vec![0]);
    }
    #[test]
    fn test4() {
        assert_eq!(Solution::top_k_frequent(vec![4,1,-1,2,-1,2,3], 2), vec![-1,2]);
    }
}
