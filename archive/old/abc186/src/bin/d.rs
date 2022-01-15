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
        mut a:[i64;n],
    }
    a.sort();

    let mut ans = 0i64;
    for i in 0..n - 1 {
        // i, i+1
        ans += (a[i + 1] - a[i]) * (i + 1) as i64 * (n - i - 1) as i64;
    }
    println!("{}", ans);
}
