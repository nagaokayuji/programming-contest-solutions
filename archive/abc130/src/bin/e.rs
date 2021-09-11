#![allow(unused, non_snake_case, dead_code, non_upper_case_globals)]
use {
    proconio::{marker::*, *},
    std::{cmp::*, collections::*, mem::*},
};

#[fastout]
fn main() {
    input! {//
        n:usize,m:usize,
        s:[i64;n],
        t:[i64;m],
    }
    let mut dp = vec![vec![0; m + 1]; n + 1];
    dp[0][0] = 1;
    for i in 0..=n {
        for j in 0..=m {
            if i >= 1 && j >= 1 && s[i - 1] == t[j - 1] {
                dp[i][j] += dp[i - 1][j - 1];
            }
            if i >= 1 {
                dp[i][j] += dp[i - 1][j];
            }
            if j >= 1 {
                dp[i][j] += dp[i][j - 1];
            }
            if i >= 1 && j >= 1 {
                dp[i][j] -= dp[i - 1][j - 1];
            }
            dp[i][j] = (dp[i][j] + MOD) % MOD;
        }
    }
    println!("{}", dp[n][m]);
}
const MOD: i64 = 1000000007;
