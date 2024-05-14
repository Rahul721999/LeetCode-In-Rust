fn main() {
    let strs = vec!["flower".to_string(),"flow".to_string(),"flight".to_string()];
    
    assert_eq!(longest_common_prefix(strs), "fl");
}

pub fn longest_common_prefix(strs: Vec<String>) -> String {
        match strs.is_empty(){
            true => return "".to_string(),
            _ => {
                // create an iterator and then use reduce method to iterate over the 
                strs.into_iter().reduce(|first_str,second_str|{
                    // take chars from both string and make tuple with .zip() method
                    first_str.chars()
                        .zip(second_str.chars())
                        // take until the condition satisfies
                        .take_while(|(f,s)| f==s)
                        // iter with .map() method
                        .map(|(z,_)| z)
                        // transform the iterator into a collection..
                        .collect()
                }).unwrap()
            }
        }
    }