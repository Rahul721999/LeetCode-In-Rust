use std::collections::HashSet;

mod tests;
fn main() {
    let intput = vec![
        "test.email+alex@leetcode.com".to_string(),
        "test.e.mail+bob.cathy@leetcode.com".to_string(),
        "testemail+david@lee.tcode.com".to_string(),
    ];
    let output = Solution::num_unique_emails(intput);
    assert_eq!(output, 2);
}
pub struct Solution;
impl Solution {
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
