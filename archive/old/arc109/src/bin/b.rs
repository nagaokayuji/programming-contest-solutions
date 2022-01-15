#![allow(unused, non_snake_case, dead_code, non_upper_case_globals)]
use {
    proconio::{marker::*, *},
    std::*,
    std::{cmp::*, collections::*, convert::*, iter::*, marker::*, mem::*, ops::*},
};
/*
    ■
*/
#[fastout]
fn main() {
    input! {//
        n:i64,
    }
    let mut ok = 0 as i64;
    let mut ng = 1e10 as i64;
    while ng - ok > 1 {
        let mid = (ok + ng) / 2;
        let isok = |x: i64| -> bool { x.saturating_mul(x + 1) <= 2 * (n + 1) };
        if isok(mid) {
            ok = mid;
        } else {
            ng = mid;
        }
    }
    println!("{}", n - ok + 1);
}

const INF: i64 = 1 << 60;
