#[cfg(test)]
mod tests {
    use crate::leetcode_1935::problem::*;

    #[test]
    fn test_example1() {
        let text = "hello world".to_string();
        let broken = "ad".to_string();
        // "hello" has 'd'? no, but it has 'a'? no → still allowed
        // actually "hello" has no broken letters, but "world" has 'd'
        // so only "hello" can be typed
        assert_eq!(Solution::can_be_typed_words(text, broken), 1);
    }

    #[test]
    fn test_example2() {
        let text = "leet code".to_string();
        let broken = "lt".to_string();
        // "leet" has 'l' and 't' → broken
        // "code" has none → allowed
        assert_eq!(Solution::can_be_typed_words(text, broken), 1);
    }

    #[test]
    fn test_example3() {
        let text = "rust is safe fast productive".to_string();
        let broken = "sf".to_string();
        // "rust" → no broken letters → allowed
        // "is" → contains 's' → broken
        // "safe" → contains 's' and 'f' → broken
        // "fast" → contains 's' and 'f' → broken
        // "productive" → contains 'f' → broken
        assert_eq!(Solution::can_be_typed_words(text, broken), 1);
    }
}
