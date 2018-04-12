use std::collections::VecDeque;
// index ^ 1 is reverse edge
#[allow(dead_code)]
fn add_bi_edge(graph: &mut Graph, index: &mut usize, src: usize, dest: usize, weight: Weight) {
    graph[src].push(Edge::new(*index, src, dest, weight));
    *index += 1;
    graph[dest].push(Edge::new(*index, dest, src, weight));
    *index += 1;
}
#[allow(dead_code)]
fn add_uni_edge(graph: &mut Graph, index: &mut usize, src: usize, dest: usize, weight: Weight) {
    graph[src].push(Edge::new(*index, src, dest, weight));
    *index += 1;
    graph[dest].push(Edge::new(*index, dest, src, 0));
    *index += 1;
}

#[allow(dead_code)]
fn augment(
    g: &Graph,
    capacity: &mut Vec<Weight>,
    level: &Vec<usize>,
    finished: &mut Vec<bool>,
    from: usize,
    t: usize,
    cur: Weight,
) -> Weight {
    if from == t || cur == 0 {
        return cur;
    }
    if finished[from] {
        return 0;
    }
    for edge in g[from].iter() {
        let to = edge.dest;
        if level[to] != level[from] + 1 {
            continue;
        }
        let ncur = std::cmp::min(cur, capacity[edge.index]);
        let f = augment(g, capacity, level, finished, to, t, ncur);
        if f > 0 {
            capacity[edge.index] -= f;
            capacity[edge.index ^ 1] += f;
            return f;
        }
    }
    finished[from] = true;
    0
}

#[allow(dead_code)]
fn max_flow(g: &Graph, e: usize, s: usize, t: usize) -> Weight {
    let n = g.len();
    let mut capacity: Vec<Weight> = vec![0; e];
    for from in 0..n {
        for edge in g[from].iter() {
            capacity[edge.index] += edge.weight;
        }
    }
    let mut ans = 0;
    loop {
        let mut level = vec![usize::max_value(); n];
        level[s] = 0;
        let mut que = VecDeque::<usize>::new();
        que.push_back(s);
        while let Some(from) = que.pop_front() {
            if from == t {
                break;
            }
            for edge in g[from].iter() {
                let to = edge.dest;
                if capacity[edge.index] > 0 && level[to] == usize::max_value() {
                    que.push_back(to);
                    level[to] = level[from] + 1;
                }
            }
        }
        let mut finished = vec![false; n];
        let mut end = true;
        loop {
            let f = augment(
                g,
                &mut capacity,
                &level,
                &mut finished,
                s,
                t,
                Weight::max_value(),
            );
            if f == 0 {
                break;
            }
            ans += f;
            end = false;
        }
        if end {
            break;
        }
    }
    ans
}
