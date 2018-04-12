fn bridge(g: &Graph) -> Edges {
    let n = g.len();
    let mut bridges: Edges = vec![];
    let mut order = vec![None; n];
    let mut roots = vec![];
    let mut st = vec![];
    let mut connect = vec![vec![]];
    let mut cnt = 0;
    for i in 0..n {
        if order[i] != None { continue; }
        bridge_dfs(g, &Edge::new(std::usize::MAX, i, i, 0), &mut bridges, &mut connect, &mut roots, &mut st, &mut order, &mut cnt);
        bridges.pop();
    }
    return bridges;
}

fn bridge_dfs(
        g: &Graph,
        edge: &Edge,
        bridges: &mut Edges,
        connect: &mut Vec<Vec<usize>>,
        roots: &mut Vec<usize>,
        st: &mut Vec<usize>,
        order: &mut Vec<Option<usize>>,
        cnt: &mut usize)
{
    let parent_index = edge.index;
    let from = edge.dest;
    order[from] = Some(*cnt);
    *cnt += 1;
    st.push(from);
    roots.push(from);
    for edge in g[from].iter() {
        if parent_index == edge.index { continue; }
        let to = edge.dest;
        if order[to] == None {
            bridge_dfs(&g, edge, bridges, connect, roots, st, order, cnt);
        } else {
            while order[*roots.last().unwrap()].unwrap() > order[to].unwrap() {
                roots.pop();
            }
        }
    }
    if from == *roots.last().unwrap() {
        bridges.push(edge.clone());
        connect.push(vec![]);
        loop {
            let w = *st.last().unwrap();
            st.pop();
            connect.last_mut().unwrap().push(w);
            if from == w { break; }
        }
        roots.pop();
    }
}