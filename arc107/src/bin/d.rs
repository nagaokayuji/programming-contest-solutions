#![allow(unused, non_snake_case, dead_code, non_upper_case_globals)]
use {
    proconio::{marker::*, *},
    std::*,
    std::{cmp::*, collections::*, convert::*, iter::*, marker::*, mem::*, ops::*},
};
fn main() {
    input! {//
        n:usize,k:usize,
    }
    let mut ans = 0i64;
    let mut mem = vec![vec![0i64; n as usize + 1]; 13];

    let mut g = G {
        mem: vec![vec![INF; n + 1]; n + 1],
    };
    println!("{}", g.rec(n, k));
    // dbg!(&g.mem);
}

struct G {
    mem: Vec<Vec<i64>>,
}
impl G {
    fn rec(&mut self, n: usize, target: usize) -> i64 {
        if self.mem[n][target] != INF {
            return self.mem[n][target];
        }
        if n < target {
            self.mem[n][target] = 0;
            return 0;
        }
        if n == 0 {
            return 0;
        }
        if target == 0 {
            return 0;
        }

        if n == target {
            self.mem[n][target] = 1;
            return 1;
        }
        let mut ret = 0i64;
        // for i in 0..=target {
        //     let nxt = target - i;
        //     let nxn = n - i;
        //     if nxt * 2 > nxn {
        //         continue;
        //     }
        //     ret += self.rec(nxn, nxt * 2);
        //     ret %= MOD;
        // }
        ret = self.rec(n - 1, target - 1)
            + if target * 2 > n {
                0
            } else {
                self.rec(n, target * 2)
            } % MOD;
        self.mem[n][target] = ret % MOD;

        self.mem[n][target]
    }
}
const INF: i64 = 1 << 60;
const MOD: i64 = 998244353;
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
