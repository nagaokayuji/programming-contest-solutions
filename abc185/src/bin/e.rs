#![allow(unused, non_snake_case, dead_code, non_upper_case_globals)]
use {
    proconio::{marker::*, *},
    std::*,
    std::{cmp::*, collections::*, convert::*, iter::*, marker::*, mem::*, ops::*},
};
fn main() {
    input! {//
        n:usize,m:usize,
        a:[i64;n],
        b:[i64;m],
    }
    let mut dp = vec![vec![INF; m + 1]; n + 1];
    dp[0][0] = (n + m) as i64;

    for i in 0..n {
        for j in 0..m {
            dp[i + 1][j] = min(dp[i + 1][j], dp[i][j] - 1);
            dp[i][j + 1] = min(dp[i][j + 1], dp[i][j] - 1);
            if a[i] == b[j] {
                dp[i + 1][j + 1] = min(dp[i + 1][j + 1], dp[i][j] - 2);
            } else {
                dp[i + 1][j + 1] = min(dp[i + 1][j + 1], dp[i][j]);
            }
        }
    }
    println!("{}", dp[n][m]);
}
const INF: i64 = 1 << 60;
