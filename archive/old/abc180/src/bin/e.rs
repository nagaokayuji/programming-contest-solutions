#![allow(unused, non_snake_case, dead_code, non_upper_case_globals)]
use {
    proconio::{marker::*, *},
    std::*,
    std::{cmp::*, collections::*, convert::*, iter::*, marker::*, mem::*, ops::*},
};
fn main() {
    input! {//
        n:usize,
       mut xyz:[(i64,i64,i64);n],
    }

    let mut dp = vec![vec![INF; n]; 1usize << n];
    let d = |i: usize, j: usize| -> i64 {
        (xyz[i].0 - xyz[j].0).abs() + (xyz[i].1 - xyz[j].1).abs() + max(0, xyz[j].2 - xyz[i].2)
    };

    dp[(1 << n) - 1][0] = 0;
    for s in (0..(1 << n) - 1).rev() {
        for v in 0..n {
            for u in 0..n {
                if !(s >> u & 1 == 1) {
                    dp[s][v] = min(dp[s][v], dp[s | 1 << u][u] + d(v, u));
                }
            }
        }
    }
    println!("{}", dp[0][0]);
}
const INF: i64 = 1 << 60;
