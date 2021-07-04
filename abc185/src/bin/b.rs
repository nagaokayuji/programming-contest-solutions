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
        n:usize,m:usize,t:i64,
        mut ab:[(i64,i64);m],
    }
    let mut now = n as i64;
    let mut prv = 0;
    ab.push((t, t));
    for &(a, b) in ab.iter() {
        now -= a - prv;
        if now <= 0 {
            yn(false);
            return;
        }
        now += b - a;
        now = min(now, n as i64);
        prv = b;
    }
    yn(true);
}

fn yn(ans: bool) {
    println!("{}", if ans { "Yes" } else { "No" });
}
