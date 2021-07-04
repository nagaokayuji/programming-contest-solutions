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
        d:[i64;n],
    }
    let mut ans = 0i64;
    for i in 0..n {
        for j in i + 1..n {
            ans += d[i] * d[j];
        }
    }
    println!("{}", ans);
}
