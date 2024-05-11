#[allow(unused)]
use crate::merge_sort;
#[test]
pub fn test_with_empty_vec() {
    let mut data: Vec<i32> = Vec::new();
    merge_sort(&mut data);
    assert_eq!(data, vec![]);
}

#[test]
pub fn test_with_nearly_shorted_vec() {
    let mut data = vec![1, 2, 3, 5, 4];
    merge_sort(&mut data);
    assert_eq!(data, vec![1, 2, 3, 4, 5]);
}

#[test]
pub fn test_with_zeroes() {
    let mut data = vec![0, 0, 0, 0, 0];
    merge_sort(&mut data);
    assert_eq!(data, vec![0, 0, 0, 0, 0]);
}

#[test]
pub fn test_with_duplicates() {
    let mut data = vec![5, 3, 2, 5, 1, 4, 2];
    merge_sort(&mut data);
    assert_eq!(data, vec![1, 2, 2, 3, 4, 5, 5]);
}

#[test]
pub fn test_merge_sort(){
    let mut data = vec![8, 5 , 7, 3, 2];
    merge_sort(&mut data);
    assert_eq!(data, vec![2, 3, 5, 7, 8]);
}