fn main() {
    let res = Solution::quick_sort_array(vec![4, 1, 3, 6, 9, 3, 2, 5].as_mut());
    println!("{:?}", res);
}

struct Solution;
impl Solution {
    pub fn quick_sort_array(arr: &mut Vec<i32>) -> Vec<i32> {
        let len = arr.len();
        if len <= 2 {
            return arr.to_vec();
        }
        // partitioning index
        let p_idx = partition(&mut arr.to_vec());
        let mut res = Vec::new();
        if p_idx > 0 {
            let mut left_sorted_arr = Solution::quick_sort_array(&mut arr[0..p_idx].to_vec());
            // arr[0..p_idx].copy_from_slice(&left_sorted_arr);
            res.append(&mut left_sorted_arr);
        }
        if p_idx < len - 1 {
            // sort the right array to the pivot ele
            let mut right_sorted_arr = Solution::quick_sort_array(&mut arr[p_idx + 1..len].to_vec());
            // arr[p_idx + 1..len].copy_from_slice(&right_sorted_arr);
            res.append(&mut right_sorted_arr);
        }
        res
    }
}
pub fn partition(arr: &mut Vec<i32>) -> usize {
    let len = arr.len();
    let p_idx = len - 1;
    let (mut l_ptr, mut r_ptr) = (0, p_idx - 1);
    while l_ptr <= r_ptr {
        while arr[l_ptr] < arr[p_idx] {
            l_ptr += 1;
        }
        while arr[r_ptr] > arr[p_idx] {
            r_ptr -= 1;
        }
        // if arr[l_ptr] > arr[r_ptr]{
        if l_ptr < r_ptr {
            // println!("swapping: {} -> {}", arr[l_ptr], arr[r_ptr]);
            arr.swap(l_ptr, r_ptr);
            l_ptr += 1;
            r_ptr -= 1;
        }
    }
    arr.swap(l_ptr, p_idx);

    //return new pivot index
    l_ptr
}
