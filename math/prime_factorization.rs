// need Erathosthenes
fn prime_factorization(n: usize, era: &Eratosthnes) -> Vec<(usize, usize)> {
    assert!(1 < n && n <= era.n * era.n);
    let mut ret = vec![];
    let mut v = n;
    for &prime in era.primes.iter() {
        if v < prime * prime {
            break;
        }
        let mut cnt = 0;
        while v % prime == 0 {
            v /= prime;
            cnt += 1;
        }
        if cnt != 0 {
            ret.push((prime, cnt));
        }
    }
    if v != 1 {
        ret.push((v, 1));
    }
    return ret;
}

#[test]
fn test_prime_factorization() {
    let era = Eratosthnes::new(10);
    assert!(prime_factorization(2, &era) == vec![(2, 1)]);
    assert!(prime_factorization(4, &era) == vec![(2, 2)]);
    assert!(prime_factorization(12, &era) == vec![(2, 2), (3, 1)]);
    assert!(prime_factorization(31, &era) == vec![(31, 1)]);
}
