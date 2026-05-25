mod tests;
fn main() {
    let tickets = vec![
        vec!["BUF".to_string(), "HOU".to_string()],
        vec!["HOU".to_string(), "SEA".to_string()],
        vec!["JFK".to_string(), "BUF".to_string()],
    ];

    let output = Solution::find_itinerary(tickets);
    let expected = vec![
        "JFK".to_string(),
        "BUF".to_string(),
        "HOU".to_string(),
        "SEA".to_string(),
    ];

    assert_eq!(output, expected);
}
pub struct Solution;

use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};
impl Solution {
    pub fn find_itinerary(tickets: Vec<Vec<String>>) -> Vec<String> {
        // create a source -> [destinations] map
        let mut map: HashMap<String, BinaryHeap<Reverse<String>>> = HashMap::new();

        // initialize the map
        for t in tickets.iter() {
            let (src, dest) = (t[0].clone(), t[1].clone());
            if let Some(destinations) = map.get_mut(&src) {
                destinations.push(Reverse(dest));
            } else {
                let mut min_heap: BinaryHeap<Reverse<String>> = BinaryHeap::new();
                min_heap.push(Reverse(dest));
                map.insert(src, min_heap);
            }
        }

        // result vector
        let mut res: Vec<String> = Vec::new();

        Self::dfs("JFK".to_string(), &mut map, &mut res);

        res.reverse();
        res
    }

    fn dfs(
        src: String,
        map: &mut HashMap<String, BinaryHeap<Reverse<String>>>,
        res: &mut Vec<String>,
    ) {
        while let Some(destinations) = map.get_mut(&src) {
            match destinations.pop() {
                Some(Reverse(dest)) => Self::dfs(dest, map, res),
                None => break, // empty heap (no edge left)
            }
        }
        res.push(src);
    }
}
