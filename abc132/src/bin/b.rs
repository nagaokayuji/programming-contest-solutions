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
    for i in 1..n - 1 {
        let mut left = min(a[i - 1], a[i + 1]);
        let mut right = max(a[i - 1], a[i + 1]);
        if (left..right).contains(&a[i]) {
            ans += 1;
        }
    }
    println!("{}", ans);
}
