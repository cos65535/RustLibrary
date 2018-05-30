#[allow(dead_code)]
struct SubsetGenerator {
    subset: usize,
    set: usize,
}
#[allow(dead_code)]
impl SubsetGenerator {
    fn new(set: usize) -> SubsetGenerator {
        SubsetGenerator {
            subset: set & (-(set as i64) as usize),
            set: set,
        }
    }
}
impl Iterator for SubsetGenerator {
    type Item = usize;

    fn next(&mut self) -> Option<usize> {
        if self.subset == 0 {
            return None;
        }
        let ret = self.subset;
        self.subset = (self.subset as i64 - self.set as i64) as usize & self.set;
        Some(ret)
    }
}

#[test]
fn test_next_subset() {
    for s in SubsetGenerator::new(0b1101) {
        println!("{:04b}", s);
    }
    let mut iterator = SubsetGenerator::new(0b1101);
    assert!(iterator.next().unwrap() == 0b0001);
    assert!(iterator.next().unwrap() == 0b0100);
    assert!(iterator.next().unwrap() == 0b0101);
    assert!(iterator.next().unwrap() == 0b1000);
    assert!(iterator.next().unwrap() == 0b1001);
    assert!(iterator.next().unwrap() == 0b1100);
    assert!(iterator.next().unwrap() == 0b1101);
    assert!(iterator.next() == None);
}
