#[allow(dead_code)]
#[derive(Eq, PartialEq)]
enum ContainState {
    OUT,
    ON,
    IN,
}
#[allow(dead_code)]
fn contain_point(poly: &Polygon, p: &Point) -> ContainState {
    let mut inner = false;
    for i in 0..poly.len() {
        let mut a = poly[i] - *p;
        let mut b = poly[(i + 1) % poly.len()] - *p;
        if a.y > b.y {
            std::mem::swap(&mut a, &mut b);
        }
        if a.y <= 0f64 && 0f64 < b.y {
            if cross(&a, &b) < 0f64 {
                inner = !inner;
            }
        }
        if cross(&a, &b) == 0f64 && dot(&a, &b) <= 0f64 {
            return ContainState::ON;
        }
    }
    if inner {
        ContainState::IN
    } else {
        ContainState::OUT
    }
}

#[test]
fn test_contain_point() {
    let p1 = Point::new(0f64, 0f64);
    let p2 = Point::new(0f64, 3f64);
    let p3 = Point::new(3f64, 0f64);
    let poly: Polygon = vec![p1, p2, p3];
    let p4 = Point::new(1f64, 1f64);
    let p5 = Point::new(1f64, 0f64);
    let p6 = Point::new(3f64, 3f64);
    assert!(contain_point(&poly, &p4) == ContainState::IN);
    assert!(contain_point(&poly, &p5) == ContainState::ON);
    assert!(contain_point(&poly, &p6) == ContainState::OUT);
}