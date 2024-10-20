mod tests;
use std::collections::HashMap;

fn main() {
    let input = vec![3, 3, 3, 1, 2, 1, 1, 2, 3, 3, 4];
    let output = Solution::total_fruit(input);
    assert_eq!(5, output);
}

pub struct Solution;
impl Solution {
    pub fn total_fruit(fruits: Vec<i32>) -> i32 {
        let mut left = 0;
        let mut res = 0;
        let mut total = 0;
        let mut baskets: HashMap<i32, i32> = HashMap::new();

        for (right, _ele) in fruits.iter().enumerate() {
            *baskets.entry(fruits[right]).or_insert(0) += 1;
            total += 1;
            // dbg!(right, baskets.clone());

            while baskets.len() > 2 {
                // dbg!(left, baskets.clone());
                let l = fruits[left];
                if let Some(value) = baskets.get_mut(&l) {
                    *value -= 1;
                    total -= 1;
                    if *value == 0 {
                        baskets.remove(&l);
                    }
                }
                left += 1;
            }
            res = res.max(total);
        }
        res
    }
}
