#[allow(dead_code)]
struct UnionFind {
    parent: Vec<i32>,
}

#[allow(dead_code)]
impl UnionFind {
    fn new(n: usize) -> UnionFind {
        UnionFind { parent: vec![-1; n] }
    }
    fn root(&mut self, x: usize) -> usize {
        let p = self.parent[x];
        return if p < 0 {
            x
        } else {
            self.parent[x] = self.root(p as usize) as i32;
            self.parent[x] as usize
        }
    }
    fn union_set(&mut self, x: usize, y: usize) -> bool {
        let mut x = self.root(x);
        let mut y = self.root(y);
        if x == y { return false; }
        if self.parent[y] < self.parent[x] { std::mem::swap(&mut x, &mut y); }
        self.parent[x] += self.parent[y];
        self.parent[y] = x as i32;
        true
    }
    fn find_set(&mut self, x: usize, y: usize) -> bool {
        self.root(x) == self.root(y)
    }
    fn size(&mut self, x: usize) -> usize {
        let r = self.root(x);
        -self.parent[r] as usize
    }
}

#[test]
fn union_find() {
    let mut ufind = UnionFind::new(10);
    assert!(ufind.root(0) == 0);
    assert!(ufind.size(3) == 1);
    assert!(ufind.union_set(1, 2));
    assert!(!ufind.union_set(1, 2));
    ufind.union_set(2, 3);
    let r = ufind.root(3);
    assert!(r == 1 || r == 2 || r == 3);
    assert!(ufind.size(3) == 3);
}