pub struct Solution;

impl Solution {
    pub fn eval_rpn(tokens: Vec<String>) -> i32 {
        let mut stack: Vec<i32> = Vec::new();

        let sign: Vec<String> = vec![
            String::from("+"),
            String::from("-"),
            String::from("*"),
            String::from("/"),
        ];

        fn apply_operation(value1: i32, value2: i32, operator: &str) -> i32 {
            match operator {
                "+" => value1 + value2,
                "-" => value1 - value2,
                "*" => value1 * value2,
                "/" => value1 / value2,
                _ => panic!("Operator not allowed"),
            }
        }

        for token in tokens {
            if !sign.contains(&token) {
                let token = token.trim().parse().expect("failed to parse string");
                stack.push(token);
            } else {
                let value2 = stack.pop().unwrap();
                let value1 = stack.pop().unwrap();
                let res = apply_operation(value1, value2, &token);
                stack.push(res)
            }
        }

        stack.pop().unwrap()
    }
}
fn main() {
    let tokens = vec![
        String::from("2"),
        String::from("1"),
        String::from("+"),
        String::from("3"),
        String::from("*"),
    ];
    let res = Solution::eval_rpn(tokens);
    assert_eq!(res, 9);
}
