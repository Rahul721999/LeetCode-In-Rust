fn main() {
    assert_eq!(
        Solution::daily_temperatures(vec![73, 74, 75, 71, 69, 72, 76, 73]),
        vec![1, 1, 4, 2, 1, 1, 0, 0]
    );
    assert_eq!(
        Solution::daily_temperatures(vec![30, 40, 50, 60]),
        vec![1, 1, 1, 0]
    );
    assert_eq!(
        Solution::daily_temperatures(vec![30, 60, 90]),
        vec![1, 1, 0]
    );
}

struct Solution;
impl Solution {
    pub fn daily_temperatures(temp: Vec<i32>) -> Vec<i32> {
        let len = temp.len();
        let mut res = vec![0; len];
        let mut stack = vec![len - 1];
        for i in (0..=len - 2).rev() {
            while !stack.is_empty() && temp[*stack.last().unwrap()] <= temp[i] {
                stack.pop();
            }
            if !stack.is_empty() {
                res[i] = (*stack.last().unwrap() as i32) - (i as i32);
            }
            stack.push(i);
        }
        res
    }
}
