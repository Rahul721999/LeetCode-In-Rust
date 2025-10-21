mod tests;
fn main() {
    let input = vec![vec![0, 1], vec![0, 2], vec![0, 3], vec![1, 4]];
    let output = Solution::valid_graph(5, input);
    assert!(output);
}
pub struct Solution;
impl Solution {
    pub fn valid_graph(n: usize, edges: Vec<Vec<usize>>) -> bool {
        // 1. Build adjacency list
        let mut map = vec![vec![]; n];
        for pair in &edges {
            let (a, b) = (pair[0], pair[1]);
            map[a].push(b);
            map[b].push(a);
        }

        // 2. Track visited nodes
        let mut visited = vec![false; n];

        // 3. DFS: check no cycle + all connected
        dfs(0, n, &map, &mut visited) && !visited.contains(&false)
    }
}

fn dfs(node: usize, parent: usize, graph: &Vec<Vec<usize>>, visited: &mut Vec<bool>) -> bool {
    if visited[node] {
        return false;
    } // found a cycle
    visited[node] = true; // mark visited

    for &nei in &graph[node] {
        if nei == parent {
            continue;
        }
        if !dfs(nei, node, graph, visited) {
            return false;
        }
    }
    true
}
