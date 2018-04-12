const KIND_NUM: usize = 2;
type TrieValue = usize;
#[derive(Clone)]
struct TrieNode {
    child: [Option<usize>; KIND_NUM],
}
impl TrieNode {
    fn new() -> TrieNode {
        TrieNode {
            child: [None; KIND_NUM],
        }
    }
}
struct Trie {
    pool: Vec<TrieNode>,
}
impl Trie {
    fn new() -> Trie {
        Trie {
            pool: vec![TrieNode::new(); 1],
        }
    }
    fn push(&mut self, vs: Vec<TrieValue>) {
        let mut index = 0;
        for v in vs {
            let v = v as usize;
            if self.pool[index].child[v] == None {
                self.pool[index].child[v] = Some(self.pool.len());
                self.pool.push(TrieNode::new());
            }
            index = self.pool[index].child[v].unwrap();
        }
    }
}