fn scc(g: &Graph) -> Vec<Vec<usize>> {
    let n = g.len();
    let mut ret = vec![];
    let mut revg: Graph = vec![vec![]; n];
    for edges in g {
        for edge in edges {
            revg[edge.dest].push(Edge::new(edge.index, edge.dest, edge.src, edge.weight));
        }
    }
    let mut st = vec![];
    let mut visit = vec![false; n];
    for i in 0..n {
        if visit[i] { continue; }
        scc_dfs(g, i, &mut visit, &mut st);
    }
    visit = vec![false; n];
    for i in (0..n).rev() {
        let index = st[i];
        if visit[index] { continue; }
        let mut nret = vec![];
        scc_dfs(&revg, index, &mut visit, &mut nret);
        ret.push(nret);
    }
    ret
}

fn scc_dfs(g: &Graph, from: usize, visit: &mut Vec<bool>, st: &mut Vec<usize>) {
    visit[from] = true;
    for edge in g[from].iter() {
        if visit[edge.dest] { continue; }
        scc_dfs(g, edge.dest, visit, st);
    }
    st.push(from);
}