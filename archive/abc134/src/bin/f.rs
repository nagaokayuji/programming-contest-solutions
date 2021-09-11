#![allow(unused, non_snake_case, dead_code, non_upper_case_globals)]
use {
    proconio::{marker::*, *},
    std::{cmp::*, collections::*, mem::*},
};

#[fastout]
fn main() {
    input! {//
        n:usize,k:usize,
    }
    let mut dp = vec![vec![vec![0i128; 3333]; 55]; 55];
    dp[0][0][0] = 1;
    for i in 0..n {
        for rest in 0..n {
            for k in 0..=k {
                let ad = dp[i][rest][k];
                dp[i + 1][rest + 1][k + 2 * rest + 2] += ad;
                dp[i + 1][rest + 1][k + 2 * rest + 2] %= MOD;
                dp[i + 1][rest][k + 2 * rest] += ad;
                dp[i + 1][rest][k + 2 * rest] %= MOD;
                if rest > 0 {
                    dp[i + 1][rest - 1][k + 2 * rest - 2] += ad * (rest * rest) as i128;
                    dp[i + 1][rest - 1][k + 2 * rest - 2] %= MOD;
                    dp[i + 1][rest][k + 2 * rest] += ad * 2 * rest as i128;
                    dp[i + 1][rest][k + 2 * rest] %= MOD;
                }
                dp[i][rest][k] %= MOD;
            }
        }
    }
    println!("{}", dp[n][0][k] % MOD);
}
const MOD: i128 = 1000000007;
