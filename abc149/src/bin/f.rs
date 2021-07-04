#![allow(unused, non_snake_case, dead_code, non_upper_case_globals)]
use {
    proconio::{marker::*, *},
    std::*,
    std::{cmp::*, collections::*, convert::*, iter::*, marker::*, mem::*, ops::*},
};
/*
    ■
*/
fn main() {
    input! {//
        n:usize,
        ab:[(Usize1,Usize1);n-1],
    }
    let mut g = vec![vec![]; n];
    for &(a, b) in ab.iter() {
        g[a].push(b);
        g[b].push(a);
    }
    let mut g = G { n: n, g: g, ans: 0 };
    g.rec(0, !0);
    let bmb = mod_pow(2, n as i64, MOD);
    let bmb_i = mod_inverse(bmb, MOD);

    println!("{}", g.ans * bmb_i % MOD);
}
struct G {
    n: usize,
    g: Vec<Vec<usize>>,
    ans: i64,
}
impl G {
    fn rec(&mut self, v: usize, p: usize) -> i64 {
        let mut res = 1;
        let mut ts = vec![];
        for &u in self.g[v].clone().iter() {
            if u == p {
                continue;
            }
            let t = self.rec(u, v);
            res += t;
            res %= MOD;
            ts.push(t);
        }
        if p != !0 {
            ts.push(self.n as i64 - res);
        }
        let mut now = mod_pow(2, (self.n as i64 - 1) as i64, MOD) - 1;
        now %= MOD;
        for &t in ts.iter() {
            now -= mod_pow(2, t, MOD) - 1;
        }
        now = ((now % MOD) + MOD) % MOD;
        self.ans += now;
        self.ans %= MOD;
        res
    }
}
const MX: usize = 1010101;
const MOD: i64 = 1000000007;
const INF: i64 = 1 << 60;
/// x ^ n % m
pub fn mod_pow(x: i64, n: i64, m: i64) -> i64 {
    let mut res = 1;
    let mut x = x % m;
    let mut n = n;
    while n > 0 {
        if n & 1 == 1 {
            res = (res * x) % m;
        }
        x = (x * x) % m;
        n >>= 1;
    }
    res
}

/// (gcd, x, y)
pub fn extgcd(a: i64, b: i64) -> (i64, i64, i64) {
    if b == 0 {
        (a, 1, 0)
    } else {
        let (gcd, x, y) = extgcd(b, a % b);
        (gcd, y, x - (a / b) * y)
    }
}
pub fn mod_inverse(a: i64, m: i64) -> i64 {
    let (_, x, _) = extgcd(a as i64, m as i64);
    ((m as i64 + x) as i64 % m) % m
}
