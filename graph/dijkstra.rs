#[allow(dead_code)]
fn dijkstra(g: &Graph, s: usize, t: usize) -> Option<Weight> {
    let n = g.len();
    let mut visit = vec![false; n];
    let mut dists = vec![Weight::max_value(); n];
    let mut que = BinaryHeap::new();
    dists[s] = 0;
    que.push((0, s));
    while let Some((cost, from)) = que.pop() {
        let cost = -cost;
        if visit[from] { continue; }
        visit[from] = true;
        if from == t { return Some(cost); }
        for edge in g[from].iter() {
            let to = edge.dest;
            let ncost = cost + edge.weight;
            if visit[to] || ncost >= dists[to] { continue; }
            dists[to] = ncost;
            que.push((-ncost, to));
        }
    }
    return None;
}