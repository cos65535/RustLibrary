use std::io::*;
use std::str::FromStr;

#[allow(dead_code)]
fn read<T: FromStr>() -> T {
    let stdin = stdin();
    let stdin = stdin.lock();
    let token: String = stdin
        .bytes()
        .map(|c| c.expect("failed to read char") as char)
        .skip_while(|c| c.is_whitespace())
        .take_while(|c| !c.is_whitespace())
        .collect();
    token.parse().ok().expect("failed to parse token")
}

struct Solver {
    // add global variable
}

impl Solver {
    fn new() -> Solver {
        Solver { }
    }
    fn solve(&mut self) {
    }
}

fn main() {
    let mut solver = Solver::new();
    solver.solve();
}