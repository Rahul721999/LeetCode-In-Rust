fn main() {

    assert_eq!(Solution::is_palindrome("  ".to_string()), true);
    assert_eq!(Solution::is_palindrome("A man, a plan, a canal: Panama".to_string()), true);
    assert_eq!(Solution::is_palindrome("race a car".to_string()), false);
    assert_eq!(Solution::is_palindrome("race car".to_string()), true);
}


struct Solution;
impl Solution{
    pub fn is_palindrome(s: String) -> bool {
        let s = s
            .chars()
            .filter(|c| c.is_alphanumeric())
            .collect::<String>()
            .to_lowercase();
        let v = s.as_bytes();
        let len = s.len() as i32;
        if s.is_empty() {
            return true;
        }
        let (mut left, mut right) = (0,(len - 1) as usize);
        
        while left < right{
            let (l, r) =  (v[left], v[right]);
            if l == r{
                left += 1;
                right -= 1;
            }else{
                return false
            }
        }
        true
    }
}