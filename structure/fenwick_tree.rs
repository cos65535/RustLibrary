#[allow(dead_code)]
struct FenwickTree {
    seq: Vec<i64>,
}
#[allow(dead_code)]
impl FenwickTree {
    fn new(n: usize) -> FenwickTree {
        FenwickTree { seq: vec![0; n + 2] }
    }
    fn add(&mut self, index: usize, value: i64) {
        let mut index = (index + 1) as i32;
        while (index as usize) < self.seq.len() {
            self.seq[index as usize] += value;
            index += index & -index;
        }
    }
    // inclusive
    fn sum(&self, index: usize) -> i64 {
        let mut ret = 0;
        let mut index = (index + 1) as i32;
        while index > 0 {
            ret += self.seq[index as usize];
            index -= index & -index;
        }
        ret
    }
}