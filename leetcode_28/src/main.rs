mod tests;
fn main() {
    assert_eq!(Solution::str_str("leetcode".to_string(), "leeto".to_string()), -1);
    println!("");
}


struct Solution;
impl Solution{
    pub fn str_str(haystack: String, needle: String) -> i32 {
        let hay_len = haystack.len();
        let need_len = needle.len();

        if need_len == 0{return 0};
        
        if need_len > hay_len{
            return -1;
        }

        for i in 0..=(hay_len-need_len){
            if haystack[i..(i+need_len)] == needle{
                return i as i32;
            }
        }

        -1
    }
}

