#[allow(dead_code)]
struct Eratosthnes {
    n: usize,  // incluesive
    bprime: Vec<bool>,  // self.bprime[5] = true
    primes: Vec<usize>, // self.primes[2] = 5
}
#[allow(dead_code)]
impl Eratosthnes {
    fn new(n: usize) -> Eratosthnes {
        let mut bprime = vec![true; n + 1];
        let mut primes = vec![];
        primes.reserve(n / 10 + 1000);
        bprime[0] = false;
        bprime[1] = false;
        for i in 2..n+1 {
            if !bprime[i] { continue; }
            primes.push(i);
            if i * i > n { continue; }
            let mut j = i * i;
            while j <= n {
                bprime[j] = false;
                j += i;
            }
        }
        Eratosthnes {
            n: n,
            bprime: bprime,
            primes: primes,
        }
    }
}