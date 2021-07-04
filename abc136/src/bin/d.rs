#![allow(unused, non_snake_case, dead_code, non_upper_case_globals)]
use {
    proconio::{marker::*, *},
    std::{cmp::*, collections::*, mem::*},
};

#[fastout]
fn main() {
    input! {//
        s:Chars,
    }
    let n = s.len();
    let mut dp = vec![vec![0; n]; 102];

    for i in 0..n {
        if s[i] == 'L' {
            dp[0][i] = i - 1;
        } else {
            dp[0][i] = i + 1;
        }
    }

    for i in 0..100 {
        for j in 0..n {
            let p = dp[i][j];
            dp[i + 1][j] = dp[i][p];
        }
    }
    let mut ret = vec![0; n];
    for &x in dp[80].iter() {
        ret[x] += 1;
    }
    for &x in ret.iter() {
        print!("{} ", x);
    }

    println!("\n");
}
