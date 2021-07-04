#![allow(unused, non_snake_case, dead_code, non_upper_case_globals)]
use {
    proconio::{marker::*, *},
    std::*,
    std::{cmp::*, collections::*, convert::*, iter::*, marker::*, mem::*, ops::*},
};
/*
    ■
*/
#[fastout]
fn main() {
    input! {//
        k:usize,
        r:[f64; 1<<k],
    }
    let mut dp = vec![vec![0f64; 1 << k]; k + 1];
    for i in 0..1 << k {
        dp[0][i] = 1f64;
    }
    for i in 1..=k {
        for j in 0..1 << k {
            let mut d = (j >> (i - 1)) as i64;
            d -= 2 * (d % 2);
            let left = (d as i64 + 1) << (i - 1);
            let left = left as usize;
            let right = (d as i64 + 2) << (i - 1);
            let right = right as usize;
            for k in left..right {
                if j == k {
                    continue;
                }
                dp[i][j] += dp[i - 1][j] * dp[i - 1][k] * win(r[j], r[k]);
            }
        }
    }
    for i in 0..1 << k {
        println!("{}", dp[k][i]);
    }
}
fn win(p: f64, q: f64) -> f64 {
    1.0 / (1.0 + (10.0f64).powf((q - p) / 400.0))
}
