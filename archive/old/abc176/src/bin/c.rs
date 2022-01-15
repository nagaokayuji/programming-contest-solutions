#![allow(unused, non_snake_case, dead_code, non_upper_case_globals)]
use {
    proconio::{marker::*, *},
    std::{cmp::*, collections::*, mem::*},
};

#[fastout]
fn main() {
    input! {//
        n:usize,
        mut a:[i64;n],
    }
    let mut mxs = -1;
    let mut ans = 0;
    for &x in a.iter() {
        if mxs > x {
            ans += mxs - x;
        }
        mxs = max(mxs, x);
    }
    println!("{}", ans);
}
