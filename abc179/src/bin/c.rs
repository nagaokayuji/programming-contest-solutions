#![allow(unused, non_snake_case, dead_code, non_upper_case_globals)]
use {
    proconio::{marker::*, *},
    std::*,
    std::{cmp::*, collections::*, convert::*, iter::*, marker::*, mem::*, ops::*},
};
fn main() {
    input! {//
        n:usize,
    }
    let mut ans = 0i64;
    for a in 1..=n {
        for b in 1..=((n + a - a) / a) {
            if a * b < n {
                ans += 1;
            }
        }
    }
    println!("{}", ans);
}
