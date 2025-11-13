pub mod tests;

fn main() {
    let n = 6;
    let edges = vec![vec![0, 1], vec![1, 2], vec![2, 3], vec![4, 5]];
    let result = Solution::count_component(n, edges);

    assert_eq!(result, 2);
}
pub struct Solution;
impl Solution {
    pub fn count_component(n: usize, edges: Vec<Vec<usize>>) -> i32 {
        let mut component_count = 0;

        let mut graph: Vec<Vec<usize>> = vec![vec![]; n];
        let mut visited = vec![false; n];

        for edge in edges {
            let (a, b) = (edge[0], edge[1]);
            graph[a].push(b);
            graph[b].push(a);
        }

        // traverse through the graph
        for node in 0..n {
            if !visited[node] {
                component_count += 1;
                dfs(node, &mut visited, &graph);
            }
        }
        component_count
    }
}
fn dfs(node: usize, visited: &mut Vec<bool>, graph: &Vec<Vec<usize>>) {
    if visited[node] {
        return;
    }
    visited[node] = true;

    for &neighbour in &graph[node] {
        dfs(neighbour, visited, graph)
    }
}
