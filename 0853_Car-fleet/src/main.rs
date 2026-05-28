mod tests;
fn main() {
    let target = 12;
    let position = vec![10, 8, 0, 5, 3];
    let speed = vec![2, 4, 1, 1, 3];
    let output =  Solution::car_fleet(target, position, speed);
    println!("{output}");
    assert_eq!(output, 3);

}
pub struct Solution;
impl Solution {
    pub fn car_fleet(target: i32, position: Vec<i32>, speed: Vec<i32>) -> i32 {
        let mut stack: Vec<f64> = Vec::new();

        // (position, time taken)
        let mut pt: Vec<(i32, f64)> = Vec::new();

        for (idx, position) in position.iter().enumerate() {
            let time_taken_to_reach = (target - position)as f64 / speed[idx] as f64;
            pt.push((*position, time_taken_to_reach));
        }

        // sort the pt array based on car position
        pt.sort_by(|a, b| b.0.cmp(&a.0));

        // start processing
        for (_car, time) in pt {
            if stack.is_empty() {
                stack.push(time);
            } else {
                if let Some(last_fleet_timing) = stack.last() {
                    if last_fleet_timing >= &time {
                        continue;
                    }
                    stack.push(time);
                }
            }
        }

        stack.len() as i32
    }
}
