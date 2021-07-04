#![allow(unused, non_snake_case, dead_code, non_upper_case_globals)]
use {
    proconio::{marker::*, *},
    std::{cmp::*, collections::*, mem::*},
};

#[fastout]
fn main() {
    input! {//
        n:usize,
        mut a:[i64;n+1],
        mut b:[i64;n],
    }
    let mut ans = 0;
    for i in 0..n {
        ans += min(b[i], a[i]);
        if b[i] > a[i] {
            b[i] -= min(b[i], a[i]);
            ans += min(a[i + 1], b[i]);
            a[i + 1] -= min(a[i + 1], b[i]);
        }
    }
    println!("{}", ans);
}
