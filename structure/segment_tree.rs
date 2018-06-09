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
        Node {
            v: i32::min_value(),
        }
    }
}
#[allow(dead_code)]
fn merge(left: &Node, right: &Node) -> Node {
    Node::new(std::cmp::max(left.v, right.v))
}

#[allow(dead_code)]
struct SegmentTree {
    n: n,
    max_depth: usize,
    data: Vec<Node>,
}
#[allow(dead_code)]
impl SegmentTree {
    fn new(n: usize) -> SegmentTree {
        let mut max_depth = 0;
        while (1 << max_depth) < n {
            max_depth += 1;
        }
        SegmentTree {
            n: n,
            max_depth: max_depth,
            data: vec![Node::default_value(); 1 << (max_depth + 1)],
        }
    }
    fn set(&mut self, index: usize, value: Node) {
        assert!(index < self.n);
        let mut target = (1 << self.max_depth) + index;
        self.data[target] = value;
        for _ in 0..self.max_depth {
            target >>= 1;
            self.data[target] = merge(&self.data[target * 2], &self.data[target * 2 + 1]);
        }
    }
    // [left, right)
    fn get(&self, left: usize, right: usize) -> Node {
        assert!(left < right);
        assert!(right <= self.n);
        self.in_get(0, 1, left, right)
    }
    fn in_get(&self, depth: usize, node: usize, left: usize, right: usize) -> Node {
        let width = 1 << (self.max_depth - depth);
        let index = node - (1 << depth);
        let node_left = index * width;
        let node_mid = node_left + (width >> 1);
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
}
