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
        n:usize,
        mut s:[String;n],
    }
    let mut s = s.iter().map(|x| x=="AND").collect::<Vec<_>>();
    // true: and, false: or

    dbg!(true && false || true);

    // dp[i][j] := i番目まででj(t or f)の個数
    let mut dp = vec![vec![0i64;2];n+2];
    let AND = true;
    let OR = false;

    dp[0][0] = 1;
    dp[0][1] = 1;
    for i in 0..n{
        if s[i] == AND{
            dp[i+1][0] = dp[i][0]*2 + dp[i][1];
            dp[i+1][1] = dp[i][1];
        }
        else {
            dp[i+1][1] = dp[i][0] + dp[i][1]*2;
            dp[i+1][0] = dp[i][0];
        }
    }
    println!("{}",dp[n][1]);
}
