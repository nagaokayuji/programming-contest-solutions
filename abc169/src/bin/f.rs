#![allow(unused, non_snake_case, dead_code, non_upper_case_globals)]
use proconio::{fastout, input};
use std::cmp::*;
use std::collections::*;
use std::mem::*;

#[fastout]
fn main() {
  input! {
    n:usize,
    s:usize,
    a:[usize;n],
  }
  let mut dp = vec![vec![0; s + 1]; n + 1];
  dp[0][0] = 1;
  for i in 0..n {
    for j in 0..=s {
      dp[i + 1][j] = (dp[i + 1][j] + 2 * dp[i][j]) % MOD;
      if j + a[i] <= s {
        dp[i + 1][j + a[i]] = (dp[i + 1][j + a[i]] + dp[i][j]) % MOD;
      }
    }
  }
  println!("{}", (dp[n][s] + MOD) % MOD);
}
const MOD: usize = 998244353;
