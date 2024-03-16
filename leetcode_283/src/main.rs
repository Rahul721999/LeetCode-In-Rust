fn main() {
    let mut nums = vec![0, 1, 0, 3, 12];
    Solution::move_zeroes(&mut nums);
    assert_eq!(nums, vec![1, 3, 12, 0, 0]);
}

struct Solution;
impl Solution {
    pub fn move_zeroes(nums: &mut Vec<i32>) {
        use std::cell::RefCell;
        let len = nums.len();
        if len > 1 {
            let nums_ref = RefCell::new(nums);
            for index in (0..len).rev() {
                let mut borrowed_nums = nums_ref.borrow_mut();
                if borrowed_nums[index] == 0 {
                    borrowed_nums.remove(index);
                    borrowed_nums.push(0);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_move_zeroes_basic() {
        let mut nums = vec![0, 1, 0, 3, 12];
        Solution::move_zeroes(&mut nums);
        assert_eq!(nums, vec![1, 3, 12, 0, 0]);
    }

    #[test]
    fn test_move_zeroes_no_zeros() {
        let mut nums = vec![1, 2, 3, 4, 5];
        Solution::move_zeroes(&mut nums);
        assert_eq!(nums, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_move_zeroes_all_zeros() {
        let mut nums = vec![0, 0, 0, 0, 0];
        Solution::move_zeroes(&mut nums);
        assert_eq!(nums, vec![0, 0, 0, 0, 0]);
    }

    #[test]
    fn test_move_zeroes_mix_zeros() {
        let mut nums = vec![0, 1, 0, 3, 0, 12];
        Solution::move_zeroes(&mut nums);
        assert_eq!(nums, vec![1, 3, 12, 0, 0, 0]);
    }

    #[test]
    fn test_move_zeroes_empty() {
        let mut nums: Vec<i32> = vec![];
        Solution::move_zeroes(&mut nums);
        assert_eq!(nums, vec![]);
    }
    #[test]
    fn test_move_zeroes_one_element() {
        let mut nums: Vec<i32> = vec![0];
        Solution::move_zeroes(&mut nums);
        assert_eq!(nums, vec![0]);
    }
    #[test]
    fn test_with_zero_and_one() {
        let mut nums: Vec<i32> = vec![0,0,1];
        Solution::move_zeroes(&mut nums);
        assert_eq!(nums, vec![1,0,0]);
    }
}
