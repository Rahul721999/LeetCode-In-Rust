mod tests;
fn main() {
    let nums = vec![2, 20, 4, 10, 3, 4, 5];
    let result = Solution::longest_consecutive(nums);
    println!("Result: {}", result);
}
pub struct Solution;
impl Solution {
    /*
        Time coplexity is:
            shorting: O(N*LogN)
            while loop: O(N)
            total: O(N*LogN)

        Space complexity:
            extra array: O(N)
    */
    pub fn longest_consecutive_bf_sln(nums: Vec<i32>) -> i32 {
        let mut nums = nums.clone();
        nums.sort();

        let len = nums.len();

        let mut curr = 0;
        let mut max_so_far = 0;
        let mut count = 1;

        while curr < len - 1 {
            if nums[curr] == nums[curr + 1] {
                curr += 1;
                continue;
            }
            if nums[curr] + 1 == nums[curr + 1] {
                count += 1;
            } else {
                count = 1;
            }

            max_so_far = count.max(max_so_far);
            curr += 1;
        }

        max_so_far
    }

    /* Better Approach (use set): "If (ele - 1) doesn't exists start counting, if exists count += 1"

        Time Complexity:
            Initialize set: O(N)
            for Loop: O(N)
            total: O(N)
        Space complexity:
            set: O(N)
     */

    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        use std::collections::HashSet;
        let set: HashSet<i32> = nums.into_iter().collect();

        let mut longest = 0;
        for &num in &set {
            if !set.contains(&(num - 1)) {
                let mut curr_num = num;
                let mut curr_streak = 1;

                while set.contains(&(curr_num + 1)) {
                    curr_streak += 1;
                    curr_num += 1;

                    longest = longest.max(curr_streak);
                }
            }
        }

        longest
    }
}
