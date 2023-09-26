mod tests;

fn main() {
    let res = Solution::is_match("aa".to_string(), "a*".to_string());
    assert_eq!(res,true);
    println!("{}", res);
}

struct Solution;
impl Solution {
    ///
    ///  recursive solution...(Very slow)
    ///
    pub fn is_match(s: String, p: String) -> bool {
        let (i, j) = (s.len(), p.len());
        // base case

        /* when the both the given string is empty */
        if i <= 0 && i <= 0 {
            return true;
        };

        /* when pattern string is empty but the i/p is not */
        if i > 0 && j == 0 {
            return false;
        };

        /* match s[0] == p[0] || p[1] == '.' */
        let first_match = i > 0 && (s.char_at(0) == p.char_at(0) || p.char_at(0) == Some('.'));
        
        /* check if the next char is '*' */
        if j >= 2 && p.char_at(1) == Some('*') {

            // Two options: 0 occurrences || 1 or more occurrences..
            return Solution::is_match(s[..].to_string(), p[2..].to_string()) || 
                first_match && Solution::is_match(s[1..].to_string(), p)
        }
        else{
            return first_match && Solution::is_match(s[1..].to_string(), p[1..].to_string());
        }

    }
}

trait MyTrait {
    fn char_at(&self, t: usize) -> Option<char>;
}

impl MyTrait for String {
    fn char_at(&self, t: usize) -> Option<char> {
        if let Some(c) = &self.chars().nth(t.try_into().unwrap()) {
            return Some(*c);
        } else {
            return None;
        }
    }
}

