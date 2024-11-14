use std::collections::HashSet;

mod tests;
fn main() {
    // calling Not Optimized solution
    let intput = vec![
        "test.email+alex@leetcode.com".to_string(),
        "test.e.mail+bob.cathy@leetcode.com".to_string(),
        "testemail+david@lee.tcode.com".to_string(),
    ];
    let output = Solution::num_unique_emails(intput);
    assert_eq!(output, 2);

    // calling optimized solution
    let intput = vec![
        "test.email+alex@leetcode.com".to_string(),
        "test.e.mail+bob.cathy@leetcode.com".to_string(),
        "testemail+david@lee.tcode.com".to_string(),
    ];
    let output = Solution::optimized(intput);
    assert_eq!(output, 2);
}
pub struct Solution;
impl Solution {
    // optimized solution
    pub fn optimized(emails: Vec<String>) -> i32 {
        let mut set: HashSet<String> = HashSet::new();
        for email in emails {
            let mut pointer = 0_usize;
            let mut slice = String::new();
            while let Some(ch) = email.chars().nth(pointer).as_mut() {
                if *ch == '+' || *ch == '@' {
                    break;
                };
                if *ch != '.' {
                    slice.push(*ch);
                }
                pointer += 1;
            }

            while let Some(ch) = email.chars().nth(pointer).as_mut() {
                if *ch == '@' {
                    break;
                }
                pointer += 1;
            }

            let domain = &email[pointer..];
            slice.push_str(domain);
            
            set.insert(slice.to_string());
        }

        set.len() as i32
    }

    // not optimized...
    pub fn num_unique_emails(emails: Vec<String>) -> i32 {
        let mut count = HashSet::new();
        for email in emails {
            let s: Vec<&str> = email.split("@").collect();
            let mut local = s[0];
            let domain = s[1];

            local = local.split("+").nth(0).unwrap();
            let mut s = local.replace(".", "");
            s.push_str("@");
            s.push_str(domain);
            count.insert(s);
        }
        println!("{count:?}");
        count.len() as i32
    }
}
