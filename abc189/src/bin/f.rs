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
        n:usize,m:usize,k:usize,
        a:[usize;k],
    }

    let mut dp = vec![0;n+1];
    dp[0] = 0;
    dp[1] = 1;

    

}
