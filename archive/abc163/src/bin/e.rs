#![allow(unused, non_snake_case, dead_code, non_upper_case_globals)]
use {
    proconio::{marker::*, *},
    std::{cmp::*, collections::*, mem::*},
};

#[fastout]
fn main() {
    input! {
    n:usize,
    a:[i64;n],
     }
    let mut ai = vec![(0, 0); n];
    for (i, &x) in a.iter().enumerate() {
        ai[i] = (x, i);
    }
    ai.sort_by_key(|x| -x.0);
    let mut dp = vec![vec![-INF; n + 1]; n + 1];
    dp[0][0] = 0;
    for i in 0..n {
        let k = ai[i].0;
        let r = ai[i].1;
        for x in 0..=i {
            let y = i - x;
            dp[x + 1][y] = max(dp[x + 1][y], dp[x][y] + k * (x as i64 - r as i64).abs());
            dp[x][y + 1] = max(
                dp[x][y + 1],
                dp[x][y] + k * ((n - 1 - y) as i64 - r as i64).abs(),
            );
        }
    }
    // dbg!(&dp);
    let mut ans = 0;
    for i in 0..=n {
        ans = max(ans, dp[i][n - i]);
    }
    println!("{}", ans);
}
const INF: i64 = 1 << 60;
