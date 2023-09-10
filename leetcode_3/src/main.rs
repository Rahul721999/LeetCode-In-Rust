use std::collections::HashSet;

fn main() {
    println!("run cargo test");
}


fn length_of_longest_substring(s: String) -> i32{
    let mut start = 0;
    let mut end = 0;
    let mut max = 0;
    let mut hashset: HashSet<char> = HashSet::new();

    while end < s.len() {
        if let Some(c) = s.char_at(end){
            if !hashset.contains(&c){
                hashset.insert(c);
                end = end+1;
                if hashset.len() > max{
                    max = hashset.len();
                };
            }else{
                let c = s.char_at(start).expect("failed to get the char at String.[Start]");
                hashset.remove(&c);
                start = start+1 ;
            }
        }
    }
    return max as i32;
}

trait MyTrait{
    fn char_at(&self,t: usize)-> Option<char>;
}

impl MyTrait for String{
    fn char_at(&self, t: usize) -> Option<char>{
        if let Some(c) = &self.chars().nth(t){
            return Some(*c)
        }
        else{
            return None
        }
    } 
}


#[cfg(test)]
mod test{
    use crate::length_of_longest_substring;

    #[test]
    fn test1(){
        assert_eq!(length_of_longest_substring(String::from("abcabcbb")), 3);
    }
    #[test]
    fn test2(){
        assert_eq!(length_of_longest_substring(String::from("bbbbb")), 1);
    }
    #[test]
    fn test3(){
        assert_eq!(length_of_longest_substring(String::from("pwwkew")), 3);
    }
}