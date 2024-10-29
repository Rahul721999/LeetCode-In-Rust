fn main() {
    let intput = String::from("aba");
    assert!(Solution::valid_palindrome(intput));

    let intput = String::from("abca");
    assert!(Solution::valid_palindrome(intput));

    let intput = String::from("abc");
    assert!(!Solution::valid_palindrome(intput));
}
pub struct Solution;
impl Solution {
    pub fn valid_palindrome(s: String) -> bool {
        let s = s.as_bytes();
        let mut l = 0;
        let mut r = s.len() - 1;

        if s.len() <= 1 {
            return true;
        }

        while l < r {
            if s[l] != s[r] {
                let (skip_l, skip_r) = (&s[l + 1..r + 1], &s[l..r]);
                return skip_r == skip_r.iter().rev().copied().collect::<Vec<u8>>()
                    || skip_l == skip_l.iter().rev().copied().collect::<Vec<u8>>();
            }

            l += 1;
            if r > 0 {
                r -= 1;
            }
        }
        true
    }
}
