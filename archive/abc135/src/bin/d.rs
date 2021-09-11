#![allow(unused, non_snake_case, dead_code, non_upper_case_globals)]
use {
    proconio::{marker::*, *},
    std::{cmp::*, collections::*, mem::*},
};

#[fastout]
fn main() {
    input! {//
        mut s:Chars,
    }
    let n = s.len();

    //5  mod 13
    let mut dp = vec![vec![0; 13]; n + 1];

    s.reverse();
    if s[0] == '?' {
        for i in 0..=9 {
            dp[0][i] = 1;
        }
    } else {
        dp[0][charconv(s[0], '0')] = 1;
    }
    let mut keta = 10;
    for i in 1..n {
        if s[i] == '?' {
            for j in 0..=9 {
                for k in 0..13 {
                    let mm = (k + j * keta) % 13;
                    dp[i][mm] += dp[i - 1][k];
                    dp[i][mm] %= MOD;
                }
            }
        } else {
            let nm = charconv(s[i], '0');
            for k in 0..13 {
                let mm = (k + nm * keta) % 13;
                dp[i][mm] += dp[i - 1][k];
                dp[i][mm] %= MOD;
            }
        }
        keta = (keta * 10) % 13;
    }
    let mut ans = 0;
    // for i in 0..13 {
    //     ans += dp[n - 1][i];
    // }
    println!("{}", dp[n - 1][5]);
}
const MOD: usize = 1000000007;
pub fn charconv(c: char, base: char) -> usize {
    return (c as u8 - base as u8) as usize;
}
