fn main() {
    println!("Hello, world!");
    let res = longest_palindrom("racecar".to_string());
    println!("{}", res);
}

pub fn longest_palindrom(s: String) -> String {
    if s.len() < 1 {
        return "".to_string();
    }
    
    let s: Vec<char> = s.chars().collect();

    let len = s.len();
    let mut start = 0;
    let mut end = 0;

    for i in 0..len{
        let mut left = i;
        let mut right = i;

        // 0 1 2 3 4 5 6
        // c b b d
        while right + 1 < len && s[right + 1] == s[left]{
            right +=1;
        }

        // 0 1 2 3 4 5 6 
        // r a c e c a r
        while right + 1 < len && left > 0 && s[right+1] == s[left-1]{
            right +=1;
            left -=1;
        }
        if right - left > end - start{
            start = left;
            end = right;
        }
    }
    s[start..=end].iter().collect()
}

#[cfg(test)]
mod test {
    use crate::longest_palindrom;
    #[test]
    fn test1() {
        assert_eq!(
            longest_palindrom(String::from("babad")),
            String::from("bab")
        );
    }
    #[test]
    fn test2() {
        assert_eq!(longest_palindrom(String::from("cbbd")), String::from("bb"));
    }
    #[test]
    fn test3() {
        assert_eq!(
            longest_palindrom(String::from("racecar")),
            String::from("racecar")
        );
    }
}
