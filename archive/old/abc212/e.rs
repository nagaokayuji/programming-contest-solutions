#![allow(unused, non_snake_case, dead_code, non_upper_case_globals)]
use {
    proconio::{marker::*, *},
    std::*,
    std::{cmp::*, collections::*, convert::*, iter::*, marker::*, mem::*, ops::*},
};
/*
    ■
*/
const MOD: i64 = 998244353;
#[fastout]
fn main() {
    input! {//
        n:usize,m:usize,k:usize,
        uv:[(Usize1,Usize1);m],
    }

    let mut g = vec![vec![]; n];
    for &(u, v) in uv.iter() {
        g[u].push(v);
        g[v].push(u);
    }

    // dp[i] := i でおわるれつ
    let mut dp = vec![0i64; n];
    dp[0] = 1i64;

    for _ in 0..k {
        let mut newdp = vec![0i64; n];
        let s = dp.iter().fold(0i64, |x, sum| (sum + x) % MOD);
        for j in 0..n {
            newdp[j] -= dp[j];
            for &ng in g[j].iter() {
                newdp[j] -= dp[ng];
            }
            newdp[j] += s;
            newdp[j] = (newdp[j] + MOD) % MOD;
        }
        dp = newdp;
    }
    println!("{}", (dp[0] + MOD) % MOD);
}
