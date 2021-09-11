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
        mut ab:[(i64,i64);n],
    }
    ab.sort_by_key(|&x| x.1);
    let mut now = 0i64;
    for &(a, b) in ab.iter() {
        now += a;
        if now > b {
            yn(false);
            return;
        }
    }
    yn(true);
}
fn yn(ans: bool) {
    println!("{}", if ans { "Yes" } else { "No" });
}
