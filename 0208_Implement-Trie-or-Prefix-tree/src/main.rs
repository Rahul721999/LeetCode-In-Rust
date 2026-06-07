mod tests;
fn main() {
    let input = vec![
        "Trie".to_string(),
        "insert".to_string(),
        "search".to_string(),
        "search".to_string(),
        "startsWith".to_string(),
        "insert".to_string(),
        "search".to_string(),
    ];
    let mut trie = Trie::new();
    for ele in input {
        trie.insert(ele);
    }

    let output = trie.search("Trie".to_string());
    println!("Search res: {output}");

    let output = trie.starts_with("tr".to_string());
    println!("Search res: {output}");
}
#[derive(Clone, Default)]
struct TrieNode {
    // usize = 208 byte but u32: 104 byte
    // 0 = null here. no Option<> overhead
    pub children: [u32; 26],
    pub eow: bool,
}
struct Trie {
    pub arena: Vec<TrieNode>,
}

impl Trie {
    pub fn new() -> Self {
        Self {
            arena: vec![TrieNode::default()],
        }
    }

    #[inline]
    pub fn char_idx(ch: char) -> usize {
        (ch as usize) - (b'a' as usize)
    }
    pub fn insert(&mut self, word: String) {
        let mut curr_idx = 0;
        for c in word.to_lowercase().chars() {
            let idx = Self::char_idx(c);

            if self.arena[curr_idx].children[idx] == 0 {
                let new_arena_idx = self.arena.len() as u32;
                self.arena.push(TrieNode::default());
                self.arena[curr_idx].children[idx] = new_arena_idx;
            }
            curr_idx = self.arena[curr_idx].children[idx] as usize;
        }
        self.arena[curr_idx].eow = true;
    }

    pub fn search(&self, word: String) -> bool {
        let mut curr_idx = 0;
        for c in word.to_lowercase().chars() {
            let idx = Self::char_idx(c);
            let next_idx = self.arena[curr_idx].children[idx];
            if next_idx == 0 {
                return false;
            }
            curr_idx = next_idx as usize;
        }
        if !self.arena[curr_idx].eow{
            return false
        }
        true
    }

    pub fn starts_with(&self, prefix: String) -> bool {
        let mut curr_idx = 0;
        for c in prefix.to_lowercase().chars() {
            let idx = Self::char_idx(c);
            let next_idx = self.arena[curr_idx].children[idx];
            if next_idx == 0 {
                return false;
            }
            curr_idx = next_idx as usize;
        }
        true
    }
}
