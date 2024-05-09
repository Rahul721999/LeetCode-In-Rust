mod tests;
fn main() {
    // Run 'cargo test' command
    let output = std::process::Command::new("cargo")
        .arg("test")
        .output()
        .expect("Failed to execute 'cargo test'");
    // Print the output
    if output.status.success() {
        println!("Tests passed successfully!");
    } else {
        println!("Tests failed!");
        println!("Error: {}", String::from_utf8_lossy(&output.stderr));
    }
    let mut data = vec![8, 5 , 7, 3, 2];
    bubble_sort(&mut data);
    assert_eq!(data, vec![2, 3, 5, 7, 8]);
}

pub fn bubble_sort(arr : &mut Vec<i32>){
    if arr.is_empty(){return}
    let len = arr.len();
    let mut swapped: bool;
    for i in 0..(len- 1) as usize{
        swapped = false;
        for j in 0..(len-1-i) as usize {
            if arr[j] > arr[j+1] {
                arr.swap(j, j + 1);
                swapped = true;
            }
        }
        if !swapped{
            break
        }
    }
}
