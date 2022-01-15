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
        n:usize,
        h:[i64;n],
    }
    let mut ret = 0i64;
    let mut dp = vec![0i64; n];
    for i in 1..n {
        if h[i - 1] >= h[i] {
            dp[i] = dp[i - 1] + 1;
            ret = max(ret, dp[i]);
        }
    }
    println!("{}", ret);
}
