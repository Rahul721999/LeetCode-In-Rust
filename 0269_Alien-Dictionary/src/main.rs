mod tests;
fn main() {
    let input = vec![
        "hrn".to_string(),
        "hrf".to_string(),
        "er".to_string(),
        "enn".to_string(),
        "rfnn".to_string(),
    ];
    let output = Solution::alien_order(input);
    println!("output: '{output}' ");
}
use std::collections::{HashMap, HashSet, VecDeque};
pub struct Solution;
impl Solution {
    pub fn alien_order(words: Vec<String>) -> String {
        let mut graph: HashMap<char, HashSet<char>> = HashMap::new();
        let mut indegree: HashMap<char, i32> = HashMap::new();

        // initialize all chars
        for word in &words {
            for ch in word.chars() {
                indegree.entry(ch).or_insert(0);
            }
        }

        // build graph
        for pair in words.windows(2) {
            let word1: Vec<char> = pair[0].chars().collect();
            let word2: Vec<char> = pair[1].chars().collect();

            let min_len = word1.len().min(word2.len());
            let mut p = 0;

            while p < min_len && word1[p] == word2[p] {
                p += 1;
            }

            // invalid prefix case
            if p == min_len {
                if word1.len() > word2.len() {
                    return String::new();
                }
                continue;
            }

            let (src, dest) = (word1[p], word2[p]);

            let inserted = graph
                .entry(src)
                .or_default()
                .insert(dest);

            if inserted {
                *indegree.get_mut(&dest).unwrap() += 1;
            }
        }

        // topo sort
        let mut queue = VecDeque::new();

        for (&node, &degree) in &indegree {
            if degree == 0 {
                queue.push_back(node);
            }
        }

        let mut res = String::new();
        let mut processed = 0;

        while let Some(node) = queue.pop_front() {
            res.push(node);
            processed += 1;

            if let Some(neighbors) = graph.get(&node) {
                for &neighbor in neighbors {
                    let degree = indegree.get_mut(&neighbor).unwrap();

                    *degree -= 1;

                    if *degree == 0 {
                        queue.push_back(neighbor);
                    }
                }
            }
        }

        if processed != indegree.len() {
            return String::new();
        }

        res
    }
}