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
        A:usize,B:usize,
        mut a:[i64;A],
        mut b:[i64;B],
    }
    let mut dp = vec![vec![0i64; B + 1]; A + 1];
    dp[A][B] = 0; // last
    for i in (0..=A).rev() {
        for j in (0..=B).rev() {
            if i == A && j == B {
                continue;
            }

            match (i + j) % 2 {
                // 先手
                0 => {
                    dp[i][j] = max(
                        if i < A { a[i] + dp[i + 1][j] } else { -INF },
                        if j < B { b[j] + dp[i][j + 1] } else { -INF },
                    );
                }
                // 後手
                _ => {
                    dp[i][j] = min(
                        if i < A { dp[i + 1][j] } else { INF },
                        if j < B { dp[i][j + 1] } else { INF },
                    );
                }
            }
        }
    }
    println!("{}", dp[0][0]);
}

const INF: i64 = 1 << 60;
