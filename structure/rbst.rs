// need XorShift
type Node = i64;
#[derive(Clone)]
#[allow(dead_code)]
struct RbstNode {
    v: Node,
    size: usize,
    l: Option<usize>,
    r: Option<usize>,
}
#[allow(dead_code)]
impl RbstNode {
    fn new(v: Node) -> RbstNode {
        RbstNode {
            v: v,
            size: 1,
            l: None,
            r: None,
        }
    }
}

#[allow(dead_code)]
struct Rbst {
    pool: Vec<RbstNode>,
    rnd: XorShift,
}
#[allow(dead_code)]
impl Rbst {
    fn new() -> Rbst {
        Rbst {
            pool: vec![],
            rnd: XorShift::new(),
        }
    }
    fn insert(&mut self, root: Option<usize>, k: usize, v: Node) -> Option<usize> {
        let s = self.split(root, k);
        let t = Some(self.pool.len());
        self.pool.push(RbstNode::new(v));
        let r = self.merge(t, s.1);
        return self.merge(s.0, r);
    }
    fn erase(&mut self, root: Option<usize>, k: usize) -> Option<usize> {
        let s1 = self.split(root, k + 1);
        let s2 = self.split(s1.0, k);
        return self.merge(s2.0, s1.1);
    }
    fn get_at(&self, root: Option<usize>, index: usize) -> Node {
        let t = self.at(root, index);
        self.pool[t.unwrap()].v
    }
    fn set_at(&mut self, root: Option<usize>, index: usize, v: Node) {
        let t = self.at(root, index);
        self.pool[t.unwrap()].v = v;
    }
    // private
    fn merge(&mut self, l: Option<usize>, r: Option<usize>) -> Option<usize> {
        if l.is_none() {
            return r;
        }
        if r.is_none() {
            return l;
        }
        if self.rnd.xor128() as usize % (self.count(l) + self.count(r)) < self.count(l) {
            let child_r = self.pool[l.unwrap()].r;
            self.pool[l.unwrap()].r = self.merge(child_r, r);
            self.update_count(l.unwrap());
            return l;
        } else {
            let child_l = self.pool[r.unwrap()].l;
            self.pool[r.unwrap()].l = self.merge(l, child_l);
            self.update_count(r.unwrap());
            return r;
        }
    }
    // [0, k), [k, n)
    fn split(&mut self, t: Option<usize>, k: usize) -> (Option<usize>, Option<usize>) {
        if t.is_none() {
            return (None, None);
        }
        let t = t.unwrap();
        if k <= self.count(self.pool[t].l) {
            let l = self.pool[t].l;
            let s = self.split(l, k);
            self.pool[t].l = s.1;
            self.update_count(t);
            return (s.0, Some(t));
        } else {
            let l = self.pool[t].l;
            let r = self.pool[t].r;
            let cnt = k - self.count(l) - 1;
            let s = self.split(r, cnt);
            self.pool[t].r = s.0;
            self.update_count(t);
            return (Some(t), s.1);
        }
    }
    fn at(&self, root: Option<usize>, index: usize) -> Option<usize> {
        assert!(root != None);
        let mut index = index;
        let t = root.unwrap();
        let l = self.pool[t].l;
        let r = self.pool[t].r;
        if index < self.count(l) {
            return self.at(l, index);
        } else {
            index -= self.count(l);
        }
        if index == 0 {
            return Some(t);
        }
        index -= 1;
        return self.at(r, index);
    }
    fn count(&self, t: Option<usize>) -> usize {
        if let Some(t) = t {
            self.pool[t].size
        } else {
            0
        }
    }
    fn update_count(&mut self, t: usize) {
        self.pool[t].size = 1 + self.count(self.pool[t].l) + self.count(self.pool[t].r);
    }
}

#[test]
fn rbst_test() {
    let mut rbst = Rbst::new();
    let mut root = None;

    root = rbst.insert(root, 0, 10);
    root = rbst.insert(root, 1, 30);
    assert!(rbst.get_at(root, 0) == 10);
    assert!(rbst.get_at(root, 1) == 30);

    root = rbst.erase(root, 0);
    assert!(rbst.get_at(root, 0) == 30);

    rbst.set_at(root, 0, 123);
    assert!(rbst.get_at(root, 0) == 123);

    root = rbst.insert(root, 0, 222);
    assert!(rbst.get_at(root, 0) == 222);
    assert!(rbst.get_at(root, 1) == 123);
}

#[test]
fn rbst_test2() {
    let mut rbst = Rbst::new();
    let mut root = None;
    for i in 0..1000 {
        root = rbst.insert(root, i, i as i64);
    }
    for i in 0..1000 {
        assert!(rbst.get_at(root, i) == i as i64);
    }
    for i in 0..1000 {
        root = rbst.insert(root, 0, i as i64);
    }
    for i in 0..1000 {
        assert!(rbst.get_at(root, i) == 999 - i as i64);
    }
    for i in 0..1000 {
        root = rbst.erase(root, i);
    }
    for i in 0..500 {
        assert!(rbst.get_at(root, i) == 998 - i as i64 * 2);
    }
    for i in 0..500 {
        assert!(rbst.get_at(root, 500 + i) == 1 + i as i64 * 2);
    }
}
