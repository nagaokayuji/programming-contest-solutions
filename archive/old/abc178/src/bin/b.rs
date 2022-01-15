#![allow(unused, non_snake_case, dead_code, non_upper_case_globals)]
use {
    proconio::{marker::*, *},
    std::{cmp::*, collections::*, mem::*},
};

#[fastout]
fn main() {
    input! {//
        mut a:[i64;4],
    }
    let mut ans = -INF;

    for i in 0..2 {
        for j in 2..4 {
            ans = max(ans, a[i] * a[j])
        }
    }
    println!("{}", ans);
}
const INF: i64 = 1 << 60;
