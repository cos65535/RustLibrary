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

const MOD: i64 = 1e+9 as i64 + 7;
const FACT_MAX: usize = 100000;
static mut FACTORIAL: [i64; FACT_MAX + 1] = [0; FACT_MAX + 1];
static mut RFACTORIAL: [i64; FACT_MAX + 1] = [0; FACT_MAX + 1];
fn init_fact() {
    unsafe {
        if FACTORIAL[0] == 0 {
            FACTORIAL[0] = 1;
            for i in 1..FACT_MAX+1 {
                FACTORIAL[i] = (FACTORIAL[i - 1] * i as i64) % MOD;
            }
            RFACTORIAL[FACT_MAX] = inv_mod(FACTORIAL[FACT_MAX], MOD);
            for i in (0..FACT_MAX).rev() {
                RFACTORIAL[i] = RFACTORIAL[i + 1] * (i as i64 + 1) % MOD;
            }
        }
    }
}
#[allow(dead_code)]
fn fact(n: usize) -> i64 {
    init_fact();
    unsafe {
        FACTORIAL[n]
    }
}
#[allow(dead_code, unused_comparisons)]
fn combi(n: usize, m: usize) -> i64 {
    if n < m || m < 0 { return 0; }
    init_fact();
    unsafe {
        FACTORIAL[n] * RFACTORIAL[m] % MOD * RFACTORIAL[n - m] % MOD
    }
}
