#[allow(dead_code)]
struct XorShift {
    x: u32,
    y: u32,
    z: u32,
    w: u32,
}
#[allow(dead_code)]
impl XorShift {
    fn new() -> XorShift {
        XorShift {
            x: 0x34fb2383,
            y: 0x327328fa,
            z: 0xabd4b54a,
            w: 0xa9dba8d1,
        }
    }
    fn seed(s: u32) -> XorShift {
        let mut rnd = XorShift::new();
        rnd.w = s;
        rnd
    }
    fn xor128(&mut self) -> u32 {
        let t = self.x ^ (self.x << 11);
        self.x = self.y;
        self.y = self.z;
        self.z = self.w;
        self.w = (self.w ^ (self.w >> 19)) ^ (t ^ (t >> 8));
        self.w
    }
    fn next_i32(&mut self, r: i32) -> i32 {
        (self.xor128() % r as u32) as i32
    }
    fn next_i32_lr(&mut self, l: i32, r: i32) -> i32 {
        self.next_i32(r - l + 1) + l
    }
    fn next_i64(&mut self, r: i64) -> i64 {
        ((((self.xor128() as u64) << 32) + self.xor128() as u64) % r as u64) as i64
    }
    fn next_i64_lr(&mut self, l: i64, r: i64) -> i64 {
        self.next_i64(r - l + 1) + l
    }
    fn next_usize(&mut self, r: usize) -> usize {
        ((((self.xor128() as u64) << 32) + self.xor128() as u64) % r as u64) as usize
    }
    fn next_usize_lr(&mut self, l: usize, r: usize) -> usize {
        self.next_usize(r - l + 1) + l
    }
    fn next_f64(&mut self, r: f64) -> f64 {
        (self.xor128() as f64) / (0xffffffffu32 as f64) * r
    }
    fn next_f64_lr(&mut self, l: f64, r: f64) -> f64 {
        self.next_f64(r - l) + l
    }
}
