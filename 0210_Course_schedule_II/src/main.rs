
mod tests;
fn main() {
    let num_courses = 4;
    let prerequisites = vec![vec![1, 0], vec![2, 0], vec![3, 1], vec![3, 2]];

    let output = Solution::find_order(num_courses, prerequisites);
    assert_eq!(output, vec![0,1,2,3]);
}
pub struct Solution;
impl Solution {
    pub fn find_order(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> Vec<i32> {
        let num_courses = num_courses as usize;
        let mut result: Vec<i32> = vec![];

        use std::collections::VecDeque;
        let mut in_degree = vec![0; num_courses];
        let mut graph = vec![vec![]; num_courses];

        for pair in prerequisites {
            let (a, b) = (pair[0] as usize, pair[1] as usize);
            graph[b].push(a);
            in_degree[a] += 1;
        }

        // init queue
        let mut queue = VecDeque::new();
        for i in 0..num_courses {
            if in_degree[i] == 0 {
                queue.push_back(i as usize);
                result.push(i as i32);
            }
        };
        while let Some(course) = queue.pop_front() {
            for &next in &graph[course] {
                in_degree[next] -= 1;
                if in_degree[next] == 0 {
                    queue.push_back(next);
                    result.push(next as i32);
                }
            }
        }

        if result.len() == num_courses {
            result
        } else {
            vec![]
        }
    }
}
