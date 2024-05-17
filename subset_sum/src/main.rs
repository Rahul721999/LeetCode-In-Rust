fn main() {
    assert!(!Solution::subset_sum(&vec![3, 34, 4, 12, 5, 2], 0, 30));
    assert!(Solution::subset_sum(&vec![3, 34, 4, 12, 5, 2], 0, 9));
    assert!(Solution::subset_memoized(&vec![3, 34, 4, 12, 5, 2], 0, 9));
    assert!(!Solution::subset_memoized(&vec![3, 34, 4, 12, 5, 2], 0, 30));
    assert!(Solution::subset_memoize_1d(&vec![1,2,3], 4));
    assert!(!Solution::subset_memoize_1d(&vec![3, 34, 4, 12, 5, 2],  30));
}


pub struct Solution;
impl Solution {
    pub fn subset_sum(v: &Vec<i32>, index: usize, sum: i32) -> bool{
        if index > v.len() - 1 && sum!= 0{return false}
        else if sum == 0 || v[index] == sum {return true}
        
        if index <= v.len() && v[index] <= sum{
            return Self::subset_sum(v, index+1, sum-v[index]);
        }
        Self::subset_sum(v, index+1, sum)
    }

    pub fn subset_memoized(v: &Vec<i32>,index: usize, sum: i32) -> bool{
        if sum == 0 {return true}
        let len = v.len() + 1;
        let mut dp: Vec<Vec<Option<bool>>> = vec![vec![None; sum as usize + 1]; len + 1];
        
        fn helper(v: &Vec<i32>,index: usize, sum: i32, dp: &mut Vec<Vec<Option<bool>>>) -> bool{
            if sum == 0 {return true}
            if index >= v.len(){return false}
            if let Some(result) = dp[index][sum as usize]{
                return result
            }
            if index <= v.len() && v[index] <= sum{
                dp[index][sum as usize] = Some(helper(v, index + 1, sum - v[index], dp));
            }else{
                dp[index][sum as usize] = Some(helper(v, index + 1, sum, dp));
            }
            if let Some(result) = dp[index][sum as usize]{
                return result; 
            }false
        }

        helper(v, index, sum,  &mut dp)
    }
    pub fn subset_memoize_1d(v: &Vec<i32>, sum: i32) -> bool {
        if sum == 0 {
            return true;
        }
        
        let mut dp = vec![false; sum as usize + 1];
        dp[0] = true;
    
        for &num in v {
            dbg!(&num);
            for j in (num as usize..=sum as usize).rev() {
                dbg!(j);
                if dp[j - num as usize] {
                    dbg!(j - num as usize, dp[j - num as usize]);
                    dp[j] = true;
                }
            }
        }
    
        dp[sum as usize]
    }
}

