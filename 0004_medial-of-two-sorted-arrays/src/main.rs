
fn main() {
    assert_eq!(Solution::find_median_sorted_arrays(vec![1,3], vec![2]), 2.0000);
    assert_eq!(Solution::find_median_sorted_arrays(vec![1,2], vec![3,4]), 2.5);
    // Solution::find_median_sorted_arrays(vec![1,2], vec![3,4,5]);
}

struct Solution;
impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        use std::i32::MAX;
        let mut new_arr: Vec<i32> = vec![];

        // marge two array into one.
        let total_length = nums1.len() + nums2.len();
        let med = total_length / 2;
        let (mut p1,mut p2) = (0,0);
        while p1+p2 <= med{
            let num1 = nums1.get(p1).unwrap_or(&MAX);
            let num2 = nums2.get(p2).unwrap_or(&MAX);

            if num1 < num2{
                new_arr.push(*num1);
                p1 +=1;
            }else{
                new_arr.push(*num2);
                p2+=1;
            }
        }

        // check if total_len is odd or even
        
        if total_length % 2 == 0{
            return (new_arr[med] + new_arr[med-1]) as f64/2.0;
        }else{
            return new_arr[med] as f64;
        }
        
    }
}

// [1,2,3] + [4,5]=> [1,2,3,4,5] => Ans. 3.0000
// [1,2] + [3]=> [1,2,3] => Ans. 2.0000
// [1,2] + [3,4]=> [1,2,3,4] => 3+2 = 5/2 => Ans. 2.5
