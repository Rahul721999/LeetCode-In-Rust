fn main() {
    let mut nums = vec![3, 2, 2, 3];
    let res = Solution::remove_element(&mut nums, 3);
    assert_eq!(res, 2);
}

pub struct Solution;

impl Solution {
    // Solution 1 | (1ms): Brutte Force
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        let mut i = 0;
        let mut j = 0;

        while i < nums.len() {
            if nums[i] != val {
                nums[j] = nums[i];
                j += 1;
            }
            i += 1;
        }

        nums.truncate(j);

        j as i32
    }

    // Solution 2 | (0ms): retain element method in Rust
    pub fn retain_element_method(nums: &mut Vec<i32>, val: i32) -> i32 {
        nums.retain(|x| *x != val);
        nums.len() as i32
    }

    // Solution 3 | (0ms): swap_remove method in Rust
    pub fn swap_remove_method(nums: &mut Vec<i32>, val: i32) -> i32{
        for i in (0..nums.len()).rev() {
            if nums[i] == val{
                nums.swap_remove(i);
            }
        }
        nums.len() as i32
    } 
}

#[cfg(test)]
mod tests {
    #[cfg(test)]
    use crate::Solution;
    #[test]
    fn test1() {
        let res = Solution::remove_element(&mut vec![3, 2, 2, 3], 3);
        assert_eq!(res, 2)
    }
    #[test]
    fn test2() {
        let res = Solution::remove_element(&mut vec![0, 1, 2, 2, 3, 0, 4, 2], 2);
        assert_eq!(res, 5)
    }
    #[test]
    fn test3() {
        let res =
            Solution::remove_element(&mut vec![0, 1, 2, 2, 3, 0, 4, 2, 2, 2, 2, 2, 2, 2, 2, 2], 2);
        assert_eq!(res, 5)
    }
    #[test]
    fn test4() {
        let res = Solution::remove_element(&mut vec![], 1);
        assert_eq!(res, 0)
    }
    #[test]
    fn test5() {
        let res = Solution::remove_element(&mut vec![1], 1);
        assert_eq!(res, 0)
    }
    #[test]
    fn test6() {
        let res = Solution::remove_element(&mut vec![2], 3);
        assert_eq!(res, 1)
    }
}
