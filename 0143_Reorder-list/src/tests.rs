use crate::{Solution, helper};

#[test]
    fn test1() {
        let mut input = helper(vec![1, 2, 3, 4]);
        let expected_output = helper(vec![1, 4, 2, 3]);
        Solution::reorder_list(&mut input);
        // Assert that the input list is reordered correctly
        assert_eq!(expected_output, input);
    }

    #[test]
    fn test2() {
        let mut input = helper(vec![1, 2, 3, 4, 5]);
        let expected_output = helper(vec![1, 5, 2, 4, 3]);
        Solution::reorder_list(&mut input);
        // Assert that the input list is reordered correctly
        assert_eq!(expected_output, input);
    }

    #[test]
    fn test_empty_list() {
        let mut input = helper(vec![]);
        let expected_output = helper(vec![]);
        Solution::reorder_list(&mut input);
        // Assert that the input list remains empty
        assert_eq!(expected_output, input);
    }

    #[test]
    fn test_single_element() {
        let mut input = helper(vec![1]);
        let expected_output = helper(vec![1]);
        Solution::reorder_list(&mut input);
        // Assert that the single-element list remains unchanged
        assert_eq!(expected_output, input);
    }