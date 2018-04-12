#[allow(dead_code)]
struct LCA {
    log_size: usize,
    parent: Vec<Vec<usize>>,
    depth: Vec<usize>,
}
#[allow(dead_code)]
impl LCA {
    fn new(g: &Graph, root: usize) -> LCA {
        let n = g.len();
        let mut log_size = 0;
        while (1 << log_size) < n { log_size += 1; }
        let mut parent = vec![vec![root; log_size]; n];
        let mut depth = vec![0; n];

        let mut que = vec![];
        que.push(root);
        while let Some(node) = que.pop() {
            for edge in g[node].iter() {
                if edge.dest == parent[node][0] { continue; }
                parent[edge.dest][0] = node;
                depth[edge.dest] = depth[node] + 1;
                que.push(edge.dest);
            }
        }

        for iter in 0..log_size - 1 {
            for i in 0..n {
                parent[i][iter + 1] = parent[parent[i][iter]][iter];
            }
        }
        LCA {
            log_size: log_size,
            parent: parent,
            depth: depth,
        }
    }

    fn lca(&self, mut u: usize, mut v: usize) -> usize {
        if self.depth[u] > self.depth[v] { std::mem::swap(&mut u, &mut v); }
        for i in 0..self.log_size {
            if (((self.depth[u] ^ self.depth[v]) >> i) & 1) == 1 { v = self.parent[v][i]; }
        }
        if u == v { return u; }
        for i in (0..self.log_size).rev() {
            if self.parent[u][i] != self.parent[v][i] {
                u = self.parent[u][i];
                v = self.parent[v][i];
            }
        }
        return self.parent[u][0];
    }
}