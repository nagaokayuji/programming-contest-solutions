use proconio::*;
use std::collections::*;

fn main() {
    input! {
        n: usize,
        ab: [(i64, i64); n],
    }

    let mut dp = BTreeSet::new();

    dp.insert(ab[0]);
    for (i, &(a, b)) in ab.iter().enumerate() {
        let mut nxt = BTreeSet::new();
        for &(x, y) in dp.iter() {
            nxt.insert((gcd(x, a), gcd(y, b)));
            nxt.insert((gcd(x, b), gcd(y, a)));
        }
        dp = nxt;
    }
    let ans = dp.iter().map(|&(r, b)| lcm(r, b)).max().unwrap();
    println!("{}", ans);
}

pub fn gcd(a: i64, b: i64) -> i64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}
pub fn lcm(a: i64, b: i64) -> i64 {
    a / gcd(a, b) * b
}
