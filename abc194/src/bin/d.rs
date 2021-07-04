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
        n:i64,
    }

    let mut ans = 0f64;
    for i in 2..=n {
        ans += (n) as f64 / (i - 1) as f64;
    }
    println!("{}", ans);
}
