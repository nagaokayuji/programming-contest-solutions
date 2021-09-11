#![allow(unused, non_snake_case, dead_code, non_upper_case_globals)]
use {
    proconio::{marker::*, *},
    std::{cmp::*, collections::*, mem::*},
};

#[fastout]
fn main() {
    input! {
        n:usize,
        b:[[usize;n];n],
    }
    let mut a = vec![vec![false; n + 1]; n + 1];
    let mut t = vec![0; n];
    for i in 0..n {
        for j in 0..n {
            t[i] |= if b[i][j] == 1 { 1 << j } else { 0 };
            a[i][j] = if b[i][j] == 1 { true } else { false };
        }
    }
    // dbg!(&a);
    let mut dp = vec![0i64; 1 << n + 2];
    dp[0] = 1i64;
    for s in 1..(1usize << n) {
        let i = s.count_ones() as usize;
        for j in 0..n {
            if (((s >> j) & 1) == 1) && a[i - 1][j] {
                dp[s] = (dp[s] + dp[s ^ (1usize << j)]) % MOD;
            }
        }
    }
    println!("{}", dp[(1 << n) - 1]);
}

const MOD: i64 = 1000000007;
