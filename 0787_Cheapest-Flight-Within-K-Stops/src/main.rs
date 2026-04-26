mod tests;
fn main() {
    let flights = vec![
        vec![0, 1, 100],
        vec![1, 2, 100],
        vec![2, 0, 100],
        vec![1, 3, 600],
        vec![2, 3, 200],
    ];
    let output = Solution::find_cheapest_price(4, flights, 0, 3, 1);
    assert_eq!(700, output);
}
pub struct Solution;
impl Solution {
    pub fn find_cheapest_price(n: i32, flights: Vec<Vec<i32>>, src: i32, dst: i32, k: i32) -> i32 {
        // create and initialize the distance array, with source = 0
        let mut price_tracker = vec![vec![std::i32::MAX; k as usize + 2]; n as usize];
        price_tracker[src as usize][0] = 0;

        use std::cmp::Reverse;
        use std::collections::{BinaryHeap, HashMap};
        let mut map: HashMap<i32, Vec<(i32, i32)>> = HashMap::new(); // src -> Vec<(dest, price)>

        for flight in flights {
            let src = flight[0];
            let dest = flight[1];
            let price = flight[2];

            if let Some(entry) = map.get_mut(&src) {
                entry.push((dest, price));
            } else {
                map.insert(src, vec![(dest, price)]);
            }
        }

        let mut min_heap: BinaryHeap<Reverse<(i32, i32, i32)>> = BinaryHeap::new(); // (price, node, stop)
        min_heap.push(Reverse((0, src, 0)));

        while let Some(Reverse((curr_spent, node, stop))) = min_heap.pop() {
            if node == dst {
                return curr_spent
            }
            if let Some(neighbers) = map.get(&node) {
                for (dest, dest_flight_price) in neighbers {
                    let d = *dest as usize;
                    let new_spent = curr_spent + dest_flight_price;
                    let new_stop = stop as usize + 1;

                    if stop < k + 1 {
                        if price_tracker[d][new_stop] > new_spent {
                            price_tracker[d][new_stop] = new_spent;
                            min_heap.push(Reverse((new_spent, *dest, stop + 1)));
                        }
                    }
                }
            }
        }
        -1
    }
}
