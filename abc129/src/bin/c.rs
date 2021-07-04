#![allow(unused, non_snake_case, dead_code, non_upper_case_globals)]
use {
    proconio::{marker::*, *},
    std::{cmp::*, collections::*, mem::*},
};

#[fastout]
fn main() {
    input! {//
        n:usize,m:usize,
        a:[usize;m]
    }
    const MOD: i64 = 1000000007;
    let mut dp = vec![0; n + 4];
    let mut ok = vec![true; n + 4];
    for &x in a.iter() {
        ok[x] = false;
    }
    dp[0] = 1;

    for i in 0..n {
        if ok[i + 1] {
            dp[i + 1] += dp[i];
            dp[i + 1] %= MOD;
        }
        if ok[i + 2] {
            dp[i + 2] += dp[i];
            dp[i + 2] %= MOD;
        }
    }
    println!("{}", dp[n]);
}
