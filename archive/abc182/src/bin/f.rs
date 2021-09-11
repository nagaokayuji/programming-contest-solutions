#![allow(unused, non_snake_case, dead_code, non_upper_case_globals)]
use {
    proconio::{marker::*, *},
    std::*,
    std::{cmp::*, collections::*, convert::*, iter::*, marker::*, mem::*, ops::*},
};
fn main() {
    input! {//
        n:usize,X:i64,
        mut a:[i64;n],
    }
    a.push(INF);
    let mut x = vec![0; n]; // 桁の値になる
    let mut X = X;
    for i in (0..n).rev() {
        x[i] = X / a[i];
        X %= a[i];
    }
    dbg!(&x);
    let mut dp = vec![vec![0i64; 2]; n + 1];
    dp[0][0] = 1;
    for i in 0..n {
        let t = a[i + 1] / a[i];
        for j in 0..2 {
            for nj in 0..2 {
                // x+z=y
                // j: 下からのきゃりー
                // nj: 上へのきゃりー
                // x[i] + z + j - t * nj = y

                // y=0
                {
                    let z = t * nj - x[i] - j;
                    if (0..t).contains(&z) {
                        dp[i + 1][nj as usize] += dp[i][j as usize];
                    }
                }

                // z=0
                {
                    let y = x[i] + j - t * nj;
                    if (1..t).contains(&y) {
                        dp[i + 1][nj as usize] += dp[i][j as usize];
                    }
                }
            }
        }
    }
    dbg!(&dp);
    println!("{}", dp[n][0]);
}

const INF: i64 = 1 << 60;
const MX: usize = 1010101;
