type Weight = i32;
#[allow(dead_code)]
#[derive(Clone, Debug, Eq, PartialEq)]
struct Edge {
    index: usize,
    src: usize,
    dest: usize,
    weight: Weight,
}
#[allow(dead_code)]
impl Edge {
    fn new(index: usize, src: usize, dest: usize, weight: Weight) -> Edge {
        Edge {
            index: index,
            src: src,
            dest: dest,
            weight: weight,
        }
    }
}
impl std::cmp::Ord for Edge {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (self.weight, self.src, self.dest).cmp(&(other.weight, other.src, other.dest))
    }
}
impl std::cmp::PartialOrd for Edge {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
type Edges = Vec<Edge>;
type Graph = Vec<Edges>;
#[allow(dead_code)]
fn graph_new(n: usize) -> Graph {
    vec![vec![]; n]
}
#[allow(dead_code)]
fn add_bi_edge(graph: &mut Graph, index: &mut usize, src: usize, dest: usize, weight: Weight) {
    graph[src].push(Edge::new(*index, src, dest, weight));
    graph[dest].push(Edge::new(*index, dest, src, weight));
    *index += 1;
}
#[allow(dead_code)]
fn add_uni_edge(graph: &mut Graph, index: &mut usize, src: usize, dest: usize, weight: Weight) {
    graph[src].push(Edge::new(*index, src, dest, weight));
    *index += 1;
}