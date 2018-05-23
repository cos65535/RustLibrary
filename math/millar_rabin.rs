fn large_mul_mod(mut lhs: i64, mut rhs: i64, modulo: i64) -> i64 {
    let mut ret = 0i64;
    while rhs > 0 {
        if (rhs & 1) == 1 {
            ret = (ret + lhs) % modulo;
        }
        rhs >>= 1;
        lhs = (lhs + lhs) % modulo;
    }
    ret
}

fn pow_mod(mut base: i64, mut power: i64, modulo: i64) -> i64 {
    let mut ans = 1i64;
    if modulo < (1 << 30) {
        while power > 0 {
            if (power & 1) == 1 {
                ans = ans * base % modulo;
            }
            power >>= 1;
            base = base * base % modulo;
        }
    } else {
        while power > 0 {
            if (power & 1) == 1 {
                ans = large_mul_mod(ans, base, modulo);
            }
            power >>= 1;
            base = large_mul_mod(base, base, modulo);
        }
    }
    ans
}

fn suspect(t: i64, mut s: i64, d: i64, n: i64) -> bool {
    let mut x = pow_mod(t, d, n);
    if x == 1 {
        return true;
    }
    while s > 0 {
        s -= 1;
        if x == n - 1 {
            return true;
        }
        if n < (1 << 30) {
            x = x * x % n;
        } else {
            x = large_mul_mod(x, x, n);
        }
    }
    false
}

#[allow(dead_code)]
fn is_prime(n: i64) -> bool {
    if n <= 1 || (n > 2 && n % 2 == 0) {
        return false;
    }
    let mut d = n - 1;
    let mut s = 0;
    while (d & 1) == 0 {
        s += 1;
        d >>= 1;
    }
    // let test = [2, 7, 61, 1i64 << 60]; // is for n < 2^32
    let test = [2, 3, 5, 7, 11, 13, 17, 19, 23, 1i64 << 60]; // is for n < 10^16 (at least)
    for &t in test.iter() {
        if t >= n {
            break;
        }
        if !suspect(t, s, d, n) {
            return false;
        }
    }
    true
}

#[test]
fn test_is_prime() {
    assert!(is_prime(2));
    assert!(is_prime(3));
    assert!(is_prime(5));
    assert!(is_prime(7));
    assert!(is_prime(11));
    assert!(is_prime(103));
    assert!(is_prime(1000000007));
    assert!(is_prime(1000000009));
    assert!(is_prime(3478329271));
    assert!(is_prime(777177177781));
    assert!(is_prime(67280421310721));

    assert!(!is_prime(-1));
    assert!(!is_prime(0));
    assert!(!is_prime(1));
    assert!(!is_prime(4));
    assert!(!is_prime(9));
    assert!(!is_prime(77));
    assert!(!is_prime(1000000001));
    assert!(!is_prime(4002002002));
    assert!(!is_prime(7478329271));
    assert!(!is_prime(1357436473381));
}
