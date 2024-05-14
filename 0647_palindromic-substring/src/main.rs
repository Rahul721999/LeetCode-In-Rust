fn main() {
    assert_eq!(Solution::count_substrings("aaa".to_string()), 6);
}

pub struct Solution;
impl Solution{
    pub fn count_substrings(s: String) -> i32 {
        let s_arr: Vec<char> = s.chars().collect();
        let mut count = 0;
        let len = s.len();
        // let (mut start, mut end) = (0,0);
        
        // Traverse the whole string...
        for i in 0..len{
            count += 1;
            let (mut left, mut right) = (i,i);
            
            // case1: repeating chars: "caaaab"
            while right+1<len && s_arr[right+1] == s_arr[left]{
                right +=1;
                count +=1;
            }

            // case2: for case
            while right+1<len && left>0 && s_arr[right+1] == s_arr[left-1]{
                right+=1;
                left-=1;
                count+=1;
            }
        }
        count
    }
}

#[cfg(test)]
mod test{
    use crate::Solution;
    #[test]
    fn test1(){
        assert_eq!(Solution::count_substrings("aaa".to_string()), 6);
    }
    #[test]
    fn test2(){
        assert_eq!(Solution::count_substrings("abc".to_string()), 3);
    }
}