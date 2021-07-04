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
        h:usize,w:usize,
        a:[[usize;w];h],
        b:[[usize;w];h],
    }
    let geta = 80 * 80 + 2;
    let c = a
        .iter()
        .zip(b.iter())
        .map(|(a, b)| {
            a.iter()
                .zip(b.iter())
                .map(|(a, b)| if a > b { a - b } else { b - a })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    // dbg!(&c);
    let mut dp = vec![vec![vec![false; geta * 2 + 1]; w + 1]; h + 1];
    dp[0][0][geta + c[0][0]] = true;
    dp[0][0][geta - c[0][0]] = true;
    // add or sub で 0 に近い値にする
    for i in 0..h {
        for j in 0..w {
            for k in 80..=geta * 2 - 80 {
                if i + 1 < h {
                    dp[i + 1][j][k + c[i + 1][j]] |= dp[i][j][k];
                    dp[i + 1][j][k - c[i + 1][j]] |= dp[i][j][k];
                }
                if j + 1 < w {
                    dp[i][j + 1][k + c[i][j + 1]] |= dp[i][j][k];
                    dp[i][j + 1][k - c[i][j + 1]] |= dp[i][j][k];
                }
            }
        }
    }
    let mut ans = 1111;
    for i in 0..geta * 2 {
        if dp[h - 1][w - 1][i] {
            ans = min(ans, if i >= geta { i - geta } else { geta - i });
        }
    }
    println!("{}", ans);
}
