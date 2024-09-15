mod tests;
fn main() {
    // test1:
    assert_eq!(vec!["()"], Solution::generate_parenthesis(1));

    // test2:
    let expected = vec!["((()))","(()())","(())()","()(())","()()()"];
    assert_eq!(expected, Solution::generate_parenthesis(3));
}

pub struct Solution;
impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        let mut res: Vec<String> = Vec::new();
        let mut stack =  "".to_string();

        fn back_tracking(opened: i32, closed: i32, res: &mut Vec<String>, stack: &mut String, n: i32) {
            /* Three diff steps to follow */
            // 1. If open == closed == n { add it to the result}
            if opened == n && closed == n{
                res.push(stack.to_owned());
                return
            }

            // 2. Add open parentheses "(" if open < n
            if opened < n{
              stack.push('(');
              back_tracking(opened+1, closed, res, stack, n);
              stack.pop();
            }
            // 3. Add close parentheses ")" if close < open
            if closed < opened{
                stack.push(')');
                back_tracking(opened, closed+1, res, stack, n);
                stack.pop();
            }
            
        }

        back_tracking(0, 0, &mut res, &mut stack, n);
        res
    }
}
