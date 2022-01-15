#![allow(unused, non_snake_case, dead_code, non_upper_case_globals)]
use {
    proconio::{marker::*, *},
    std::{cmp::*, collections::*, mem::*},
};
macro_rules! chmax {
    ($base:expr, $cmp:expr) => {
        if $base < $cmp {
            $base = $cmp;
        }
    };
}

fn main() {
    input! {//
        r:usize,
        c:usize,
        k:usize,
        rcv:[(Usize1,Usize1,usize);k],
    }
    let mut rcvs = vec![vec![0; c]; r];
    for &(r, c, v) in rcv.iter() {
        rcvs[r][c] = v;
    }
    let mut dp = vec![vec![vec![0; 4]; c]; r]; // dp[r][c][k]
    let mut ans = 0;
    for rr in 0..r {
        for cc in 0..c {
            for i in 0..=3 {
                if rr + 1 < r {
                    chmax!(dp[rr + 1][cc][0], dp[rr][cc][i]);
                    if i < 3 {
                        chmax!(dp[rr + 1][cc][0], dp[rr][cc][i] + rcvs[rr][cc]);
                    }
                }
                if cc + 1 < c {
                    chmax!(dp[rr][cc + 1][i], dp[rr][cc][i]);
                    if i < 3 {
                        chmax!(dp[rr][cc + 1][i + 1], dp[rr][cc][i] + rcvs[rr][cc]);
                    }
                }
            }
        }
    }
    let mut ans = dp[r - 1][c - 1][3];
    for i in 0..3 {
        chmax!(ans, dp[r - 1][c - 1][i] + rcvs[r - 1][c - 1]);
    }
    println!("{}", ans);
}
