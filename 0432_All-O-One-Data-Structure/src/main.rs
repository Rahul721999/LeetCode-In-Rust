use std::collections::HashMap;

#[derive(Debug)]
pub struct AllOne {
    key_count: HashMap<String, i32>,
    max_count: (i32, String),
    min_count: (i32, String),
}

impl AllOne {
    // Constructor
    fn new() -> Self {
        AllOne {
            key_count: HashMap::new(),
            max_count: (0, "".to_string()),
            min_count: (0, "".to_string()),
        }
    }

    // Increment the count of the key
    fn inc(&mut self, key: String) {
        // Check if the key exists
        let count = self.key_count.entry(key.clone()).or_insert(0);
        *count += 1;  // Increment the count

        // Update maxCount
        if *count > self.max_count.0 {
            self.max_count = (*count, key.clone());
        }

        // Update minCount
        if self.min_count.0 == 0 || *count < self.min_count.0 {
            self.min_count = (*count, key);
        }

        self.update_min_max_count();
    }


    fn dec(&mut self, key: String) {
        let count = self.key_count.entry(key.clone()).or_insert(0);
        *count -=1 ;

        // if count becomes 0, remove key from map
        if *count == 0{
            self.key_count.remove(&key);
        }

        // update maxCount & minCount
        self.update_min_max_count();
    }

    fn update_min_max_count(&mut self){
        // Reset max and min
        self.max_count = (0, "".to_string());
        self.min_count = (i32::MAX, "".to_string());

        for (key, &count) in &self.key_count {
            if count > self.max_count.0 {
                self.max_count = (count, key.clone());
            }
            if count < self.min_count.0 {
                self.min_count = (count, key.clone());
            }
        }

        // If no elements left, reset min_count to (0, "")
        if self.key_count.is_empty() {
            self.min_count = (0, "".to_string());
        }
    }

    fn get_max_key(&self) -> String {
        if self.max_count.0 == 0{
            "".to_string()
        }else{
            self.max_count.1.to_owned()
        }
    }

    fn get_min_key(&self) -> String {
        if self.min_count.0 == 0{
            "".to_string()
        }else{
            self.min_count.1.to_owned()
        }
    }
}


fn main(){
    let mut all_one = AllOne::new();  // ["AllOne"]
        
        // ["inc", "hello"]
        all_one.inc("hello".to_string());
        
        // ["inc", "hello"]
        all_one.inc("hello".to_string());
        
        // ["getMaxKey"], expected output: "hello"
        assert_eq!(all_one.get_max_key(), "hello".to_string());
        
        // ["getMinKey"], expected output: "hello"
        assert_eq!(all_one.get_min_key(), "hello".to_string());

        // ["inc", "leet"]
        all_one.inc("leet".to_string());
        
        // ["getMaxKey"], expected output: "hello"
        assert_eq!(all_one.get_max_key(), "hello".to_string());

        // ["getMinKey"], expected output: "leet"
        assert_eq!(all_one.get_min_key(), "leet".to_string());

        // ["dec", "hello"], decrement count of "hello"
        all_one.dec("hello".to_string());

        // After decrementing "hello", its count is now 1 (same as "leet").
        // ["getMaxKey"], expected output: "hello" or "leet" (either could be the max key)
        assert_eq!(all_one.get_max_key(), "leet".to_string());

        // ["getMinKey"], expected output: "hello" or "leet"
        assert_eq!(all_one.get_min_key(), "leet".to_string());

        // ["dec", "hello"], decrement count of "hello" again, now "hello" should be removed
        all_one.dec("hello".to_string());

        // After removing "hello", "leet" should be both max and min key since it's the only remaining key.
        // ["getMaxKey"], expected output: "leet"
        assert_eq!(all_one.get_max_key(), "leet".to_string());

        // ["getMinKey"], expected output: "leet"
        assert_eq!(all_one.get_min_key(), "leet".to_string());
}