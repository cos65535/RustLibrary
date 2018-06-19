#[allow(dead_code)]
#[derive(Clone)]
struct Node {
    v: i32,
}

#[allow(dead_code)]
impl Node {
    fn new(v: i32) -> Node {
        Node { v: v }
    }
    fn default_value() -> Node {
        Node { v: 0 }
    }
}
#[allow(dead_code)]
fn merge(left: &Node, right: &Node) -> Node {
    Node::new(std::cmp::max(left.v, right.v))
}
// add op
#[allow(dead_code)]
fn resolve_lazy(lhs: &Node, rhs: &Node) -> Node {
    Node::new(lhs.v + rhs.v)
}

#[allow(dead_code)]
struct LazySegmentTree {
    n: usize,
    max_depth: usize,
    data: Vec<Node>,
    lazy_data: Vec<Node>,
    updated: Vec<bool>,
}
#[allow(dead_code)]
impl LazySegmentTree {
    fn new(n: usize) -> LazySegmentTree {
        let mut max_depth = 0;
        while (1 << max_depth) < n {
            max_depth += 1;
        }
        LazySegmentTree {
            n: n,
            max_depth: max_depth,
            data: vec![Node::default_value(); 1 << (max_depth + 1)],
            lazy_data: vec![Node::default_value(); 1 << (max_depth + 1)],
            updated: vec![false; 1 << (max_depth + 1)],
        }
    }
    // [left, right)
    fn range_op(&mut self, left: usize, right: usize, value: Node) {
        assert!(left < right);
        assert!(right <= self.n);
        self.in_range_op(value, 0, 1, left, right);
    }
    // [left, right)
    fn get(&mut self, left: usize, right: usize) -> Node {
        assert!(left < right);
        assert!(right <= self.n);
        self.in_get(0, 1, left, right)
    }
    // private
    fn in_range_op(&mut self, value: Node, depth: usize, node: usize, left: usize, right: usize) {
        let width = 1 << (self.max_depth - depth);
        let index = node - (1 << depth);
        let node_left = index * width;
        let node_mid = node_left + (width >> 1);
        self.divide(node);
        if right - left == width && left == node_left {
            self.updated[node] = true;
            self.lazy_data[node] = value;
            self.divide(node);
        } else {
            if right <= node_mid {
                self.in_range_op(value, depth + 1, node * 2, left, right);
                self.divide(node * 2 + 1);
            } else if node_mid <= left {
                self.divide(node * 2);
                self.in_range_op(value, depth + 1, node * 2 + 1, left, right);
            } else {
                self.in_range_op(value.clone(), depth + 1, node * 2, left, node_mid);
                self.in_range_op(value, depth + 1, node * 2 + 1, node_mid, right);
            }
            self.data[node] = merge(&self.data[node * 2], &self.data[node * 2 + 1]);
        }
    }
    fn in_get(&mut self, depth: usize, node: usize, left: usize, right: usize) -> Node {
        let width = 1 << (self.max_depth - depth);
        let index = node - (1 << depth);
        let node_left = index * width;
        let node_mid = node_left + (width >> 1);
        self.divide(node);
        if right - left == width && left == node_left {
            return self.data[node].clone();
        } else if right <= node_mid {
            return self.in_get(depth + 1, node * 2, left, right);
        } else if node_mid <= left {
            return self.in_get(depth + 1, node * 2 + 1, left, right);
        }
        merge(
            &self.in_get(depth + 1, node * 2, left, node_mid),
            &self.in_get(depth + 1, node * 2 + 1, node_mid, right),
        )
    }
    fn divide(&mut self, node: usize) {
        if !self.updated[node] {
            return;
        }
        if node < (1 << self.max_depth) {
            for i in 0..2 {
                if self.updated[node * 2 + i] {
                    self.lazy_data[node * 2 + i] =
                        resolve_lazy(&self.lazy_data[node * 2 + i], &self.lazy_data[node]);
                } else {
                    self.lazy_data[node * 2 + i] = self.lazy_data[node].clone();
                    self.updated[node * 2 + i] = true;
                }
            }
        }
        self.updated[node] = false;
        self.data[node] = resolve_lazy(&self.data[node], &self.lazy_data[node]);
    }
}

#[test]
fn test_lazy_segment_tree() {
    let mut stree = LazySegmentTree::new(10);
    stree.range_op(1, 4, Node::new(3));
    assert!(stree.get(0, 1).v == 0);
    assert!(stree.get(1, 2).v == 3);
    assert!(stree.get(3, 4).v == 3);
    assert!(stree.get(4, 5).v == 0);
    assert!(stree.get(0, 10).v == 3);

    stree.range_op(3, 6, Node::new(7));
    assert!(stree.get(2, 3).v == 3);
    assert!(stree.get(3, 4).v == 10);
    assert!(stree.get(5, 6).v == 7);
    assert!(stree.get(6, 7).v == 0);
    assert!(stree.get(0, 10).v == 10);

    stree.range_op(3, 6, Node::new(10));
    stree.range_op(3, 6, Node::new(10));
    stree.range_op(3, 6, Node::new(10));
    assert!(stree.get(3, 4).v == 40);
}
