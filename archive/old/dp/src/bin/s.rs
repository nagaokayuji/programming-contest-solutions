#![allow(unused, non_snake_case, dead_code, non_upper_case_globals)]
use {
    proconio::{marker::*, *},
    std::{cmp::*, collections::*, mem::*},
};

#[fastout]
fn main() {
    input! {
        k:Chars,
        d:usize,
    }
    //剰余を持つ？

    let n = k.len();
    let mut dp = vec![vec![vec![0i64; 2]; d]; n + 1];

    dp[0][0][1] = 1;
    for (i, &c) in k.iter().enumerate() {
        let num = charconv(c, '0');
        for amari in 0..d {
            dp[i + 1][(amari + num) % d][1] += dp[i][amari][1];
            for j in 0..num {
                dp[i + 1][(amari + j) % d][0] += dp[i][amari][1];
            }
            for j in 0..=9 {
                dp[i + 1][(amari + j) % d][0] += dp[i][amari][0];
            }
            dp[i][amari][0] %= MOD;
            dp[i][amari][1] %= MOD;
            dp[i + 1][amari][0] %= MOD;
            dp[i + 1][amari][1] %= MOD;
        }
    }
    // dbg!(&dp);
    println!("{}", (dp[n][0][0] + dp[n][0][1] - 1 + MOD) % MOD);
}
const MOD: i64 = 1000000007;
pub fn charconv(c: char, base: char) -> usize {
    return (c as u8 - base as u8) as usize;
}
