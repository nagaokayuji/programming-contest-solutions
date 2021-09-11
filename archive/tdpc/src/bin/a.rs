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
        p:[usize;n],
    }
    let mut dp = vec![false; n * 102 + 333];
    dp[0] = true;
    for &x in p.iter() {
        for i in (0..=n * 100).rev() {
            dp[i + x] |= dp[i];
        }
    }
    println!("{}", dp.iter().filter(|&&x| x).count());
}
