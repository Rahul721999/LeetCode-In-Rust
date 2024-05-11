mod tests;
fn main() {
    let mut test_vec = vec![ 12, 11, 13, 5, 6, 7 ];
    merge_sort(&mut test_vec);
    assert_eq!(test_vec, vec![5, 6, 7, 11, 12, 13])
}


pub fn merge_sort(arr: &mut [i32]){
    let len = arr.len();
    sort(arr, 0, len);

    fn sort(arr: &mut [i32], start: usize, end: usize){
        if end - start <= 1 {return}
        let mid = start + ((end - start) / 2);
        
        sort(arr, start, mid);
        sort(arr, mid, end);
        merge(arr, start, mid, end)
    }
    fn merge(arr: &mut [i32], start: usize, mid: usize, end: usize){
        // copy subarrays to two diff temp array
        let temp_arr1  = arr[start..mid].to_vec(); // this is actually from [0..mid-1]
        let temp_arr2  = arr[mid..end].to_vec();
        let(mut i,mut j, mut k) = (0,0,start);

        while i < temp_arr1.len() && j < temp_arr2.len(){
            if temp_arr1[i] <= temp_arr2[j]{
                arr[k] = temp_arr1[i];
                i+=1;
            }else{
                arr[k] = temp_arr2[j];
                j+=1;
            }
            k+=1;
        }

        // copy remaining element of temp_arr1
        while i < temp_arr1.len(){
            arr[k] = temp_arr1[i];
            i+=1; k+=1;
        }
        // copy remaining element of temp_arr2
        while j < temp_arr2.len(){
            arr[k] = temp_arr2[j];
            j+=1; k+=1;
        }
        
    }
}