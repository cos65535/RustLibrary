const INF: Weight = 1 << 30;
fn bellman_ford(g: &Graph, s: usize, t: usize) -> Weight {
    let n = g.len();
    let mut dist = vec![INF; n];
    dist[s] = 0;
    for _iter in 0..n {
        let mut end = true;
        for i in 0..n {
            for edge in g[i].iter() {
                let ncost = dist[i] + edge.weight;
                if dist[edge.dest] > ncost {
                    dist[edge.dest] = ncost;
                    end = false;
                }
            }
        }
        if end {
            return dist[t];
        }
    }
    -INF
}
