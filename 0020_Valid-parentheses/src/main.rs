mod tests;
fn main() {
    let output = Solution::is_valid("[{}".to_string());
    assert_eq!(output, false);
}
pub struct Solution;
impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut stack: Vec<char> = Vec::new();

        for char in s.chars() {
            match char{
                '(' | '{' | '[' => { stack.push(char) },
                ')' => {
                        if stack.pop() != Some('(') {return false}
                    },
                '}' => {
                    if stack.pop() != Some('{'){return false}
                },
                ']' => {
                    if stack.pop() != Some('[') {return false}
                }

                _ => return false
            }
        }
        stack.is_empty()
    }
}
