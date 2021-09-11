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
    let mut iter = 0;
    let mut num = vec![0; MX];
    let mut dp = vec![vec![0; MX]; 111];
    let mut dp_sum = vec![0; MX];
    let mut i = 1;
    while i <= n {
        let j = n / i;
        if i <= j {
            num[iter] = 1;
            i += 1;
        } else {
            num[iter] = n / j - i + 1;
            i = n / j + 1;
        }
        iter += 1;
    }
    dp[0][0] = 1;
    for kk in 0..k {
        dp_sum = vec![0; MX];
        for j in 0..=iter {
            dp_sum[j + 1] = (dp_sum[j] + dp[kk][j]) % MOD;
        }
        for j in 0..iter {
            dp[kk + 1][j] = dp_sum[iter - j] * num[j] % MOD;
        }
    }
    let mut ans = 0;
    for i in 0..=iter {
        ans += dp[k][i];
        ans %= MOD;
    }
    println!("{}", ans);
}
const MOD: usize = 1000000007;
const MX: usize = 101101;
