mod tests;

fn main() {
    let mut data = vec![12, 19, 23, 45, 51, 8];
    insertion_sort(&mut data);
    assert_eq!(data, vec![8,12,19,23,45,51]);
}

pub fn insertion_sort(arr: &mut [i32]) {
    if arr.is_empty() {
        return;
    };
    let len = arr.len();
    for i in 0..len {
        let current = arr[i];
        let mut j = i;
        while j > 0 && arr[j - 1] > current {
            arr[j] = arr[j - 1];
            j -= 1;
        }
        arr[j] = current;
    }
}
