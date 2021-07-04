#![allow(unused, non_snake_case, dead_code, non_upper_case_globals)]
use {
    proconio::{marker::*, *},
    std::{cmp::*, collections::*, mem::*},
};

#[fastout]
fn main() {
    input! {//
        n:usize,
        w:[i64;n],
    }

    let mut sm = w.iter().fold(0, |sum, x| sum + x);
    let mut ans = 99999;
    let mut left = 0;
    for &x in w.iter() {
        left += x;
        ans = min(ans, (sm - left - left).abs());
    }
    println!("{}", ans);
}
