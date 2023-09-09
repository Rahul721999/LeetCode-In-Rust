fn main() {
    let mut nums = vec![1, 1, 2];

    let res = remove_duplicates(nums.as_mut());
    println!("{}", res);
}

pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    match nums.len() {
        0 => return 0,
        1 => return 1,
        _ => {
            let mut count = 1;

            for i in 1..nums.len() {
                if Some(nums[i]) != Some(nums[i - 1]) {
                    nums[count] = nums[i];
                    count += 1;
                }
            }
            count as i32
        }
    }
}
