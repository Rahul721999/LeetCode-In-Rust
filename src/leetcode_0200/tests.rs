#[cfg(test)]
mod tests {
    use crate::leetcode_0200::problem::{self};

    #[test]
    fn test_example1() {
        let input = vec![
            vec!['0', '1', '1', '1', '0'],
            vec!['0', '1', '0', '1', '0'],
            vec!['1', '1', '0', '0', '0'],
            vec!['0', '0', '0', '0', '0'],
        ];
        let expect = 1;

        let output = problem::Solution::num_islands(input);
        assert_eq!(output, expect);
    }

    #[test]
    fn test_example2() {
        let input = vec![
            vec!['1', '1', '0', '0', '0'],
            vec!['1', '1', '0', '0', '0'],
            vec!['0', '0', '1', '0', '0'],
            vec!['0', '0', '0', '1', '1'],
        ];
        let expect = 3;

        let output = problem::Solution::num_islands(input);
        assert_eq!(output, expect);
    }

    #[test]
    fn test_example3() {
        let input = vec![
            vec!['1', '1', '0', '1', '0'],
            vec!['1', '0', '1', '1', '0'],
            vec!['0', '1', '0', '0', '1'],
            vec!['1', '1', '0', '1', '1'],
        ];
        let expect = 4;

        let output = problem::Solution::num_islands(input);
        assert_eq!(output, expect);
    }
}
