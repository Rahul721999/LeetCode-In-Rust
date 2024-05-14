fn main() {
    assert_eq!(Solution::three_sum_closest(vec![-1,2,1,-4], 1), 2);
    assert_eq!(Solution::three_sum_closest(vec![0,0,0], 1), 0);
}

struct Solution;
impl Solution {
    pub fn three_sum_closest(mut nums: Vec<i32>, t: i32) -> i32 { // [-1,2,1,-4]
        let len = nums.len();
        nums.sort(); // [-4, -1, 1, 2]
        let mut closest = i32::MAX;


        for i in 0..len-2{
            
            let (mut j,mut k) = (i+1, len-1);
            
            while j < k {
                let sum = nums[i] + nums[j] + nums[k];
                if (closest - t).abs() > (sum-t).abs(){
                    closest = sum;
                }
                match sum{
                    s if s > t => { 
                        k -= 1;
                    },
                    s if s < t => { 
                        j+=1
                    },
                    _ => { 
                        return t
                    }
                };
            }
        }
        return closest
    }
}