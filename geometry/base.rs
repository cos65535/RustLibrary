#[allow(dead_code)]
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
struct Point {
    x: f64,
    y: f64,
}
#[allow(dead_code)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, PartialOrd, Ord)]
struct Line {
    s: Point,
    t: Point,
}
#[allow(dead_code)]
type Polygon = Vec<Point>;

impl std::cmp::Eq for Point {}
impl std::cmp::Ord for Point {
    fn cmp(&self, rhs: &Point) -> std::cmp::Ordering {
        (self.x, self.y).partial_cmp(&(rhs.x, rhs.y)).unwrap()
    }
}
impl std::ops::Neg for Point {
    type Output = Point;
    fn neg(self) -> Point {
        Point {
            x: -self.x,
            y: -self.y,
        }
    }
}
impl std::ops::Add for Point {
    type Output = Point;
    fn add(self, rhs: Point) -> Point {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}
impl std::ops::AddAssign for Point {
    fn add_assign(&mut self, rhs: Point) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}
impl std::ops::Sub for Point {
    type Output = Point;
    fn sub(self, rhs: Point) -> Point {
        Point {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}
impl std::ops::SubAssign for Point {
    fn sub_assign(&mut self, rhs: Point) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}
impl std::ops::Mul<Point> for f64 {
    type Output = Point;
    fn mul(self, rhs: Point) -> Point {
        Point {
            x: self * rhs.x,
            y: self * rhs.y,
        }
    }
}
impl std::ops::Mul<f64> for Point {
    type Output = Point;
    fn mul(self, rhs: f64) -> Point {
        Point {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}
impl std::ops::MulAssign<f64> for Point {
    fn mul_assign(&mut self, rhs: f64) {
        self.x *= rhs;
        self.y *= rhs;
    }
}
impl std::ops::Div<f64> for Point {
    type Output = Point;
    fn div(self, rhs: f64) -> Point {
        Point {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}
impl std::ops::DivAssign<f64> for Point {
    fn div_assign(&mut self, rhs: f64) {
        self.x /= rhs;
        self.y /= rhs;
    }
}

#[allow(dead_code)]
impl Point {
    fn new(x: f64, y: f64) -> Point {
        Point { x: x, y: y }
    }
    fn norm(&self) -> f64 {
        (self.x * self.x + self.y * self.y).sqrt()
    }
}
#[allow(dead_code)]
impl Line {
    fn new(s: Point, t: Point) -> Line {
        Line { s: s, t: t }
    }
}
#[allow(dead_code)]
fn cross(lhs: &Point, rhs: &Point) -> f64 {
    lhs.x * rhs.y - lhs.y * rhs.x
}
#[allow(dead_code)]
fn dot(lhs: &Point, rhs: &Point) -> f64 {
    lhs.x * rhs.x + lhs.y * rhs.y
}
#[allow(dead_code)]
fn ccw(a: &Point, b: &Point, c: &Point) -> i32 {
    let b = *b - *a;
    let c = *c - *a;
    if cross(&b, &c) > 0f64 {
        return 1; // ccw
    }
    if cross(&b, &c) < 0f64 {
        return -1; // cw
    }
    if dot(&b, &c) < 0f64 {
        return 2; // online & reverse direction
    }
    if b.norm() < c.norm() {
        return -2; // online & same direction & |b| < |c|
    }
    0 // online & same direction & b| > |c|
}

#[test]
fn test_add_sub() {
    let p1 = Point::new(1f64, 2f64);
    let p2 = Point::new(10f64, 20f64);
    let p3 = p1 + p2;
    assert!(p3.x == 11f64);
    assert!(p3.y == 22f64);
    let p4 = p3 - p2;
    assert!(p4 == p1);
    let mut p5 = p1;
    p5 += p2;
    assert!(p3 == p5);
    let mut p6 = p3;
    p6 -= p2;
    assert!(p6 == p1);
}
#[test]
fn test_mul_div() {
    let p1 = Point::new(1f64, 2f64);
    let v = 10f64;
    let p2 = v * p1;
    assert!(p2.x == 10f64);
    assert!(p2.y == 20f64);
    let p3 = p2 / 10f64;
    assert!(p3.x == 1f64);
    assert!(p3.y == 2f64);
}
#[test]
fn test_cross_dot() {
    let p1 = Point::new(1f64, 2f64);
    let p2 = Point::new(3f64, -7f64);
    assert!(dot(&p1, &p2) == -11f64);
    assert!(cross(&p1, &p2) == -13f64);
}
#[test]
fn test_ccw() {
    let p1 = Point::new(1f64, 1f64);
    let p2 = Point::new(2f64, 1f64);
    let p3 = Point::new(1f64, 2f64);
    assert!(ccw(&p1, &p2, &p3) == 1);
    assert!(ccw(&p1, &p3, &p2) == -1);
    assert!(ccw(&p1, &p2, &p2) == 0);
}
