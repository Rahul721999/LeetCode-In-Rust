#[cfg(test)]
mod tests {
    use crate::{Solution, helper};

    #[test]
    fn test_swap_pairs_empty_list() {
        let test_case = None;
        let expected_result = None;
        assert_eq!(Solution::swap_pairs(test_case), expected_result);
    }

    #[test]
    fn test_swap_pairs_single_node() {
        let test_case = helper(vec![1]);
        let expected_result = helper(vec![1]);
        assert_eq!(Solution::swap_pairs(test_case), expected_result);
    }

    #[test]
    fn test_swap_pairs_two_nodes() {
        let test_case = helper(vec![1, 2]);
        let expected_result = helper(vec![2, 1]);
        assert_eq!(Solution::swap_pairs(test_case), expected_result);
    }

    #[test]
    fn test_swap_pairs_odd_number_of_nodes() {
        let test_case = helper(vec![1, 2, 3, 4, 5]);
        let expected_result = helper(vec![2, 1, 4, 3, 5]);
        assert_eq!(Solution::swap_pairs(test_case), expected_result);
    }

    #[test]
    fn test_swap_pairs_even_number_of_nodes() {
        let test_case = helper(vec![1, 2, 3, 4]);
        let expected_result = helper(vec![2, 1, 4, 3]);
        assert_eq!(Solution::swap_pairs(test_case), expected_result);
    }
}
