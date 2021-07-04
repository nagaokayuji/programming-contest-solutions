#![allow(unused, non_snake_case, dead_code, non_upper_case_globals)]
use {
    proconio::{marker::*, *},
    std::{cmp::*, collections::*, mem::*},
};

#[fastout]
fn main() {
    input! {//
        n:usize,
        a:[i64;n],
    }
    let mut ans = 0;
    for i in 0..n {
        if i % 2 == 0 && a[i] % 2 == 1 {
            ans += 1;
        }
    }
    println!("{}", ans);
}
