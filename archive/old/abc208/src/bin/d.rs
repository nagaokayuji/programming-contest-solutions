#![allow(unused, non_snake_case, dead_code, non_upper_case_globals)]
use {
    proconio::{marker::*, *},
    std::*,
    std::{cmp::*, collections::*, convert::*, iter::*, marker::*, mem::*, ops::*},
};
/*
    ■
*/
fn main() {
    input! {//
        n:usize,m:usize,
        mut abc:[(Usize1,Usize1,i64);m],
    }
    let mut ans = 0i64;
    let mut dp = vec![vec![INF; n]; n];
    for i in 0..n {
        dp[i][i] = 0;
    }
    for &(a, b, c) in abc.iter() {
        dp[a][b] = c;
    }
    for k in 0..n {
        let mut dp2 = dp.clone();
        for i in 0..n {
            for j in 0..n {
                dp2[i][j] = dp2[i][j].min(dp[i][k] + dp[k][j]);
                if dp2[i][j] < INF {
                    ans += dp2[i][j];
                }
            }
        }
        dp = dp2;
    }
    println!("{}", ans);
}

const INF: i64 = 1 << 59;
