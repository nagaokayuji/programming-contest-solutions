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
        b:[i64;n-1],
    }
    let mut ans = 0i64;
    for i in 0..n - 2 {
        ans += min(b[i], b[i + 1]);
    }
    ans += b[0];
    ans += b[n - 2];
    println!("{}", ans);
}
