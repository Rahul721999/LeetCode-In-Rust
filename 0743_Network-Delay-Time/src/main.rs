mod tests;
fn main() {
    let times = vec![vec![2, 1, 1], vec![2, 3, 1], vec![3, 4, 1]];
    let output = Solution::network_delay_time(times, 4, 2);
    assert_eq!(2, output);
}
pub struct Solution;
impl Solution {
    // n = number of nodes
    // k = source node
    pub fn network_delay_time(times: Vec<Vec<i32>>, n: i32, k: i32) -> i32 {
        let mut dist = vec![std::i32::MAX; n as usize + 1];

        use std::collections::{BinaryHeap, HashMap};
        use std::cmp::Reverse;
        
        // create one adjucency list or hash
        let mut map: HashMap<i32, Vec<(i32, i32)>> = HashMap::new(); // source -> (Dest, weight)

        for time in times {
            let source = time[0];
            let dest = time[1];
            let weight = time[2];
            if let Some(edges) = map.get_mut(&source) {
                edges.push((dest, weight));
            } else {
                map.insert(source, vec![(dest, weight)]);
            }
        }

        // create min-heap
        let mut queue: BinaryHeap<Reverse<(i32, i32)>> = BinaryHeap::new(); // (weight, node)

        // mark source as 0 and push to queue
        dist[k as usize] = 0;
        queue.push(Reverse((0, k)));

        while let Some(Reverse((curr_dist, source))) = queue.pop() {
            if let Some(edge) = map.get(&source) {
                for (dest, weight) in edge {
                    let dest = *dest as usize;
                    if curr_dist > dist[source as usize] {
                        continue;
                    }
                    if dist[dest] > curr_dist + weight {
                        dist[dest] = curr_dist + weight;
                        queue.push(Reverse((dist[dest], dest as i32)));
                    }
                }
            }
        }

        let mut max_time = 0;
        for i in 1..=n as usize {
            if dist[i] == std::i32::MAX {
                return -1;
            }
            max_time = max_time.max(dist[i]);
        }
        max_time
    }
}
