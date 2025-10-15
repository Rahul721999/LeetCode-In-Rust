mod tests;
fn main() {
    let num_courses = 4;
    let prerequisites = vec![vec![1, 0], vec![2, 0], vec![3, 1], vec![3, 2]];

    let output = Solution::can_finish(num_courses, prerequisites);
    assert!(output);
}
pub struct Solution;
impl Solution {
    pub fn can_finish(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> bool {
        let num_of_cources = num_courses as usize;

        use std::collections::VecDeque;
        let mut in_degree = vec![0; num_of_cources];
        let mut graph = vec![vec![]; num_of_cources];

        for pair in prerequisites {
            let (a, b) = (pair[0] as usize, pair[1] as usize);

            graph[b].push(a);
            in_degree[a] += 1;
        }

        let mut queue = VecDeque::new();
        for i in 0..num_of_cources {
            if in_degree[i] == 0 {
                queue.push_back(i);
            }
        }

        let mut visited = 0;
        while let Some(course) = queue.pop_front() {
            visited += 1;
            for &next in &graph[course] {
                in_degree[next] -= 1;
                if in_degree[next] == 0 {
                    queue.push_back(next);
                }
            }
        }
        visited == num_of_cources
    }
}
