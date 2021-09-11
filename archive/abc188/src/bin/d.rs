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
        n:usize,C:i64,
        mut abc:[(i64,i64,i64);n],
    }
    let mut event = vec![];
    for &(a, b, c) in abc.iter() {
        event.push((a, c));
        event.push((b + 1, -c));
    }
    event.sort();
    let mut ans = 0i64;
    let mut now = 0i64;
    let mut t = 0i64;
    for &(x, y) in event.iter() {
        if x != t {
            ans += now.min(C) * (x - t);
            t = x;
        }
        now += y;
    }
    println!("{}", ans);
}

const INF: i64 = 1 << 60;
