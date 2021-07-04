#![allow(unused, non_snake_case, dead_code, non_upper_case_globals)]
use {
    proconio::{marker::*, *},
    std::{cmp::*, collections::*, mem::*},
};

#[fastout]
fn main() {
    input! {//
        L:Chars,
    }
    let n = L.len();
    let mut dp = vec![vec![0; 2]; n + 1];

    dp[0][0] = 0;
    dp[0][1] = 1;
    for i in 0..n {
        dp[i][0] %= MOD;
        dp[i][1] %= MOD;
        if L[i] == '0' {
            dp[i + 1][0] += dp[i][0] * 3;
            dp[i + 1][1] += dp[i][1];
        } else {
            dp[i + 1][0] += dp[i][0] * 3;
            dp[i + 1][1] += dp[i][1] * 2;
            dp[i + 1][0] += dp[i][1];
        }
    }
    println!("{}", (dp[n][0] + dp[n][1]) % MOD);
}

const MOD: i64 = 1000000007;
