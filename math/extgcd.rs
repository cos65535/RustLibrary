#[allow(dead_code)]
fn gcd(a: i64, b: i64) -> i64 {
    if b == 0 { return a; }
    gcd(b, a % b)
}

#[allow(dead_code)]
fn lcm(a: i64, b: i64) -> i64 {
    a / gcd(a, b) * b
}

#[allow(dead_code)]
// a x + b y = gcd(a, b)
fn extgcd(a: i64, b: i64, x: &mut i64, y: &mut i64) -> i64 {
    let mut g = a;
    *x = 1;
    *y = 0;
    if b != 0 {
        g = extgcd(b, a % b, y, x);
        *y -= (a / b) * *x;
    }
    g
}

#[allow(dead_code)]
fn pow_mod(base: i64, power: i64, modulo: i64) -> i64 {
    let mut base = base;
    let mut power = power;
    let mut ans = 1;
    while power > 0 {
        if power & 1 == 1 { ans = ans * base % modulo; }
        power >>= 1;
        base = base * base % modulo;
    }
    ans
}

#[allow(dead_code)]
fn inv_mod(a: i64, modulo: i64) -> i64 {
    pow_mod(a, modulo - 2, modulo)
}